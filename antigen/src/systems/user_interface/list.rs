use crate::{
    assemblage::{ComponentBuilder, EntityBuilder, MapComponentBuilder, MapEntityBuilder},
    components::{GlobalZIndex, Name, SoftwareShader},
    primitive_types::HashMap,
};
use std::{cell::Ref, cell::RefMut};

use crate::{
    components::{
        EventQueue, GlobalPosition, ListData, LocalMousePositionData, ParentEntity, Position, Size,
        StringShader,
    },
    core::events::AntigenInputEvent,
    entity_component_system::{ComponentStore, EntityID, SystemError, SystemTrait},
    primitive_types::{ColorRGBF, Vector2I},
};

use store::{StoreBuilder, StoreQuery};

type ReadListEntities<'a> = (
    EntityID,
    Ref<'a, Position>,
    Ref<'a, Size>,
    Ref<'a, ListData>,
);

type ReadAntigenEventQueueEntity<'a> = (EntityID, Ref<'a, EventQueue<AntigenInputEvent>>);

type WriteListEventQueue<'a> = (
    EntityID,
    RefMut<'a, ListData>,
    Option<RefMut<'a, EventQueue<ListEvent>>>,
);

type WriteListData<'a> = (EntityID, RefMut<'a, ListData>);

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ListEvent {
    Hovered(i64),
    Pressed(Option<usize>),
}

#[derive(Debug)]
pub struct List {
    // Maps list control entities -> rectangle entities
    list_focus_entities: HashMap<EntityID, EntityID>,

    // Maps list control entities -> rectangle entities
    list_hover_entities: HashMap<EntityID, EntityID>,

    // Maps list control entities -> string entities
    list_string_entities: HashMap<EntityID, Vec<EntityID>>,

    list_item_assembler: fn(ComponentBuilder) -> ComponentBuilder,
}

impl List {
    pub fn new(list_item_assembler: fn(ComponentBuilder) -> ComponentBuilder) -> Self {
        List {
            list_focus_entities: Default::default(),
            list_hover_entities: Default::default(),
            list_string_entities: Default::default(),
            list_item_assembler,
        }
    }
}

fn hover_highlight(entity_id: EntityID, parent_entity: EntityID) -> impl MapEntityBuilder {
    move |builder: EntityBuilder| {
        builder.key_fields(
            entity_id,
            (
                Name("List Hover Entity".into()),
                Position::default(),
                Size::default(),
                GlobalPosition::default(),
                GlobalZIndex::default(),
                SoftwareShader::color(ColorRGBF::new(0.5, 0.5, 0.5)),
                ParentEntity(parent_entity),
            ),
        )
    }
}

fn focus_highlight(entity_id: EntityID, parent_entity: EntityID) -> impl MapEntityBuilder {
    move |builder: EntityBuilder| {
        builder.key_fields(
            entity_id,
            (
                Name("List Focus Entity".into()),
                Position::default(),
                Size::default(),
                GlobalPosition::default(),
                GlobalZIndex::default(),
                SoftwareShader::color(ColorRGBF::new(1.0, 1.0, 1.0)),
                ParentEntity(parent_entity),
            ),
        )
    }
}

fn list_item(parent_id: EntityID) -> impl MapComponentBuilder {
    move |builder: ComponentBuilder| {
        builder.fields((
            Name("List String Entity".into()),
            StringShader,
            Position::default(),
            GlobalPosition::default(),
            GlobalZIndex::default(),
            ParentEntity(parent_id),
            String::default(),
            ColorRGBF::default(),
        ))
    }
}

impl SystemTrait for List {
    fn run<'a>(&mut self, db: &mut ComponentStore) -> Result<(), SystemError> {
        let list_control_entities: Vec<EntityID> =
            StoreQuery::<ReadListEntities>::iter(db.as_ref())
                .map(|(entity_id, _, _, _)| entity_id)
                .collect();

        for list_control_entity in list_control_entities {
            let list_hover_entity = self
                .list_hover_entities
                .entry(list_control_entity)
                .or_insert_with(|| {
                    let entity_id = EntityID::next();

                    StoreBuilder::new()
                        .map(hover_highlight(entity_id, list_control_entity))
                        .finish(db);

                    entity_id
                });

            let list_focus_entity = self
                .list_focus_entities
                .entry(list_control_entity)
                .or_insert_with(|| {
                    let entity_id = EntityID::next();

                    StoreBuilder::new()
                        .map(focus_highlight(entity_id, list_control_entity))
                        .finish(db);

                    entity_id
                });

            // Fetch entity references
            let pancurses_list_control_component = db
                .get::<ListData>(&list_control_entity)
                .expect("Failed to get ListData");

            let scroll_offset = pancurses_list_control_component.get_scroll_offset();

            drop(pancurses_list_control_component);

            // The list entity is valid

            // Fetch size
            let Vector2I(width, height) = **db
                .get::<Size>(&list_control_entity)
                .expect("Failed to get size component");

            // Fetch strings
            let string_list: Vec<Vec<String>> = db
                .get::<Vec<String>>(&list_control_entity)
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

            let mut builder = StoreBuilder::new();
            while string_entities.len() < string_count {
                let string_entity = EntityID::next();
                builder = builder
                    .key(string_entity)
                    .map(list_item(list_control_entity))
                    .map(self.list_item_assembler)
                    .finish();
                string_entities.push(string_entity);
            }
            builder.finish(db);

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
                if let Some((_, event_queue)) =
                    StoreQuery::<ReadAntigenEventQueueEntity>::iter(db.as_ref()).next()
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
                                let (_, mut list, list_event_queue) =
                                    StoreQuery::<WriteListEventQueue>::get(
                                        db.as_ref(),
                                        &list_control_entity,
                                    );

                                if let Some(mut list_event_queue) = list_event_queue {
                                    list_event_queue.push(ListEvent::Pressed(index));
                                }

                                list.set_selected_index(index);
                            }
                            AntigenInputEvent::MouseScroll { delta } => {
                                let (_, mut list) = StoreQuery::<WriteListData>::get(
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
                        ColorRGBF::new(0.0, 0.0, 0.0)
                    } else {
                        ColorRGBF::new(1.0, 1.0, 1.0)
                    };

                    *db.get_mut::<ColorRGBF>(&string_entity).unwrap() = data;

                    y += 1;
                }

                if done {
                    break;
                }
            }
        }

        Ok(())
    }
}
