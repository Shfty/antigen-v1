use crate::entity_component_system::ComponentDebugTrait;

#[derive(Debug, Default, Clone)]
pub struct DebugEntityList;

impl ComponentDebugTrait for DebugEntityList {
    fn get_name() -> String {
        "Debug Entity List".into()
    }

    fn get_description() -> String {
        "Tag component for debug entity list".into()
    }
}
