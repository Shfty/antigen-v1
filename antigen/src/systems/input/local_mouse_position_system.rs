use crate::{
    components::EventQueueComponent,
    components::{
        GlobalPositionComponent, ParentEntityComponent, PositionComponent, WindowComponent,
    },
    entity_component_system::SystemDebugTrait,
    entity_component_system::{
        system_interface::SystemInterface, ComponentStorage, EntityComponentDirectory, SystemError,
        SystemTrait,
    },
    events::AntigenEvent,
    primitive_types::Vector2I,
};

use crate::components::LocalMousePositionComponent;

#[derive(Debug)]
pub struct LocalMousePositionSystem;

impl Default for LocalMousePositionSystem {
    fn default() -> Self {
        LocalMousePositionSystem
    }
}

impl LocalMousePositionSystem {
    pub fn new() -> Self {
        LocalMousePositionSystem::default()
    }
}

impl<CS, CD> SystemTrait<CS, CD> for LocalMousePositionSystem
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
                        .entity_has_component::<EventQueueComponent<AntigenEvent>>(entity_id)
                });

        if let Some(event_queue_entity) = event_queue_entity {
            let event_queue = db
                .get_entity_component::<EventQueueComponent<AntigenEvent>>(event_queue_entity)?
                .get_events()
                .clone();

            for event in event_queue {
                let mouse_position = match event {
                    AntigenEvent::MouseMove { position, delta: _ } => position,
                    _ => continue,
                };

                let entities =
                    db.entity_component_directory
                        .get_entities_by_predicate(|entity_id| {
                            db.entity_component_directory
                                .entity_has_component::<LocalMousePositionComponent>(entity_id)
                                && db
                                    .entity_component_directory
                                    .entity_has_component::<PositionComponent>(entity_id)
                        });

                for entity_id in entities {
                    let mut candidate_id = entity_id;
                    let mut window_position = Vector2I::default();
                    loop {
                        if let Ok(parent_entity_component) =
                            db.get_entity_component::<ParentEntityComponent>(candidate_id)
                        {
                            candidate_id = parent_entity_component.get_parent_id();
                        } else {
                            break;
                        }

                        if db
                            .get_entity_component::<WindowComponent>(candidate_id)
                            .is_ok()
                        {
                            let position_component =
                                db.get_entity_component::<PositionComponent>(candidate_id)?;
                            window_position = position_component.get_position();
                            break;
                        }
                    }

                    let position = match db
                        .get_entity_component::<GlobalPositionComponent>(entity_id)
                    {
                        Ok(global_position_component) => {
                            global_position_component.get_global_position()
                        }
                        Err(_) => match db.get_entity_component::<PositionComponent>(entity_id) {
                            Ok(position_component) => position_component.get_position(),
                            Err(err) => return Err(err.into()),
                        },
                    };

                    db.get_entity_component_mut::<LocalMousePositionComponent>(entity_id)?
                        .set_local_mouse_position(mouse_position - (window_position + position));
                }
            }
        }

        Ok(())
    }
}

impl SystemDebugTrait for LocalMousePositionSystem {
    fn get_name() -> &'static str {
        "Local Mouse Position"
    }
}
