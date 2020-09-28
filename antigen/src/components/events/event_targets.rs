use std::fmt::Debug;

use crate::entity_component_system::EntityID;

/// Holds a list of entities to be used as targets for emitted events
#[derive(Debug, Clone)]
pub struct EventTargets(Vec<EntityID>);

impl EventTargets {
    pub fn new(targets: Vec<EntityID>) -> Self {
        EventTargets(targets)
    }
}

impl AsRef<Vec<EntityID>> for EventTargets {
    fn as_ref(&self) -> &Vec<EntityID> {
        &self.0
    }
}

impl AsMut<Vec<EntityID>> for EventTargets {
    fn as_mut(&mut self) -> &mut Vec<EntityID> {
        &mut self.0
    }
}

impl Default for EventTargets {
    fn default() -> Self {
        EventTargets::new(Vec::new())
    }
}
