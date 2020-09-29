use std::collections::HashMap;

use crate::{
    components::{
        Control, DebugExclude, EventQueue, GlobalPosition, IntRange, List, LocalPosition,
        ParentEntity, Position, Size,
    },
    core::events::AntigenInputEvent,
    entity_component_system::{
        system_interface::SystemInterface, ComponentStorage, EntityComponentDirectory, EntityID,
        SystemDebugTrait, SystemError, SystemTrait,
    },
    primitive_types::{ColorRGB, ColorRGBF, Vector2I},
};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ListEvent {
    Hovered(i64),
    Pressed(i64),
}

#[derive(Debug)]
pub struct ListSystem {
    // Maps list control entities -> rectangle entities
    list_focus_entities: HashMap<EntityID, EntityID>,

    // Maps list control entities -> rectangle entities
    list_hover_entities: HashMap<EntityID, EntityID>,

    // Maps list control entities -> string entities
    list_string_entities: HashMap<EntityID, Vec<EntityID>>,
}

impl ListSystem {
    pub fn new() -> Self {
        ListSystem {
            list_focus_entities: HashMap::new(),
            list_hover_entities: HashMap::new(),
            list_string_entities: HashMap::new(),
        }
    }
}

impl Default for ListSystem {
    fn default() -> Self {
        ListSystem::new()
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
        let list_control_entities =
            db.entity_component_directory
                .get_entities_by_predicate(|entity_id| {
                    db.entity_component_directory
                        .entity_has_component::<List>(entity_id)
                        && db
                            .entity_component_directory
                            .entity_has_component::<Position>(entity_id)
                        && db
                            .entity_component_directory
                            .entity_has_component::<Size>(entity_id)
                        && db
                            .entity_component_directory
                            .entity_has_component::<ParentEntity>(entity_id)
                });

        for list_control_entity in list_control_entities {
            self.list_hover_entities
                .entry(list_control_entity)
                .or_insert_with(|| {
                    let list_hover_entity = db.create_entity(Some("List Hover Entity")).unwrap();
                    db.insert_entity_component(list_hover_entity, Control)
                        .unwrap();
                    db.insert_entity_component(list_hover_entity, Position::default())
                        .unwrap();
                    db.insert_entity_component(list_hover_entity, Size::default())
                        .unwrap();
                    db.insert_entity_component(list_hover_entity, GlobalPosition::default())
                        .unwrap();
                    db.insert_entity_component(list_hover_entity, ColorRGB(0.5f32, 0.5f32, 0.5f32))
                        .unwrap();
                    db.insert_entity_component(
                        list_hover_entity,
                        ParentEntity(list_control_entity),
                    )
                    .unwrap();
                    list_hover_entity
                });

            let list_hover_entity = self
                .list_hover_entities
                .get(&list_control_entity)
                .ok_or("Error getting list hover entity")?;

            self.list_focus_entities
                .entry(list_control_entity)
                .or_insert_with(|| {
                    let list_focus_entity = db.create_entity(Some("List Focus Entity")).unwrap();
                    db.insert_entity_component(list_focus_entity, Control)
                        .unwrap();
                    db.insert_entity_component(list_focus_entity, Position::default())
                        .unwrap();
                    db.insert_entity_component(list_focus_entity, Size::default())
                        .unwrap();
                    db.insert_entity_component(list_focus_entity, GlobalPosition::default())
                        .unwrap();
                    db.insert_entity_component(
                        list_focus_entity,
                        ParentEntity(list_control_entity),
                    )
                    .unwrap();
                    list_focus_entity
                });

            let list_focus_entity = self
                .list_focus_entities
                .get(&list_control_entity)
                .ok_or("Error getting list focus entity")?;

            // Fetch entity references
            let (string_list_entity, list_index_entity) =
                match db.get_entity_component::<List>(list_control_entity) {
                    Ok(pancurses_list_control_component) => (
                        pancurses_list_control_component.get_string_list_entity(),
                        pancurses_list_control_component.get_list_index_entity(),
                    ),
                    Err(err) => return Err(err.into()),
                };

            if let Some(string_list_entity) = string_list_entity {
                // The list entity is valid

                // Fetch width and height
                let Vector2I(width, height) =
                    match db.get_entity_component::<Size>(list_control_entity) {
                        Ok(size_component) => (*size_component).into(),
                        Err(err) => return Err(err.into()),
                    };

                // TODO: Scroll offset
                let scroll_offset = 0usize;

                // Fetch strings
                let string_list: Vec<Vec<String>> = (*db
                    .get_entity_component::<Vec<String>>(string_list_entity)?)
                .iter()
                .skip(scroll_offset)
                .map(|string| {
                    let substrings: Vec<String> = string
                        .split('\n')
                        .map(|string| &string[..std::cmp::min(width, string.len() as i64) as usize])
                        .map(std::string::ToString::to_string)
                        .collect();
                    substrings
                })
                .collect();

                // If this list doesn't have a vector of item entity references, create one
                if self
                    .list_string_entities
                    .get(&list_control_entity)
                    .is_none()
                {
                    self.list_string_entities
                        .insert(list_control_entity, Vec::new());
                }

                // Fetch vector of item entity references
                let string_entities = self
                    .list_string_entities
                    .get_mut(&list_control_entity)
                    .ok_or(format!(
                        "Failed to get list string entities for list control entity {}",
                        list_control_entity
                    ))?;

                // Create item entities for uninitialized lines
                let string_count: usize = string_list.iter().map(|strings| strings.len()).sum();
                while string_entities.len() < string_count {
                    let string_entity = db.create_entity(Some("List String Entity"))?;
                    db.insert_entity_component(string_entity, Control)?;
                    db.insert_entity_component(string_entity, Position::default())?;
                    db.insert_entity_component(string_entity, GlobalPosition::default())?;
                    db.insert_entity_component(string_entity, ParentEntity(list_control_entity))?;
                    db.insert_entity_component(string_entity, String::default())?;
                    db.insert_entity_component(string_entity, ColorRGBF::default())?;

                    if db
                        .entity_component_directory
                        .entity_has_component::<DebugExclude>(&list_control_entity)
                    {
                        db.insert_entity_component(string_entity, DebugExclude)?;
                    }

                    string_entities.push(string_entity);
                }

                // Destroy item entities for lines that no longer exist
                while string_entities.len() > string_count {
                    if let Some(string_entity) = string_entities.pop() {
                        db.destroy_entity(string_entity)?;
                    }
                }

                // Fetch local mouse position
                let local_mouse_position: Option<Vector2I> =
                    match db.get_entity_component::<LocalPosition>(list_control_entity) {
                        Ok(local_position) => {
                            let local_position = *local_position;
                            let local_position: Vector2I = local_position.into();
                            Some(local_position)
                        }
                        Err(_) => None,
                    };

                // Determine whether the mouse is inside this control
                let contains_mouse = match local_mouse_position {
                    Some(Vector2I(mouse_x, mouse_y)) => {
                        let range_x = 0i64..width;
                        let range_y = 0i64..height as i64;
                        range_x.contains(&mouse_x) && range_y.contains(&mouse_y)
                    }
                    None => false,
                };

                // Convert the list of string vectors into a list of line heights, find the index of the first one lower than the mouse
                let hovered_item = if contains_mouse {
                    let Vector2I(_, mouse_y) = local_mouse_position.unwrap();
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

                // Update the list's IntRangeComponent to match the list size
                if let Some(list_index_entity) = list_index_entity {
                    if let Ok(int_range_component) =
                        db.get_entity_component_mut::<IntRange>(list_index_entity)
                    {
                        int_range_component.set_range(-1..(string_list.len() as i64));
                    }
                }

                // Clear local event queue
                if let Ok(list_event_queue) =
                    db.get_entity_component_mut::<EventQueue<ListEvent>>(list_control_entity)
                {
                    let list_event_queue: &mut Vec<ListEvent> = list_event_queue.as_mut();
                    list_event_queue.clear();
                }

                // If the mouse was clicked inside this control, update the selected index
                if contains_mouse {
                    let event_queue_entity =
                        db.entity_component_directory
                            .get_entity_by_predicate(|entity_id| {
                                db.entity_component_directory
                                    .entity_has_component::<EventQueue<AntigenInputEvent>>(
                                        entity_id,
                                    )
                            });

                    if let Some(event_queue_entity) = event_queue_entity {
                        let event_queue_component = db
                            .get_entity_component::<EventQueue<AntigenInputEvent>>(
                                event_queue_entity,
                            )?;

                        let event_queue: &Vec<AntigenInputEvent> = event_queue_component.as_ref();
                        for event in event_queue.clone() {
                            if let AntigenInputEvent::MousePress { button_mask: 1 } = event {
                                let index = if let Some(hovered_item) = hovered_item {
                                    hovered_item as i64
                                } else {
                                    -1
                                };

                                // Push press event into queue
                                if let Ok(list_event_queue) = db
                                    .get_entity_component_mut::<EventQueue<ListEvent>>(
                                        list_control_entity,
                                    )
                                {
                                    let list_event_queue: &mut Vec<ListEvent> =
                                        list_event_queue.as_mut();
                                    list_event_queue.push(ListEvent::Pressed(index));
                                }

                                if let Some(list_index_entity) = list_index_entity {
                                    if let Ok(int_range_component) =
                                        db.get_entity_component_mut::<IntRange>(list_index_entity)
                                    {
                                        int_range_component
                                            .set_range(-1..(string_list.len() as i64));

                                        int_range_component.set_index(index);
                                    }
                                }
                            }
                        }
                    }
                }

                // Fetch the selected index from the list's IntRangeComponent
                let focused_item = match list_index_entity {
                    Some(list_index_entity) => {
                        match db.get_entity_component_mut::<IntRange>(list_index_entity) {
                            Ok(int_range_component) => Some(int_range_component.get_index()),
                            Err(_) => None,
                        }
                    }
                    None => None,
                };

                *db.get_entity_component_mut::<Position>(*list_hover_entity)? = Vector2I(
                    0,
                    if let Some(hovered_item) = hovered_item {
                        hovered_item as i64
                    } else {
                        0
                    },
                )
                .into();

                *db.get_entity_component_mut::<Size>(*list_hover_entity)? =
                    if let Some(hovered_item) = hovered_item {
                        Vector2I(width, string_list[hovered_item as usize].len() as i64)
                    } else {
                        Vector2I(0, 0)
                    }
                    .into();

                *db.get_entity_component_mut::<Position>(*list_focus_entity)? =
                    Vector2I(0, focused_item.unwrap_or(0) as i64).into();

                *db.get_entity_component_mut::<Size>(*list_focus_entity)? =
                    if let Some(focused_item) = focused_item {
                        if focused_item >= 0 && focused_item < string_list.len() as i64 {
                            Vector2I(width, string_list[focused_item as usize].len() as i64)
                        } else {
                            Vector2I(0, 0)
                        }
                    } else {
                        Vector2I(0, 0)
                    }
                    .into();

                // Iterate over the lists of strings and update their position, text and color
                let mut y = 0i64;
                for (string_index, strings) in string_list.iter().enumerate() {
                    let mut done = false;
                    for string in strings {
                        let string_entity = string_entities[y as usize];

                        // Update each string entity's position
                        *db.get_entity_component_mut::<Position>(string_entity)? =
                            Vector2I(0, y).into();

                        // Update each string entity's text
                        *db.get_entity_component_mut::<String>(string_entity)? = string.clone();

                        // Update color pair based on focused item
                        let data = if Some(string_index as i64) == focused_item {
                            ColorRGB(0.0, 0.0, 0.0)
                        } else {
                            ColorRGB(1.0, 1.0, 1.0)
                        };

                        *db.get_entity_component_mut::<ColorRGBF>(string_entity)? = data;

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
                // The list control's string list has been removed, remove it from the set of string entities
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
