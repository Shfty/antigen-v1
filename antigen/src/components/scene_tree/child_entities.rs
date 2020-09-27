use std::borrow::{Borrow, BorrowMut};

use crate::entity_component_system::{ComponentDebugTrait, ComponentTrait, EntityID};

#[derive(Debug, Clone)]
pub struct ChildEntities(Vec<EntityID>);

impl Borrow<Vec<EntityID>> for ChildEntities {
    fn borrow(&self) -> &Vec<EntityID> {
        &self.0
    }
}

impl BorrowMut<Vec<EntityID>> for ChildEntities {
    fn borrow_mut(&mut self) -> &mut Vec<EntityID> {
        &mut self.0
    }
}

impl Default for ChildEntities {
    fn default() -> Self {
        ChildEntities(Vec::new())
    }
}

impl ComponentTrait for ChildEntities {}

impl ComponentDebugTrait for ChildEntities {
    fn get_name() -> String {
        "Child Entities".into()
    }

    fn get_description() -> String {
        "Holds child entity IDs".into()
    }
}
