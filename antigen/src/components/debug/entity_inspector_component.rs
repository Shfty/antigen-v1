use crate::ecs::{ComponentDebugTrait, ComponentTrait};

#[derive(Debug, Default, Clone)]
pub struct EntityInspectorComponent;

impl ComponentTrait for EntityInspectorComponent {}

impl ComponentDebugTrait for EntityInspectorComponent {
    fn get_name() -> String {
        "Entity Inspector".into()
    }

    fn get_description() -> String {
        "Tag component for entity inspector".into()
    }
}
