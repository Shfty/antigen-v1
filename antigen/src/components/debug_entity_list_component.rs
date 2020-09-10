use crate::ecs::{ComponentMetadataTrait, ComponentTrait};

#[derive(Debug, Clone)]
pub struct DebugEntityListComponent;

impl ComponentTrait for DebugEntityListComponent {}

impl ComponentMetadataTrait for DebugEntityListComponent {
    fn get_name() -> &'static str {
        "Debug Entity List"
    }

    fn get_description() -> &'static str {
        "Tag component for debug entity list"
    }
}
