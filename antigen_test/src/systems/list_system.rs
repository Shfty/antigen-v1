use std::collections::HashMap;

use crate::{
    components::local_mouse_position_component::LocalMousePositionComponent,
    components::pancurses_mouse_component::PancursesMouseComponent,
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
    entity_component_system::system_interface::SystemInterface,
    entity_component_system::ComponentStorage,
    entity_component_system::EntityComponentDirectory,
    entity_component_system::EntityID,
    entity_component_system::SystemDebugTrait,
    entity_component_system::{SystemError, SystemTrait},
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

impl<CS, CD> SystemTrait<CS, CD> for ListSystem
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
{
    fn run<'a>(&mut self, db: &'a mut SystemInterface<CS, CD>) -> Result<(), SystemError>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
    {
        let mouse_state = if let Some(entity_id) = db
            .entity_component_directory
            .get_entity_by_predicate(|entity_id| {
                db.entity_component_directory
                    .entity_has_component::<PancursesMouseComponent>(entity_id)
            }) {
            if let Ok(pancurses_mouse_component) =
                db.get_entity_component::<PancursesMouseComponent>(entity_id)
            {
                Some(pancurses_mouse_component.was_button_just_pressed(2))
            } else {
                None
            }
        } else {
            None
        };

        let list_control_entities =
            db.entity_component_directory
                .get_entities_by_predicate(|entity_id| {
                    db.entity_component_directory
                        .entity_has_component::<ListComponent>(entity_id)
                        && db
                            .entity_component_directory
                            .entity_has_component::<PositionComponent>(entity_id)
                        && db
                            .entity_component_directory
                            .entity_has_component::<SizeComponent>(entity_id)
                        && db
                            .entity_component_directory
                            .entity_has_component::<ParentEntityComponent>(entity_id)
                });

        for list_control_entity in list_control_entities {
            let (string_list_entity, list_index_entity) =
                match db.get_entity_component::<ListComponent>(list_control_entity) {
                    Ok(pancurses_list_control_component) => (
                        pancurses_list_control_component.get_string_list_entity(),
                        pancurses_list_control_component.get_list_index_entity(),
                    ),
                    Err(err) => return Err(err.into()),
                };

            if let Some(string_list_entity) = string_list_entity {
                let IVector2(width, height) =
                    match db.get_entity_component::<SizeComponent>(list_control_entity) {
                        Ok(size_component) => size_component.get_size(),
                        Err(err) => return Err(err.into()),
                    };

                // If we have a string list entity, fetch its strings
                let string_list: Vec<Vec<String>> = db
                    .get_entity_component::<StringListComponent>(string_list_entity)?
                    .get_data()
                    .iter()
                    .map(|string| {
                        let substrings: Vec<String> = string
                            .split('\n')
                            .map(|string| {
                                &string[..std::cmp::min(width, string.len() as i64) as usize]
                            })
                            .map(std::string::ToString::to_string)
                            .collect();
                        substrings
                    })
                    .collect();

                let string_count: usize = string_list.iter().map(|strings| strings.len()).sum();

                if self
                    .list_string_entities
                    .get(&list_control_entity)
                    .is_none()
                {
                    self.list_string_entities
                        .insert(list_control_entity, Vec::new());
                }

                let string_entities = match self.list_string_entities.get_mut(&list_control_entity)
                {
                    Some(string_entities) => string_entities,
                    None => {
                        return Err(format!(
                            "Failed to get list string entities for list control entity {}",
                            list_control_entity
                        )
                        .into())
                    }
                };

                while string_entities.len() < string_count {
                    let string_entity = db.create_entity(Some("List String Entity"))?;
                    db.insert_entity_component(string_entity, ControlComponent)?;
                    db.insert_entity_component(string_entity, PositionComponent::default())?;
                    db.insert_entity_component(string_entity, GlobalPositionComponent::default())?;
                    db.insert_entity_component(
                        string_entity,
                        ParentEntityComponent::new(list_control_entity),
                    )?;
                    db.insert_entity_component(string_entity, StringComponent::default())?;
                    db.insert_entity_component(
                        string_entity,
                        PancursesColorPairComponent::new(PancursesColorPair::default()),
                    )?;

                    if db
                        .entity_component_directory
                        .entity_has_component::<DebugExcludeComponent>(&list_control_entity)
                    {
                        db.insert_entity_component(string_entity, DebugExcludeComponent)?;
                    }

                    string_entities.push(string_entity);
                }

                // Create or update string components for this set of strings
                while string_entities.len() > string_count {
                    if let Some(string_entity) = string_entities.pop() {
                        db.destroy_entity(string_entity)?;
                    }
                }

                let local_mouse_position = match db
                    .get_entity_component::<LocalMousePositionComponent>(list_control_entity)
                {
                    Ok(local_mouse_position_component) => {
                        Some(local_mouse_position_component.get_local_mouse_position())
                    }
                    Err(_) => None,
                };

                let contains_mouse = match local_mouse_position {
                    Some(IVector2(mouse_x, mouse_y)) => {
                        let range_x = 0i64..width;
                        let range_y = 0i64..height as i64;
                        range_x.contains(&mouse_x) && range_y.contains(&mouse_y)
                    }
                    None => false,
                };

                let hovered_item = if contains_mouse {
                    let IVector2(_, mouse_y) = local_mouse_position.unwrap();
                    string_list
                        .iter()
                        .fold(vec![], |mut acc, next| {
                            acc.push(acc.last().unwrap_or(&0) + next.len());
                            acc
                        })
                        .into_iter()
                        .position(|i| i > mouse_y as usize)
                } else {
                    None
                };

                if let Some(list_index_entity) = list_index_entity {
                    if let Ok(int_range_component) =
                        db.get_entity_component_mut::<IntRangeComponent>(list_index_entity)
                    {
                        int_range_component.set_range(-1..(string_list.len() as i64));

                        if let Some(true) = mouse_state {
                            if contains_mouse {
                                if let Some(hovered_item) = hovered_item {
                                    int_range_component.set_index(hovered_item as i64);
                                } else {
                                    int_range_component.set_index(-1);
                                }
                            }
                        }
                    }
                }

                let focused_item = match list_index_entity {
                    Some(list_index_entity) => {
                        match db.get_entity_component_mut::<IntRangeComponent>(list_index_entity) {
                            Ok(int_range_component) => Some(int_range_component.get_index()),
                            Err(_) => None,
                        }
                    }
                    None => None,
                };

                let mut y = 0i64;
                for (string_index, strings) in string_list.iter().enumerate() {
                    let string_index = string_index as i64;

                    let mut done = false;
                    for string in strings {
                        let string_entity = string_entities[y as usize];

                        // Update each string entity's position
                        db.get_entity_component_mut::<PositionComponent>(string_entity)?
                            .set_position(IVector2(0, y));

                        // Update each string entity's text
                        db.get_entity_component_mut::<StringComponent>(string_entity)?
                            .set_data(string.clone());

                        // Update color pair based on focused item
                        let data = if Some(string_index) == focused_item {
                            PancursesColorPair::new(
                                PancursesColor::new(0, 0, 0),
                                PancursesColor::new(1000, 1000, 1000),
                            )
                        } else if Some(string_index as usize) == hovered_item {
                            PancursesColorPair::new(
                                PancursesColor::new(1000, 1000, 1000),
                                PancursesColor::new(500, 500, 500),
                            )
                        } else {
                            PancursesColorPair::default()
                        };

                        db.get_entity_component_mut::<PancursesColorPairComponent>(string_entity)?
                            .set_data(data);

                        y += 1;
                        if y >= height {
                            done = true;
                            break;
                        }
                    }

                    if done {
                        break;
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

        Ok(())
    }
}

impl SystemDebugTrait for ListSystem {
    fn get_name() -> &'static str {
        "List"
    }
}
