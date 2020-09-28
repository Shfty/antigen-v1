use crate::entity_component_system::EntityID;

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
