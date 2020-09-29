use crate::entity_component_system::EntityID;

#[derive(Debug, Clone)]
pub struct ChildEntitiesData(Vec<EntityID>);

impl AsRef<Vec<EntityID>> for ChildEntitiesData {
    fn as_ref(&self) -> &Vec<EntityID> {
        &self.0
    }
}

impl AsMut<Vec<EntityID>> for ChildEntitiesData {
    fn as_mut(&mut self) -> &mut Vec<EntityID> {
        &mut self.0
    }
}

impl Default for ChildEntitiesData {
    fn default() -> Self {
        ChildEntitiesData(Vec::new())
    }
}
