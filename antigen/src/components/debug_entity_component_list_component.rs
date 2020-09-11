use crate::ecs::{ComponentMetadataTrait, ComponentTrait};

#[derive(Debug, Clone)]
pub struct DebugEntityComponentListComponent;

impl ComponentTrait for DebugEntityComponentListComponent {}

impl ComponentMetadataTrait for DebugEntityComponentListComponent {
    fn get_name() -> &'static str {
        "Debug Entity Component List"
    }

    fn get_description() -> &'static str {
        "Tag component for debug entity component list"
    }
}
