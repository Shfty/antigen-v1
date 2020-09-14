use antigen::{
    ecs::SystemError,
    ecs::{EntityComponentDatabase, SystemTrait},
};

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

impl<T> SystemTrait<T> for DestructionTestInputSystem
where
    T: EntityComponentDatabase,
{
    fn run(&mut self, db: &mut T) -> Result<(), SystemError>
    where
        T: EntityComponentDatabase,
    {
        let destruction_test_components = db.get_entities_by_predicate(|entity_id| {
            db.entity_has_component::<DestructionTestInputComponent>(entity_id)
        });

        for entity_id in destruction_test_components {
            while let Some(input) = db
                .get_entity_component_mut::<PancursesInputBufferComponent>(entity_id)?
                .pop()
            {
                if input
                    == pancurses::Input::Character(
                        db.get_entity_component::<DestructionTestInputComponent>(entity_id)?
                            .get_input_char(),
                    )
                {
                    db.destroy_entity(entity_id)?;
                }
            }
        }

        Ok(())
    }
}
