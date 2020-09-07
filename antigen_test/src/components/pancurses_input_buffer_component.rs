use antigen::ecs::{ComponentMetadataTrait, ComponentTrait};
use pancurses::Input;

#[derive(Debug, Clone)]
pub struct PancursesInputBufferComponent {
    pub input_buffer: Vec<Input>,
}

impl PancursesInputBufferComponent {
    pub fn new() -> Self {
        PancursesInputBufferComponent {
            input_buffer: Vec::new()
        }
    }
}

impl Default for PancursesInputBufferComponent {
    fn default() -> Self {
        PancursesInputBufferComponent::new()
    }
}

impl ComponentTrait for PancursesInputBufferComponent {}

impl ComponentMetadataTrait for PancursesInputBufferComponent {
    fn get_name() -> &'static str {
        "Pancurses Input Buffer"
    }

    fn get_description() -> &'static str {
        "Buffer for receiving input events"
    }
}
