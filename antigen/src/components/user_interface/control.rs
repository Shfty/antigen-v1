use crate::entity_component_system::{ComponentDebugTrait, ComponentTrait};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Control;

impl ComponentTrait for Control {}

impl ComponentDebugTrait for Control {
    fn get_name() -> String {
        "Control".into()
    }

    fn get_description() -> String {
        "Tags an entity for rendering by UI systems".into()
    }
}
