use antigen::ecs::{ComponentMetadataTrait, ComponentTrait};
use pancurses::Input;

#[derive(Debug, Clone)]
pub struct PancursesInputAxisComponent {
    pub negative_input: Input,
    pub positive_input: Input,
}

impl PancursesInputAxisComponent {
    pub fn new(negative_input: Input, positive_input: Input) -> Self {
        PancursesInputAxisComponent { negative_input, positive_input }
    }
}

impl Default for PancursesInputAxisComponent {
    fn default() -> Self {
        PancursesInputAxisComponent::new(Input::Unknown(0), Input::Unknown(0))
    }
}

impl ComponentTrait for PancursesInputAxisComponent {}

impl ComponentMetadataTrait for PancursesInputAxisComponent {
    fn get_name() -> &'static str {
        "Pancurses Input Axis"
    }

    fn get_description() -> &'static str {
        "1D prev/next input map"
    }
}
