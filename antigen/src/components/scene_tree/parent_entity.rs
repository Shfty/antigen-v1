use crate::entity_component_system::{ComponentDebugTrait, EntityID};

#[derive(Debug, Default, Copy, Clone)]
pub struct ParentEntity(pub EntityID);

impl From<EntityID> for ParentEntity {
    fn from(id: EntityID) -> Self {
        ParentEntity(id)
    }
}

impl Into<EntityID> for ParentEntity {
    fn into(self) -> EntityID {
        self.0
    }
}

impl ComponentDebugTrait for ParentEntity {
    fn get_name() -> String {
        "Parent".into()
    }

    fn get_description() -> String {
        "Parent Entity".into()
    }
}
