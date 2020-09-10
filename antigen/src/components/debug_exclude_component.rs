use crate::ecs::{ComponentMetadataTrait, ComponentTrait};

#[derive(Debug, Copy, Clone)]
pub struct DebugExcludeComponent;

impl ComponentTrait for DebugExcludeComponent {}

impl ComponentMetadataTrait for DebugExcludeComponent {
    fn get_name() -> &'static str {
        "Debug Exclude"
    }

    fn get_description() -> &'static str {
        "Used to exclude an entity from debug visualization"
    }
}
