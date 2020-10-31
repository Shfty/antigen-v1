use std::cell::{Ref, RefMut};

use store::StoreQuery;

use crate::{
    components::EventQueue,
    components::{GlobalPositionData, ParentEntity, Position, Window},
    core::events::AntigenInputEvent,
    entity_component_system::{
        ComponentStore, EntityID, SystemError, SystemTrait,
    },
    primitive_types::Vector2I,
};

use crate::components::LocalMousePositionData;

#[derive(Debug)]
pub struct LocalMousePosition;

impl SystemTrait for LocalMousePosition {
    fn run(&mut self, db: &mut ComponentStore) -> Result<(), SystemError> {
        let (_, event_queue) =
            StoreQuery::<(EntityID, Ref<EventQueue<AntigenInputEvent>>)>::iter(db.as_ref())
                .next()
                .expect("No antigen input event queue");

        for event in event_queue.iter() {
            let mouse_position = match event {
                AntigenInputEvent::MouseMove { position, delta: _ } => position,
                _ => continue,
            };

            for (entity_id, position, global_position, mut local_mouse_position) in
                StoreQuery::<(
                    EntityID,
                    Ref<Position>,
                    Option<Ref<GlobalPositionData>>,
                    RefMut<LocalMousePositionData>,
                )>::iter(db.as_ref())
            {
                let mut candidate_id = entity_id;
                let mut window_position = Vector2I::default();
                loop {
                    let (_, parent_entity, window, position) =
                        StoreQuery::<(
                            EntityID,
                            Option<Ref<ParentEntity>>,
                            Option<Ref<Window>>,
                            Option<Ref<Position>>,
                        )>::get(db.as_ref(), &candidate_id);

                    if let Some(parent_entity) = parent_entity {
                        candidate_id = **parent_entity;

                        if window.is_some() {
                            window_position = **position.unwrap();
                            break;
                        }
                    } else {
                        break;
                    }
                }

                let position: Vector2I = if let Some(global_position) = global_position {
                    **global_position
                } else {
                    **position
                };

                *local_mouse_position = (*mouse_position - (window_position + position)).into();
            }
        }

        Ok(())
    }
}
