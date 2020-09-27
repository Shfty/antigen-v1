use crate::entity_component_system::{ComponentDebugTrait, ComponentTrait};

#[derive(Debug, Default, Clone)]
pub struct SystemInspector;

impl ComponentTrait for SystemInspector {}

impl ComponentDebugTrait for SystemInspector {
    fn get_name() -> String {
        "System Inspector".into()
    }

    fn get_description() -> String {
        "Tag component for system inspector".into()
    }
}
