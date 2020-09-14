use antigen::ecs::{ComponentDebugTrait, ComponentTrait};
use pancurses::Input;

#[derive(Debug, Clone)]
pub struct PancursesInputAxisComponent {
    negative_input: Input,
    positive_input: Input,
}

impl PancursesInputAxisComponent {
    pub fn new(negative_input: Input, positive_input: Input) -> Self {
        PancursesInputAxisComponent {
            negative_input,
            positive_input,
        }
    }

    pub fn get_positive_input(&self) -> Input {
        self.positive_input
    }

    pub fn get_negative_input(&self) -> Input {
        self.negative_input
    }
}

impl Default for PancursesInputAxisComponent {
    fn default() -> Self {
        PancursesInputAxisComponent::new(Input::Unknown(0), Input::Unknown(0))
    }
}

impl ComponentTrait for PancursesInputAxisComponent {}

impl ComponentDebugTrait for PancursesInputAxisComponent {
    fn get_name() -> String {
        "Pancurses Input Axis".into()
    }

    fn get_description() -> String {
        "1D prev/next input map".into()
    }
}
