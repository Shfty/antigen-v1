use crate::entity_component_system::{ComponentDebugTrait, ComponentTrait};

#[derive(Default, Debug, Clone)]
pub struct ComponentInspector;

impl ComponentTrait for ComponentInspector {}

impl ComponentDebugTrait for ComponentInspector {
    fn get_name() -> String {
        "Component Inspector".into()
    }

    fn get_description() -> String {
        "Tag component for component inspector".into()
    }
}
