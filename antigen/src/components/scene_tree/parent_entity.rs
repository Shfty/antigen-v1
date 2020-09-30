use std::ops::{Deref, DerefMut};

use crate::entity_component_system::EntityID;

#[derive(Debug, Default, Copy, Clone)]
pub struct ParentEntity(pub EntityID);

impl Deref for ParentEntity {
    type Target = EntityID;

    fn deref(&self) -> &EntityID {
        &self.0
    }
}

impl DerefMut for ParentEntity {
    fn deref_mut(&mut self) -> &mut EntityID {
        &mut self.0
    }
}
