use crate::entity_component_system::{ComponentDebugTrait, ComponentTrait};

#[derive(Debug, Default, Clone)]
pub struct DebugComponentListComponent;

impl ComponentTrait for DebugComponentListComponent {}

impl ComponentDebugTrait for DebugComponentListComponent {
    fn get_name() -> String {
        "Debug Component List".into()
    }

    fn get_description() -> String {
        "Tag component for debug component list".into()
    }
}
