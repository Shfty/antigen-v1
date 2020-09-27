use crate::entity_component_system::{ComponentDebugTrait, EntityID};

#[derive(Debug, Clone)]
pub struct ChildEntities(Vec<EntityID>);

impl AsRef<Vec<EntityID>> for ChildEntities {
    fn as_ref(&self) -> &Vec<EntityID> {
        &self.0
    }
}

impl AsMut<Vec<EntityID>> for ChildEntities {
    fn as_mut(&mut self) -> &mut Vec<EntityID> {
        &mut self.0
    }
}

impl Default for ChildEntities {
    fn default() -> Self {
        ChildEntities(Vec::new())
    }
}

impl ComponentDebugTrait for ChildEntities {
    fn get_name() -> String {
        "Child Entities".into()
    }

    fn get_description() -> String {
        "Holds child entity IDs".into()
    }
}
