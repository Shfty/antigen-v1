use crate::entity_component_system::ComponentDebugTrait;

#[derive(Debug, Default, Clone)]
pub struct DebugSystemList;

impl ComponentDebugTrait for DebugSystemList {
    fn get_name() -> String {
        "Debug System List".into()
    }

    fn get_description() -> String {
        "Tag component for debug system list".into()
    }
}
