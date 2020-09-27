use crate::entity_component_system::{ComponentDebugTrait, ComponentTrait};

#[derive(Debug, Default, Clone)]
pub struct DebugComponentList;

impl ComponentTrait for DebugComponentList {}

impl ComponentDebugTrait for DebugComponentList {
    fn get_name() -> String {
        "Debug Component List".into()
    }

    fn get_description() -> String {
        "Tag component for debug component list".into()
    }
}
