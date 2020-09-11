use antigen::ecs::{ComponentMetadataTrait, ComponentTrait};

#[derive(Debug, Default, Copy, Clone)]
pub struct DestructionTestInputComponent {
    pub input_char: char,
}

impl DestructionTestInputComponent {
    pub fn new(input_char: char) -> Self {
        DestructionTestInputComponent { input_char }
    }
}

impl ComponentTrait for DestructionTestInputComponent {}

impl ComponentMetadataTrait for DestructionTestInputComponent {
    fn get_name() -> &'static str {
        "Destruction Test Input"
    }

    fn get_description() -> &'static str {
        "Component to tag a component for destruction testing"
    }
}
