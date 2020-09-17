use crate::entity_component_system::{ComponentDebugTrait, ComponentTrait};

#[derive(Debug, Default, Clone)]
pub struct DebugSceneTreeComponent;

impl ComponentTrait for DebugSceneTreeComponent {}

impl ComponentDebugTrait for DebugSceneTreeComponent {
    fn get_name() -> String {
        "Debug Scene Tree".into()
    }

    fn get_description() -> String {
        "Tag component for debug scene tree".into()
    }
}
