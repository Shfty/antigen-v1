use crate::entity_component_system::ComponentDebugTrait;

#[derive(Debug, Default, Clone)]
pub struct DebugExclude;

impl ComponentDebugTrait for DebugExclude {
    fn get_name() -> String {
        "Debug Exclude".into()
    }

    fn get_description() -> String {
        "Tag component for excluding an entity from debug visualization".into()
    }
}
