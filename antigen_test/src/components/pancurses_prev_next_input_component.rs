use antigen::ecs::{ComponentMetadataTrait, ComponentTrait};
use pancurses::Input;

#[derive(Debug, Clone)]
pub struct PancursesPrevNextInputComponent {
    pub prev_input: Input,
    pub next_input: Input,
}

impl PancursesPrevNextInputComponent {
    pub fn new(prev_key: Input, next_key: Input) -> Self {
        PancursesPrevNextInputComponent { prev_input: prev_key, next_input: next_key }
    }
}

impl Default for PancursesPrevNextInputComponent {
    fn default() -> Self {
        PancursesPrevNextInputComponent::new(Input::Unknown(0), Input::Unknown(0))
    }
}

impl ComponentTrait for PancursesPrevNextInputComponent {}

impl ComponentMetadataTrait for PancursesPrevNextInputComponent {
    fn get_name() -> &'static str {
        "Pancurses UI Tab"
    }

    fn get_description() -> &'static str {
        "UI tab with prev and next keys"
    }
}
