use antigen::{
    entity_component_system::ComponentStorage,
    entity_component_system::EntityComponentDirectory,
    entity_component_system::SystemDebugTrait,
    entity_component_system::SystemError,
    entity_component_system::{system_interface::SystemInterface, SystemTrait},
};
use pancurses::Input;

use crate::components::{
    destruction_test_input_component::DestructionTestInputComponent,
    pancurses_input_buffer_component::PancursesInputBufferComponent,
};

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
        let destruction_test_components =
            db.entity_component_directory
                .get_entities_by_predicate(|entity_id| {
                    db.entity_component_directory
                        .entity_has_component::<DestructionTestInputComponent>(entity_id)
                });

        for entity_id in destruction_test_components {
            let input_char = pancurses::Input::Character(
                db.get_entity_component::<DestructionTestInputComponent>(entity_id)?
                    .get_input_char(),
            );

            let inputs: Vec<Input> = db
                .get_entity_component_mut::<PancursesInputBufferComponent>(entity_id)?
                .get_inputs();

            for input in inputs {
                if input == input_char {
                    db.destroy_entity(entity_id)?;
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
