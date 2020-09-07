use crate::ecs::{ComponentMetadataTrait, ComponentTrait, EntityID};

#[derive(Debug, Copy, Clone)]
pub struct ParentEntityComponent {
    pub parent_id: EntityID,
}

impl ParentEntityComponent {
    pub fn new(parent_id: EntityID) -> Self {
        ParentEntityComponent { parent_id }
    }
}

impl ComponentTrait for ParentEntityComponent {}

impl ComponentMetadataTrait for ParentEntityComponent {
    fn get_name() -> &'static str {
        "Parent"
    }

    fn get_description() -> &'static str {
        "Parent Entity"
    }
}
