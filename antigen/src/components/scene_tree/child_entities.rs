use std::ops::{Deref, DerefMut};

use crate::entity_component_system::EntityID;

#[derive(Debug, Clone)]
pub struct ChildEntitiesData(Vec<EntityID>);

impl Deref for ChildEntitiesData {
    type Target = Vec<EntityID>;

    fn deref(&self) -> &Vec<EntityID> {
        &self.0
    }
}

impl DerefMut for ChildEntitiesData {
    fn deref_mut(&mut self) -> &mut Vec<EntityID> {
        &mut self.0
    }
}

impl Default for ChildEntitiesData {
    fn default() -> Self {
        ChildEntitiesData(Vec::new())
    }
}
