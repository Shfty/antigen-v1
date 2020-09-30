use std::{fmt::Debug, ops::Deref, ops::DerefMut};

use crate::entity_component_system::EntityID;

/// Holds a list of entities to be used as targets for emitted events
#[derive(Debug, Clone)]
pub struct EventTargets(Vec<EntityID>);

impl EventTargets {
    pub fn new(targets: Vec<EntityID>) -> Self {
        EventTargets(targets)
    }
}

impl Deref for EventTargets {
    type Target = Vec<EntityID>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EventTargets {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for EventTargets {
    fn default() -> Self {
        EventTargets::new(Vec::new())
    }
}
