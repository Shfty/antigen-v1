use crate::entity_component_system::{ComponentDebugTrait, ComponentTrait};

#[derive(Debug, Default, Clone)]
pub struct DebugSystemListComponent;

impl ComponentTrait for DebugSystemListComponent {}

impl ComponentDebugTrait for DebugSystemListComponent {
    fn get_name() -> String {
        "Debug System List".into()
    }

    fn get_description() -> String {
        "Tag component for debug system list".into()
    }
}
