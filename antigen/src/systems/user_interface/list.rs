use crate::{components::Name, primitive_types::HashMap};
use std::{cell::Ref, cell::RefMut};

use crate::{
    components::{
        Control, DebugExclude, EventQueue, GlobalPositionData, ListData, LocalMousePositionData,
        ParentEntity, Position, Size,
    },
    core::events::AntigenInputEvent,
    entity_component_system::{ComponentStore, EntityID, SystemError, SystemTrait},
    primitive_types::{ColorRGB, ColorRGBF, Vector2I},
};

use store::{Assembler, StoreQuery};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ListEvent {
    Hovered(i64),
    Pressed(Option<usize>),
}

#[derive(Debug, Default)]
pub struct List {
    // Maps list control entities -> rectangle entities
    list_focus_entities: HashMap<EntityID, EntityID>,

    // Maps list control entities -> rectangle entities
    list_hover_entities: HashMap<EntityID, EntityID>,

    // Maps list control entities -> string entities
    list_string_entities: HashMap<EntityID, Vec<EntityID>>,
}

impl SystemTrait for List {
    fn run<'a>(&mut self, db: &mut ComponentStore) -> Result<(), SystemError> {
        let list_control_entities: Vec<EntityID> = StoreQuery::<(
            EntityID,
            Ref<ListData>,
            Ref<Position>,
            Ref<Size>,
            Ref<ParentEntity>,
        )>::iter(db.as_ref())
        .map(|(entity_id, _, _, _, _)| entity_id)
        .collect();

        for list_control_entity in list_control_entities {
            self.list_hover_entities
                .entry(list_control_entity)
                .or_insert_with(|| {
                    let entity_id = EntityID::next();
                    
                    Assembler::new()
                        .key(entity_id)
                        .fields((
                            Name("List Hover Entity".into()),
                            Control,
                            Position::default(),
                            Size::default(),
                            GlobalPositionData::default(),
                            ColorRGB(0.5f32, 0.5f32, 0.5f32),
                            ParentEntity(list_control_entity),
                        ))
                        .finish(db);
                        
                    entity_id
                });

            let list_hover_entity = self
                .list_hover_entities
                .get(&list_control_entity)
                .ok_or("Error getting list hover entity")?;

            self.list_focus_entities
                .entry(list_control_entity)
                .or_insert_with(|| {
                    let entity_id = EntityID::next();

                    Assembler::new()
                        .key(entity_id)
                        .fields((
                            Name("List Focus Entity".into()),
                            Control,
                            Position::default(),
                            Size::default(),
                            GlobalPositionData::default(),
                            ParentEntity(list_control_entity),
                        ))
                        .finish(db);

                    entity_id
                });

            let list_focus_entity = self
                .list_focus_entities
                .get(&list_control_entity)
                .ok_or("Error getting list focus entity")?;

            // Fetch entity references
            let (string_list_entity, scroll_offset) = match db.get::<ListData>(&list_control_entity)
            {
                Some(pancurses_list_control_component) => (
                    pancurses_list_control_component.get_string_list_entity(),
                    pancurses_list_control_component.get_scroll_offset(),
                ),
                None => return Err("Failed to get ListData".into()),
            };

            if let Some(string_list_entity) = string_list_entity {
                // The list entity is valid

                // Fetch width and height
                let Vector2I(width, height) = match db.get::<Size>(&list_control_entity) {
                    Some(size_component) => **size_component,
                    None => return Err("Failed to get Size component".into()),
                };

                // Fetch strings
                let string_list: Vec<Vec<String>> = db
                    .get::<Vec<String>>(&string_list_entity)
                    .expect("Failed to get string list")
                    .iter()
                    .skip(scroll_offset)
                    .take(height as usize)
                    .map(|string| {
                        let substrings: Vec<String> = string
                            .split('\n')
                            .map(|string| {
                                string
                                    .chars()
                                    .take(std::cmp::min(width as usize, string.len() as usize))
                                    .collect::<String>()
                            })
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
                let string_count = std::cmp::min(string_count, height as usize);
                while string_entities.len() < string_count {
                    let string_entity = EntityID::next();

                    Assembler::new()
                        .key(string_entity)
                        .fields((
                            Name("List String Entity".into()),
                            Control,
                            Position::default(),
                            GlobalPositionData::default(),
                            ParentEntity(list_control_entity),
                            String::default(),
                            ColorRGBF::default(),
                            DebugExclude,
                        ))
                        .finish(db);

                    string_entities.push(string_entity);
                }

                // Destroy item entities for lines that no longer exist
                while string_entities.len() > string_count {
                    if let Some(string_entity) = string_entities.pop() {
                        db.remove_key(&string_entity);
                    }
                }

                // Fetch local mouse position
                let local_mouse_position: Option<Vector2I> =
                    match db.get::<LocalMousePositionData>(&list_control_entity) {
                        Some(local_position) => {
                            let local_position = *local_position;
                            let local_position: Vector2I = local_position.into();
                            Some(local_position)
                        }
                        None => None,
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

                // Clear local event queue
                if let Some(mut list_event_queue) =
                    db.get_mut::<EventQueue<ListEvent>>(&list_control_entity)
                {
                    list_event_queue.clear();
                }

                // If the mouse was clicked inside this control, update the selected index
                if contains_mouse {
                    if let Some((_, event_queue)) = StoreQuery::<(
                        EntityID,
                        Ref<EventQueue<AntigenInputEvent>>,
                    )>::iter(db.as_ref())
                    .next()
                    {
                        for event in event_queue.iter() {
                            match event {
                                AntigenInputEvent::MousePress { button_mask: 1 } => {
                                    let index = if let Some(hovered_item) = hovered_item {
                                        Some(hovered_item)
                                    } else {
                                        None
                                    };

                                    // Push press event into queue
                                    let (_, mut list, list_event_queue) = StoreQuery::<(
                                        EntityID,
                                        RefMut<ListData>,
                                        Option<RefMut<EventQueue<ListEvent>>>,
                                    )>::get(
                                        db.as_ref(),
                                        &list_control_entity,
                                    );

                                    if let Some(mut list_event_queue) = list_event_queue {
                                        list_event_queue.push(ListEvent::Pressed(index));
                                    }

                                    list.set_selected_index(index);
                                }
                                AntigenInputEvent::MouseScroll { delta } => {
                                    let (_, mut list) =
                                        StoreQuery::<(EntityID, RefMut<ListData>)>::get(
                                            db.as_ref(),
                                            &list_control_entity,
                                        );

                                    list.add_scroll_offset(*delta as i64);
                                }
                                _ => (),
                            }
                        }
                    }
                }

                // Fetch focused / selected indices
                let selected_item = db
                    .get_mut::<ListData>(&list_control_entity)
                    .unwrap()
                    .get_selected_index();

                **db.get_mut::<Position>(list_hover_entity).unwrap() = Vector2I(
                    0,
                    if let Some(hovered_item) = hovered_item {
                        hovered_item as i64
                    } else {
                        0
                    },
                );

                **db.get_mut::<Size>(list_hover_entity).unwrap() =
                    if let Some(hovered_item) = hovered_item {
                        Vector2I(width, string_list[hovered_item as usize].len() as i64)
                    } else {
                        Vector2I(0, 0)
                    };

                **db.get_mut::<Position>(list_focus_entity).unwrap() =
                    Vector2I(0, selected_item.unwrap_or(0) as i64);

                **db.get_mut::<Size>(list_focus_entity).unwrap() =
                    if let Some(focused_item) = selected_item {
                        if focused_item < string_list.len() {
                            Vector2I(width, string_list[focused_item as usize].len() as i64)
                        } else {
                            Vector2I(0, 0)
                        }
                    } else {
                        Vector2I(0, 0)
                    };

                // Iterate over the lists of strings and update their position, text and color
                let mut y = 0i64;
                for (string_index, strings) in string_list.iter().enumerate() {
                    let mut done = false;

                    for string in strings {
                        if y >= height {
                            done = true;
                            break;
                        }

                        let string_entity = string_entities[y as usize];

                        // Update each string entity's position
                        **db.get_mut::<Position>(&string_entity).unwrap() = Vector2I(0, y);

                        // Update each string entity's text
                        *db.get_mut::<String>(&string_entity).unwrap() = string.clone();

                        // Update color pair based on focused item
                        let data = if Some(string_index) == selected_item {
                            ColorRGB(0.0, 0.0, 0.0)
                        } else {
                            ColorRGB(1.0, 1.0, 1.0)
                        };

                        *db.get_mut::<ColorRGBF>(&string_entity).unwrap() = data;

                        y += 1;
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
