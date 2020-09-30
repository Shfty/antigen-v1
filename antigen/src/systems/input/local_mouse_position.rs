use crate::{
    components::EventQueue,
    components::{GlobalPositionData, ParentEntity, Position, Window},
    core::events::AntigenInputEvent,
    entity_component_system::{
        system_interface::SystemInterface, ComponentStorage, EntityComponentDirectory, SystemError,
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

impl<CS, CD> SystemTrait<CS, CD> for LocalMousePosition
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
{
    fn run(&mut self, db: &mut SystemInterface<CS, CD>) -> Result<(), SystemError>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
    {
        let event_queue_entity =
            db.entity_component_directory
                .get_entity_by_predicate(|entity_id| {
                    db.entity_component_directory
                        .entity_has_component::<EventQueue<AntigenInputEvent>>(entity_id)
                });

        if let Some(event_queue_entity) = event_queue_entity {
            let event_queue: &Vec<AntigenInputEvent> =
                db.get_entity_component::<EventQueue<AntigenInputEvent>>(event_queue_entity)?;

            for event in event_queue.clone() {
                let mouse_position = match event {
                    AntigenInputEvent::MouseMove { position, delta: _ } => position,
                    _ => continue,
                };

                let entities =
                    db.entity_component_directory
                        .get_entities_by_predicate(|entity_id| {
                            db.entity_component_directory
                                .entity_has_component::<LocalMousePositionData>(entity_id)
                                && db
                                    .entity_component_directory
                                    .entity_has_component::<Position>(entity_id)
                        });

                for entity_id in entities {
                    let mut candidate_id = entity_id;
                    let mut window_position = Vector2I::default();
                    loop {
                        if let Ok(parent_entity) =
                            db.get_entity_component::<ParentEntity>(candidate_id)
                        {
                            candidate_id = **parent_entity;
                        } else {
                            break;
                        }

                        if db.get_entity_component::<Window>(candidate_id).is_ok() {
                            window_position =
                                **db.get_entity_component::<Position>(candidate_id)?;
                            break;
                        }
                    }

                    let position: Vector2I =
                        match db.get_entity_component::<GlobalPositionData>(entity_id) {
                            Ok(global_position) => **global_position,
                            Err(_) => match db.get_entity_component::<Position>(entity_id) {
                                Ok(position) => **position,
                                Err(err) => return Err(err.into()),
                            },
                        };

                    *db.get_entity_component_mut::<LocalMousePositionData>(entity_id)? =
                        (mouse_position - (window_position + position)).into();
                }
            }
        }

        Ok(())
    }
}
