use crate::entity_component_system::{ComponentDebugTrait, ComponentTrait};

#[derive(Debug, Default, Clone)]
pub struct DebugEntityListComponent;

impl ComponentTrait for DebugEntityListComponent {}

impl ComponentDebugTrait for DebugEntityListComponent {
    fn get_name() -> String {
        "Debug Entity List".into()
    }

    fn get_description() -> String {
        "Tag component for debug entity list".into()
    }
}
