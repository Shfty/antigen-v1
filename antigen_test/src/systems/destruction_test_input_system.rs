use antigen::{
    components::EventQueueComponent,
    entity_component_system::ComponentStorage,
    entity_component_system::EntityComponentDirectory,
    entity_component_system::SystemDebugTrait,
    entity_component_system::SystemError,
    entity_component_system::{system_interface::SystemInterface, SystemTrait},
    events::AntigenEvent,
};

use crate::components::destruction_test_input_component::DestructionTestInputComponent;

#[derive(Debug)]
pub struct DestructionTestInputSystem;

impl DestructionTestInputSystem {
    pub fn new() -> Self {
        DestructionTestInputSystem
    }
}

impl<CS, CD> SystemTrait<CS, CD> for DestructionTestInputSystem
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
            let destruction_test_entities = db
                .entity_component_directory
                .get_entities_by_predicate(|entity_id| {
                    db.entity_component_directory
                        .entity_has_component::<DestructionTestInputComponent>(entity_id)
                });

            for entity_id in destruction_test_entities {
                let input_key = db
                    .get_entity_component::<DestructionTestInputComponent>(entity_id)?
                    .get_input_key();

                for event in db
                    .get_entity_component::<EventQueueComponent<AntigenEvent>>(event_queue_entity)?
                    .get_events()
                    .clone()
                {
                    if let AntigenEvent::KeyPress { key_code } = event {
                        if key_code == input_key {
                            db.destroy_entity(entity_id)?;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

impl SystemDebugTrait for DestructionTestInputSystem {
    fn get_name() -> &'static str {
        "Destruction Test Input"
    }
}
