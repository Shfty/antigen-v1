use std::collections::HashMap;

use crate::{
    components::local_mouse_position_component::LocalMousePositionComponent,
    components::{
        control_component::ControlComponent, list_component::ListComponent,
        pancurses_color_pair_component::PancursesColorPairComponent,
    },
    pancurses_color::PancursesColor,
    pancurses_color::PancursesColorPair,
};
use antigen::{
    components::DebugExcludeComponent,
    components::GlobalPositionComponent,
    components::IntRangeComponent,
    components::ParentEntityComponent,
    components::PositionComponent,
    components::SizeComponent,
    components::StringComponent,
    components::StringListComponent,
    ecs::EntityComponentDatabaseDebug,
    ecs::EntityID,
    ecs::{EntityComponentDatabase, SystemEvent, SystemTrait},
    primitive_types::IVector2,
};

#[derive(Debug)]
pub struct ListSystem {
    // Maps list control entities -> string entities -> strings
    list_string_entities: HashMap<EntityID, Vec<EntityID>>,
}

impl ListSystem {
    pub fn new() -> Self {
        ListSystem {
            list_string_entities: HashMap::new(),
        }
    }
}

impl<T> SystemTrait<T> for ListSystem
where
    T: EntityComponentDatabase + EntityComponentDatabaseDebug,
{
    fn run(&mut self, db: &mut T) -> Result<SystemEvent, String> {
        let list_control_entities = db.get_entities_by_predicate(|entity_id| {
            db.entity_has_component::<ListComponent>(entity_id)
                && db.entity_has_component::<PositionComponent>(entity_id)
                && db.entity_has_component::<SizeComponent>(entity_id)
                && db.entity_has_component::<ParentEntityComponent>(entity_id)
        });

        for list_control_entity in list_control_entities {
            let (string_list_entity, list_index_entity) =
                match db.get_entity_component::<ListComponent>(list_control_entity) {
                    Ok(pancurses_list_control_component) => (
                        pancurses_list_control_component.string_list_entity,
                        pancurses_list_control_component.list_index_entity,
                    ),
                    Err(err) => return Err(err),
                };

            if let Some(string_list_entity) = string_list_entity {
                let IVector2(width, height) =
                    match db.get_entity_component::<SizeComponent>(list_control_entity) {
                        Ok(size_component) => size_component.data,
                        Err(err) => return Err(err),
                    };

                // If we have a string list entity, fetch its strings
                let string_list: Vec<String> =
                    match db.get_entity_component::<StringListComponent>(string_list_entity) {
                        Ok(string_list_component) => string_list_component
                            .data
                            .iter()
                            .flat_map(|string| {
                                let substrings: Vec<String> = string
                                    .split('\n')
                                    .map(|string| {
                                        &string[..std::cmp::min(width, string.len() as i64) as usize]
                                    })
                                    .map(std::string::ToString::to_string)
                                    .collect();
                                substrings
                            })
                            .take(height as usize)
                            .collect(),
                        Err(err) => return Err(err),
                    };

                if self
                    .list_string_entities
                    .get(&list_control_entity)
                    .is_none()
                {
                    self.list_string_entities
                        .insert(list_control_entity, Vec::new());
                }

                let string_entities = self
                    .list_string_entities
                    .get_mut(&list_control_entity)
                    .unwrap();

                while string_entities.len() < string_list.len() {
                    // Create string entities for this list
                    let string_entity = db.create_entity("List String Entity");
                    db.add_component_to_entity(string_entity, ControlComponent)?;
                    db.add_component_to_entity(string_entity, PositionComponent::default())?;
                    db.add_component_to_entity(string_entity, GlobalPositionComponent::default())?;
                    db.add_component_to_entity(
                        string_entity,
                        ParentEntityComponent::new(list_control_entity),
                    )?;
                    db.add_component_to_entity(string_entity, StringComponent::default())?;
                    db.add_component_to_entity(
                        string_entity,
                        PancursesColorPairComponent::new(PancursesColorPair::default()),
                    )?;
                    if db.entity_has_component::<DebugExcludeComponent>(&list_control_entity) {
                        db.add_component_to_entity(string_entity, DebugExcludeComponent)?;
                    }

                    let list_control_component =
                        db.get_entity_component::<ListComponent>(list_control_entity)?;
                    if let Some(assemblage) =
                        list_control_component.string_entity_assemblage.clone()
                    {
                        assemblage.assemble_entity(db, string_entity)?;
                    }
                    string_entities.push(string_entity);
                }

                // Create or update string components for this set of strings
                while string_entities.len() > string_list.len() {
                    if let Some(string_entity) = string_entities.pop() {
                        db.destroy_entity(string_entity)?;
                    }
                }

                let local_mouse_position = match db
                    .get_entity_component::<LocalMousePositionComponent>(list_control_entity)
                {
                    Ok(local_mouse_position_component) => Some(local_mouse_position_component.data),
                    Err(_) => None,
                };

                for (i, (entity_id, string)) in string_entities.iter().zip(&string_list).enumerate()
                {
                    // Update each string entity's position
                    match db.get_entity_component_mut::<PositionComponent>(*entity_id) {
                        Ok(position_component) => position_component.data = IVector2(0, i as i64),
                        Err(err) => return Err(err),
                    }

                    // Update each string entity's text
                    match db.get_entity_component_mut::<StringComponent>(*entity_id) {
                        Ok(string_component) => string_component.data = string.clone(),
                        Err(err) => return Err(err),
                    }

                    // Update color pair based on focused item
                    let focused_item = match list_index_entity {
                        Some(list_index_entity) => {
                            match db
                                .get_entity_component_mut::<IntRangeComponent>(list_index_entity)
                            {
                                Ok(int_range_component) => {
                                    let len = string_list.len();
                                    int_range_component.range = 0..(len as i64);
                                    if let Some(IVector2(mouse_x, mouse_y)) = local_mouse_position {
                                        let range_x = 0i64..width;
                                        if range_x.contains(&mouse_x) && mouse_y == i as i64 {
                                            int_range_component.index = i as i64;
                                        }
                                    }
                                    Some(int_range_component.index)
                                }
                                Err(_) => None,
                            }
                        }
                        None => None,
                    };

                    let pancurses_color_pair_component =
                        db.get_entity_component_mut::<PancursesColorPairComponent>(*entity_id)?;

                    if Some(i as i64) == focused_item {
                        pancurses_color_pair_component.data = PancursesColorPair::new(
                            PancursesColor::new(0, 0, 0),
                            PancursesColor::new(1000, 1000, 1000),
                        );
                    } else {
                        pancurses_color_pair_component.data = PancursesColorPair::default();
                    }
                }
            } else if self
                .list_string_entities
                .get(&list_control_entity)
                .is_some()
            {
                println!(
                    "Clearing string entities for list control entity {:?}",
                    &list_control_entity
                );
                // The list control's string list entity has been cleared, remove it from the set of string entities
                self.list_string_entities.remove(&list_control_entity);
            }
        }

        Ok(SystemEvent::None)
    }
}
