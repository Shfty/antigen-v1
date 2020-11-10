use crate::{
    assemblage::{ComponentBuilder, EntityBuilder, MapComponentBuilder, MapEntityBuilder},
    components::{GlobalZIndex, Name, SoftwareShader},
    primitive_types::HashMap,
    systems::LocalMousePress,
    systems::LocalMouseScroll,
};
use std::{cell::Ref, cell::RefMut, cmp::max, cmp::min, collections::hash_map::Entry};

use crate::{
    components::{
        EventQueue, GlobalPosition, ListData, LocalMousePositionData, ParentEntity, Position, Size,
        StringShader,
    },
    entity_component_system::{ComponentStore, EntityID, SystemError, SystemTrait},
    primitive_types::{ColorRGBF, Vector2I},
};

use store::{StoreBuilder, StoreQuery};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ListHovered(pub i64);

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ListPressed(pub Option<usize>);

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

    fn populate_hover_focus_entities(&mut self, db: &mut ComponentStore) {
        let builders: Vec<EntityBuilder> =
            StoreQuery::<(EntityID, Ref<Size>, Ref<Vec<String>>, Ref<ListData>)>::iter(db.as_ref())
                .flat_map(|(entity_id, _, _, _)| {
                    let mut builders: Vec<EntityBuilder> = Vec::with_capacity(2);

                    if let Entry::Vacant(entry) = self.list_hover_entities.entry(entity_id) {
                        let list_hover_entity = EntityID::next();
                        entry.insert(list_hover_entity);
                        builders.push(
                            EntityBuilder::new().map(hover_highlight(list_hover_entity, entity_id)),
                        )
                    }

                    if let Entry::Vacant(entry) = self.list_focus_entities.entry(entity_id) {
                        let list_focus_entities = EntityID::next();
                        entry.insert(list_focus_entities);
                        builders.push(
                            EntityBuilder::new()
                                .map(focus_highlight(list_focus_entities, entity_id)),
                        )
                    }

                    builders
                })
                .collect();

        for builder in builders {
            builder.finish(db.as_mut());
        }
    }

    fn populate_string_entities(&mut self, db: &mut ComponentStore) {
        let mut builders: Vec<EntityBuilder> = vec![];
        let mut keys_to_remove: Vec<EntityID> = vec![];

        // Batch up sets of entities to add and remove
        for (entity_id, size, strings, list_data) in
            StoreQuery::<(EntityID, Ref<Size>, Ref<Vec<String>>, Ref<ListData>)>::iter(db.as_ref())
        {
            let Vector2I(_, height) = **size;
            let scroll_offset = list_data.get_scroll_offset();

            self.list_string_entities.entry(entity_id).or_default();

            let string_entities = self
                .list_string_entities
                .get_mut(&entity_id)
                .expect("Failed to get list string entities");

            let string_count: i64 = strings
                .iter()
                .map(|string| (string.matches('\n').count() + 1) as i64)
                .sum();
            let string_count = min(string_count - scroll_offset as i64, height);
            let string_count = max(string_count, 0);
            let string_count = string_count as usize;

            let mut builder = StoreBuilder::new();
            while string_entities.len() < string_count {
                let string_entity = EntityID::next();
                builder = builder
                    .key(string_entity)
                    .map(list_item(entity_id))
                    .map(self.list_item_assembler)
                    .finish();
                string_entities.push(string_entity);
            }
            builders.push(builder);

            // Batch removal for entities for lines that no longer exist
            while string_entities.len() > string_count {
                if let Some(string_entity) = string_entities.pop() {
                    keys_to_remove.push(string_entity);
                }
            }
        }

        // Execute add / remove
        for builder in builders {
            builder.finish(db.as_mut());
        }

        for entity_id in keys_to_remove {
            db.remove_key(&entity_id);
        }
    }

    fn handle_input(&mut self, db: &mut ComponentStore) {
        for (entity_id, size, strings, local_mouse_position, mut list_data) in
            StoreQuery::<(
                EntityID,
                Ref<Size>,
                Ref<Vec<String>>,
                Option<Ref<LocalMousePositionData>>,
                RefMut<ListData>,
            )>::iter(db.as_ref())
        {
            let Vector2I(width, height) = **size;
            drop(size);

            // Fetch local mouse position
            let local_mouse_position: Option<Vector2I> =
                local_mouse_position.map(|local_mouse_position| (*local_mouse_position).into());

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
            let string_indices: Vec<usize> = strings
                .iter()
                .enumerate()
                .flat_map(|(i, string)| string.split('\n').map(move |_| i))
                .collect();

            let hovered_item = if contains_mouse {
                let Vector2I(_, mouse_y) = local_mouse_position.unwrap();
                let local_y = mouse_y + list_data.get_scroll_offset() as i64;
                if (local_y as usize) < string_indices.len() {
                    Some(string_indices[local_y as usize])
                } else {
                    None
                }
            } else {
                None
            };

            // Clear local event queue
            if let Some(mut list_event_queue) = db.get_mut::<EventQueue<ListPressed>>(&entity_id) {
                list_event_queue.clear();
            }

            // If the mouse was clicked inside this control, update the selected index
            if let (_, Some(event_queue)) = StoreQuery::<(
                EntityID,
                Option<Ref<EventQueue<LocalMousePress>>>,
            )>::get(db.as_ref(), &entity_id)
            {
                for event in event_queue.iter() {
                    if event.button_mask == 1 {
                        let index = if let Some(hovered_item) = hovered_item {
                            Some(hovered_item)
                        } else {
                            None
                        };

                        // Push press event into queue
                        let (_, list_event_queue) =
                            StoreQuery::<(EntityID, Option<RefMut<EventQueue<ListPressed>>>)>::get(
                                db.as_ref(),
                                &entity_id,
                            );

                        if let Some(mut list_event_queue) = list_event_queue {
                            list_event_queue.push(ListPressed(index));
                        }

                        list_data.set_selected_index(index.map(|i| i));
                    }
                }
            }

            if let (_, Some(event_queue)) = StoreQuery::<(
                EntityID,
                Option<Ref<EventQueue<LocalMouseScroll>>>,
            )>::get(db.as_ref(), &entity_id)
            {
                for event in event_queue.iter() {
                    list_data.add_scroll_offset(event.delta as i64);
                }
            }

            // Fetch focused / selected indices
            let selected_item = list_data.get_selected_index();
            let list_hover_entity = self.list_hover_entities[&entity_id];
            let list_focus_entity = self.list_focus_entities[&entity_id];

            let string_list: Vec<Vec<String>> = strings
                .iter()
                .map(|string| {
                    string
                        .split('\n')
                        .map(|string| {
                            string
                                .chars()
                                .take(min(width as usize, string.len() as usize))
                                .collect::<String>()
                        })
                        .collect()
                })
                .collect();

            {
                let (_, mut position, mut size) =
                    StoreQuery::<(EntityID, RefMut<Position>, RefMut<Size>)>::get(
                        db.as_ref(),
                        &list_hover_entity,
                    );
                **position = Vector2I(
                    0,
                    if let Some(hovered_item) = hovered_item {
                        let idx: i64 = string_list
                            .iter()
                            .take(hovered_item)
                            .map(|strings| strings.len() as i64)
                            .sum();
                        idx - list_data.get_scroll_offset() as i64
                    } else {
                        0
                    },
                );

                **size = if let Some(hovered_item) = hovered_item {
                    if hovered_item < string_list.len() {
                        Vector2I(width, string_list[hovered_item as usize].len() as i64)
                    } else {
                        Vector2I::ZERO
                    }
                } else {
                    Vector2I::ZERO
                };
            }

            {
                let (_, mut position, mut size) =
                    StoreQuery::<(EntityID, RefMut<Position>, RefMut<Size>)>::get(
                        db.as_ref(),
                        &list_focus_entity,
                    );

                **position = Vector2I(
                    0,
                    if let Some(selected_item) = selected_item {
                        let idx: i64 = string_list
                            .iter()
                            .take(selected_item)
                            .map(|strings| strings.len() as i64)
                            .sum();
                        idx - list_data.get_scroll_offset() as i64
                    } else {
                        0
                    },
                );

                **size = if let Some(selected_item) = selected_item {
                    if position.1 >= 0 && position.1 < height && selected_item < string_list.len() {
                        Vector2I(width, string_list[selected_item as usize].len() as i64)
                    } else {
                        Vector2I(0, 0)
                    }
                } else {
                    Vector2I(0, 0)
                };
            }
        }
    }

    fn update_string_entities(&mut self, db: &mut ComponentStore) {
        for (entity_id, size, strings, list_data, _) in StoreQuery::<(
            EntityID,
            Ref<Size>,
            Ref<Vec<String>>,
            Ref<ListData>,
            Option<Ref<LocalMousePositionData>>,
        )>::iter(db.as_ref())
        {
            let Vector2I(width, height) = **size;

            // Iterate over the lists of strings and update their position, text and color
            let string_entities = self
                .list_string_entities
                .get_mut(&entity_id)
                .expect("Failed to get list string entities");

            for (string_index, line_string) in strings
                .iter()
                .flat_map(|string| {
                    string.split('\n').map(|string| {
                        string
                            .chars()
                            .take(min(width as usize, string.len() as usize))
                            .collect::<String>()
                    })
                })
                .skip(list_data.get_scroll_offset())
                .take(height as usize)
                .enumerate()
            {
                let y = string_index as i64;
                // Update string entity
                let (_, mut position, mut string, mut color) =
                    StoreQuery::<(
                        EntityID,
                        RefMut<Position>,
                        RefMut<String>,
                        RefMut<ColorRGBF>,
                    )>::get(db.as_ref(), &string_entities[y as usize]);

                **position = Vector2I(0, y);
                *string = line_string.clone();
                *color = if Some(string_index) == list_data.get_selected_index() {
                    ColorRGBF::new(0.0, 0.0, 0.0)
                } else {
                    ColorRGBF::new(1.0, 1.0, 1.0)
                };
            }
        }
    }
}

impl SystemTrait for List {
    fn run<'a>(&mut self, db: &mut ComponentStore) -> Result<(), SystemError> {
        self.populate_hover_focus_entities(db);
        self.handle_input(db);
        self.populate_string_entities(db);
        self.update_string_entities(db);

        Ok(())
    }
}
