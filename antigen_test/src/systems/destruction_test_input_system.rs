use antigen::{
    entity_component_system::entity_component_database::ComponentStorage,
    entity_component_system::entity_component_database::EntityComponentDirectory,
    entity_component_system::get_entity_component,
    entity_component_system::get_entity_component_mut,
    entity_component_system::SystemError,
    entity_component_system::{entity_component_database::EntityComponentDatabase, SystemTrait},
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
    fn run(&mut self, db: &mut EntityComponentDatabase<CS, CD>) -> Result<(), SystemError>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
    {
        let destruction_test_components = db.get_entities_by_predicate(|entity_id| {
            db.entity_has_component::<DestructionTestInputComponent>(entity_id)
        });

        for entity_id in destruction_test_components {
            let input_char = pancurses::Input::Character(
                get_entity_component::<CS, CD, DestructionTestInputComponent>(
                    &mut db.component_storage,
                    &mut db.entity_component_directory,
                    entity_id,
                )?
                .get_input_char(),
            );

            let inputs: Vec<Input> =
                get_entity_component_mut::<CS, CD, PancursesInputBufferComponent>(
                    &mut db.component_storage,
                    &mut db.entity_component_directory,
                    entity_id,
                )?
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
