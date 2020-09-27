use crate::entity_component_system::{ComponentDebugTrait, ComponentTrait};

#[derive(Debug, Default, Clone)]
pub struct DebugSceneTree;

impl ComponentTrait for DebugSceneTree {}

impl ComponentDebugTrait for DebugSceneTree {
    fn get_name() -> String {
        "Debug Scene Tree".into()
    }

    fn get_description() -> String {
        "Tag component for debug scene tree".into()
    }
}
