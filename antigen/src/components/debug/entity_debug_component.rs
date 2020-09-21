use std::{collections::HashMap, fmt::Debug};

use crate::entity_component_system::{ComponentDebugTrait, ComponentTrait, EntityID};

#[derive(Clone)]
pub struct EntityDebugComponent {
    labels: HashMap<EntityID, String>,
}

impl Debug for EntityDebugComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EntityDebugComponent").finish()
    }
}

impl EntityDebugComponent {
    pub fn new() -> Self {
        EntityDebugComponent {
            labels: HashMap::new(),
        }
    }

    pub fn register_entity(&mut self, entity_id: EntityID, label: String) -> &mut Self {
        self.labels.insert(entity_id, label);
        self
    }

    pub fn get_label(&self, entity_id: &EntityID) -> String {
        self.labels
            .get(entity_id)
            .cloned()
            .unwrap_or(format!("Entity {}", entity_id))
    }
}

impl Default for EntityDebugComponent {
    fn default() -> Self {
        EntityDebugComponent::new()
    }
}

impl ComponentTrait for EntityDebugComponent {}

impl ComponentDebugTrait for EntityDebugComponent {
    fn get_name() -> String {
        "Entity Debug".into()
    }

    fn get_description() -> String {
        "Container for entity debug data".into()
    }

    fn is_debug_exclude() -> bool {
        true
    }
}
