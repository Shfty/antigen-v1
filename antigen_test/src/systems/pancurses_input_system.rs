use crate::components::pancurses_input_buffer_component::PancursesInputBufferComponent;
use antigen::ecs::{SystemTrait, ECS};

pub struct PancursesInputSystem {
    input_buffer: Vec<pancurses::Input>,
}

impl PancursesInputSystem {
    pub fn new() -> Self {
        PancursesInputSystem {
            input_buffer: Vec::new(),
        }
    }

    pub fn set_input_buffer(&mut self, buffer: &[pancurses::Input]) {
        self.input_buffer
            .resize(buffer.len(), pancurses::Input::Unknown(0));
        self.input_buffer.copy_from_slice(buffer)
    }
}

impl<T> SystemTrait<T> for PancursesInputSystem where T: ECS {
    fn run(&mut self, ecs: &mut T) -> Result<(), String> {
        let entities = ecs.get_entities_by_predicate(|entity_id| {
            ecs.entity_has_component::<PancursesInputBufferComponent>(entity_id)
        });

        for entity_id in entities {
            let pancurses_input_buffer_component =
                ecs.get_entity_component::<PancursesInputBufferComponent>(entity_id)?;

            pancurses_input_buffer_component.input_buffer = self.input_buffer.clone();
        }

        Ok(())
    }
}
