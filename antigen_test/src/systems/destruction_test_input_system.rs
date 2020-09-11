use antigen::{
    ecs::EntityComponentDatabaseDebug,
    ecs::SystemEvent,
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
    T: EntityComponentDatabase + EntityComponentDatabaseDebug,
{
    fn run(&mut self, db: &mut T) -> Result<SystemEvent, String>
    where
        T: EntityComponentDatabase + EntityComponentDatabaseDebug,
    {
        let destruction_test_components = db.get_entities_by_predicate(|entity_id| {
            db.entity_has_component::<DestructionTestInputComponent>(entity_id)
        });

        for entity_id in destruction_test_components {
            let input_buffer_component =
                db.get_entity_component::<PancursesInputBufferComponent>(entity_id)?;
                
            let mut input_buffer = input_buffer_component.input_buffer.clone();

            while let Some(input) = input_buffer.pop() {
                if input == pancurses::Input::Character(' ') {
                    db.destroy_entity(entity_id)?;
                }
            }
        }

        Ok(SystemEvent::None)
    }
}
