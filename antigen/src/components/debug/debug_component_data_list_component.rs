use crate::entity_component_system::{ComponentDebugTrait, ComponentTrait};

#[derive(Debug, Default, Clone)]
pub struct DebugComponentDataListComponent;

impl ComponentTrait for DebugComponentDataListComponent {}

impl ComponentDebugTrait for DebugComponentDataListComponent {
    fn get_name() -> String {
        "Debug Entity Component List".into()
    }

    fn get_description() -> String {
        "Tag component for debug entity component list".into()
    }
}
