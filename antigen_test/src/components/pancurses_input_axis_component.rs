use antigen::entity_component_system::{ComponentDebugTrait, ComponentTrait};

#[derive(Debug, Clone)]
pub struct InputAxisComponent {
    negative_input: antigen::Key,
    positive_input: antigen::Key,
}

impl InputAxisComponent {
    pub fn new(negative_input: antigen::Key, positive_input: antigen::Key) -> Self {
        InputAxisComponent {
            negative_input,
            positive_input,
        }
    }

    pub fn get_positive_input(&self) -> antigen::Key {
        self.positive_input
    }

    pub fn get_negative_input(&self) -> antigen::Key {
        self.negative_input
    }
}

impl ComponentTrait for InputAxisComponent {}

impl ComponentDebugTrait for InputAxisComponent {
    fn get_name() -> String {
        "Pancurses Input Axis".into()
    }

    fn get_description() -> String {
        "1D prev/next input map".into()
    }
}
