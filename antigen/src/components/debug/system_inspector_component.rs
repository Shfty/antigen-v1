use crate::entity_component_system::{ComponentDebugTrait, ComponentTrait};

#[derive(Debug, Default, Clone)]
pub struct SystemInspectorComponent;

impl ComponentTrait for SystemInspectorComponent {}

impl ComponentDebugTrait for SystemInspectorComponent {
    fn get_name() -> String {
        "System Inspector".into()
    }

    fn get_description() -> String {
        "Tag component for system inspector".into()
    }
}
