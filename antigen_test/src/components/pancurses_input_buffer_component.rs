use antigen::entity_component_system::{ComponentDebugTrait, ComponentTrait};
use pancurses::Input;

#[derive(Debug, Clone)]
pub struct PancursesInputBufferComponent {
    input_buffer: Vec<Input>,
}

impl PancursesInputBufferComponent {
    pub fn new() -> Self {
        PancursesInputBufferComponent {
            input_buffer: Vec::new()
        }
    }

    pub fn push(&mut self, input: Input) -> &mut Self {
        self.input_buffer.push(input);
        self
    }

    pub fn pop(&mut self) -> Option<Input> {
        self.input_buffer.pop()
    }

    pub fn clear(&mut self) -> &mut Self {
        self.input_buffer.clear();
        self
    }
}

impl Default for PancursesInputBufferComponent {
    fn default() -> Self {
        PancursesInputBufferComponent::new()
    }
}

impl ComponentTrait for PancursesInputBufferComponent {}

impl ComponentDebugTrait for PancursesInputBufferComponent {
    fn get_name() -> String {
        "Pancurses Input Buffer".into()
    }

    fn get_description() -> String {
        "Buffer for receiving input events".into()
    }
}
