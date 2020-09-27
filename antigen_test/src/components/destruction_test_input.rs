use antigen::{
    core::keyboard::Key,
    entity_component_system::{ComponentDebugTrait, ComponentTrait},
};

#[derive(Debug, Copy, Clone)]
pub struct DestructionTestInput(pub Key);

impl From<Key> for DestructionTestInput {
    fn from(key: Key) -> Self {
        DestructionTestInput(key)
    }
}

impl Into<Key> for DestructionTestInput {
    fn into(self) -> Key {
        self.0
    }
}

impl ComponentTrait for DestructionTestInput {}

impl ComponentDebugTrait for DestructionTestInput {
    fn get_name() -> String {
        "Destruction Test Input".into()
    }

    fn get_description() -> String {
        "Component to tag a component for destruction testing".into()
    }
}
