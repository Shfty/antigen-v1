use antigen::entity_component_system::{ComponentDebugTrait, ComponentTrait};

#[derive(Debug, Copy, Clone)]
pub struct DestructionTestInputComponent {
    input_key: antigen::core::keyboard::Key,
}

impl DestructionTestInputComponent {
    pub fn new(input_key: antigen::core::keyboard::Key) -> Self {
        DestructionTestInputComponent { input_key }
    }

    pub fn get_input_key(&self) -> antigen::core::keyboard::Key {
        self.input_key
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
