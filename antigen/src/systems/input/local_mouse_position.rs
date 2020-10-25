use std::cell::{Ref, RefMut};

use store::StoreQuery;

use crate::{
    components::EventQueue,
    components::{GlobalPositionData, ParentEntity, Position, Window},
    core::events::AntigenInputEvent,
    entity_component_system::{
        system_interface::SystemInterface, EntityComponentDirectory, EntityID, SystemError,
        SystemTrait,
    },
    primitive_types::Vector2I,
};

use crate::components::LocalMousePositionData;

#[derive(Debug)]
pub struct LocalMousePosition;

impl Default for LocalMousePosition {
    fn default() -> Self {
        LocalMousePosition
    }
}

impl LocalMousePosition {
    pub fn new() -> Self {
        LocalMousePosition::default()
    }
}

impl<CD> SystemTrait<CD> for LocalMousePosition
where
    CD: EntityComponentDirectory,
{
    fn run(&mut self, db: &mut SystemInterface<CD>) -> Result<(), SystemError>
    where
        CD: EntityComponentDirectory,
    {
        let (_, event_queue) =
            StoreQuery::<(EntityID, Ref<EventQueue<AntigenInputEvent>>)>::iter(db.component_store)
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
                )>::iter(db.component_store)
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
                        )>::get(db.component_store, &candidate_id);

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
