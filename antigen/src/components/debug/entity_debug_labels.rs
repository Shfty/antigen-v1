use std::{collections::HashMap, fmt::Debug};

use crate::entity_component_system::{ComponentDebugTrait, ComponentTrait, EntityID};

#[derive(Clone)]
pub struct EntityDebugLabels {
    labels: HashMap<EntityID, String>,
}

impl Debug for EntityDebugLabels {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EntityDebugLabels").finish()
    }
}

impl EntityDebugLabels {
    pub fn new() -> Self {
        EntityDebugLabels {
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

impl Default for EntityDebugLabels {
    fn default() -> Self {
        EntityDebugLabels::new()
    }
}

impl ComponentTrait for EntityDebugLabels {}

impl ComponentDebugTrait for EntityDebugLabels {
    fn get_name() -> String {
        "Entity Debug Labels".into()
    }

    fn get_description() -> String {
        "Container for entity debug labels".into()
    }

    fn is_debug_exclude() -> bool {
        true
    }
}
