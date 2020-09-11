use crate::ecs::{ComponentMetadataTrait, ComponentTrait};

#[derive(Debug, Clone)]
pub struct EntityInspectorComponent;

impl ComponentTrait for EntityInspectorComponent {}

impl ComponentMetadataTrait for EntityInspectorComponent {
    fn get_name() -> &'static str {
        "Entity Inspector"
    }

    fn get_description() -> &'static str {
        "Tag component for entity inspector"
    }
}
