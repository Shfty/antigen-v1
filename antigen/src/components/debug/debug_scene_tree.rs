use crate::entity_component_system::ComponentDebugTrait;

#[derive(Debug, Default, Clone)]
pub struct DebugSceneTree;

impl ComponentDebugTrait for DebugSceneTree {
    fn get_name() -> String {
        "Debug Scene Tree".into()
    }

    fn get_description() -> String {
        "Tag component for debug scene tree".into()
    }
}
