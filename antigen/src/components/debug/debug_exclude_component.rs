use crate::ecs::{ComponentDebugTrait, ComponentTrait};

#[derive(Debug, Default, Clone)]
pub struct DebugExcludeComponent;

impl ComponentTrait for DebugExcludeComponent {}

impl ComponentDebugTrait for DebugExcludeComponent {
    fn get_name() -> String {
        "Debug Exclude".into()
    }

    fn get_description() -> String {
        "Tag component for excluding an entity from debug visualization".into()
    }
}
