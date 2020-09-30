use antigen::{
    components::EventQueue,
    core::events::AntigenInputEvent,
    entity_component_system::ComponentStorage,
    entity_component_system::EntityComponentDirectory,
    entity_component_system::SystemError,
    entity_component_system::{system_interface::SystemInterface, SystemTrait},
};

use crate::components::DestructionTestInputData;

#[derive(Debug)]
pub struct DestructionTestInput;

impl DestructionTestInput {
    pub fn new() -> Self {
        DestructionTestInput
    }
}

impl<CS, CD> SystemTrait<CS, CD> for DestructionTestInput
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
            let destruction_test_entities = db
                .entity_component_directory
                .get_entities_by_predicate(|entity_id| {
                    db.entity_component_directory
                        .entity_has_component::<DestructionTestInputData>(entity_id)
                });

            for entity_id in destruction_test_entities {
                let input_key: antigen::core::keyboard::Key =
                    **db.get_entity_component::<DestructionTestInputData>(entity_id)?;

                let event_queue: &Vec<AntigenInputEvent> =
                    db.get_entity_component::<EventQueue<AntigenInputEvent>>(event_queue_entity)?;

                for event in event_queue.clone() {
                    if let AntigenInputEvent::KeyPress { key_code } = event {
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
