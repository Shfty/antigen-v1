use std::borrow::Borrow;

use antigen::{
    components::EventQueue,
    core::events::AntigenInputEvent,
    entity_component_system::ComponentStorage,
    entity_component_system::EntityComponentDirectory,
    entity_component_system::SystemDebugTrait,
    entity_component_system::SystemError,
    entity_component_system::{system_interface::SystemInterface, SystemTrait},
};

use crate::components::DestructionTestInput;

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
                        .entity_has_component::<EventQueue<AntigenInputEvent>>(entity_id)
                });

        if let Some(event_queue_entity) = event_queue_entity {
            let destruction_test_entities = db
                .entity_component_directory
                .get_entities_by_predicate(|entity_id| {
                    db.entity_component_directory
                        .entity_has_component::<DestructionTestInput>(entity_id)
                });

            for entity_id in destruction_test_entities {
                let input_key: antigen::core::keyboard::Key =
                    (*db.get_entity_component::<DestructionTestInput>(entity_id)?).into();

                let event_queue: &Vec<AntigenInputEvent> = db
                    .get_entity_component::<EventQueue<AntigenInputEvent>>(event_queue_entity)?
                    .borrow();

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

impl SystemDebugTrait for DestructionTestInputSystem {
    fn get_name() -> &'static str {
        "Destruction Test Input"
    }
}
