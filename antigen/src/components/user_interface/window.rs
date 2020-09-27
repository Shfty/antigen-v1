use crate::entity_component_system::{ComponentDebugTrait, ComponentTrait};

#[derive(Debug, Default, Copy, Clone)]
pub struct Window;

impl ComponentTrait for Window {}

impl ComponentDebugTrait for Window {
    fn get_name() -> String {
        "Window".into()
    }

    fn get_description() -> String {
        "Represents a unique window".into()
    }
}
