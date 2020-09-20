use antigen::{
    components::EventQueueComponent,
    entity_component_system::system_interface::SystemInterface,
    entity_component_system::ComponentStorage,
    entity_component_system::EntityComponentDirectory,
    entity_component_system::SystemDebugTrait,
    entity_component_system::{SystemError, SystemTrait},
    events::AntigenEvent,
};

#[derive(Debug)]
pub struct QuitSystem;

impl<CS, CD> SystemTrait<CS, CD> for QuitSystem
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
            let event_queue_component =
                db.get_entity_component::<EventQueueComponent<AntigenEvent>>(event_queue_entity)?;

            for event in event_queue_component.get_events() {
                if let AntigenEvent::KeyPress { key_code } = event {
                    if *key_code == antigen::Key::Escape {
                        return Err(SystemError::Quit);
                    }
                }
            }
        }

        Ok(())
    }
}

impl SystemDebugTrait for QuitSystem {
    fn get_name() -> &'static str {
        "Quit"
    }
}
