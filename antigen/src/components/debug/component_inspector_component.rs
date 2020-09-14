use crate::ecs::{ComponentDebugTrait, ComponentTrait};

#[derive(Default, Debug, Clone)]
pub struct ComponentInspectorComponent;

impl ComponentTrait for ComponentInspectorComponent {}

impl ComponentDebugTrait for ComponentInspectorComponent {
    fn get_name() -> String {
        "Component Inspector".into()
    }

    fn get_description() -> String {
        "Tag component for component inspector".into()
    }
}
