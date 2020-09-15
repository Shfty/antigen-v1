use antigen::entity_component_system::{ComponentDebugTrait, ComponentTrait};

#[derive(Debug, Default, Copy, Clone)]
pub struct DestructionTestInputComponent {
    input_char: char,
}

impl DestructionTestInputComponent {
    pub fn new(input_char: char) -> Self {
        DestructionTestInputComponent { input_char }
    }

    pub fn get_input_char(&self) -> char {
        self.input_char
    }
}

impl ComponentTrait for DestructionTestInputComponent {}

impl ComponentDebugTrait for DestructionTestInputComponent {
    fn get_name() -> String {
        "Destruction Test Input".into()
    }

    fn get_description() -> String {
        "Component to tag a component for destruction testing".into()
    }
}
