use std::collections::HashMap;

use crate::entity_component_system::{ComponentDebugTrait, ComponentID, ComponentTrait};

#[derive(Debug, Clone)]
pub struct ComponentDebugComponent {
    labels: HashMap<ComponentID, String>,
    descriptions: HashMap<ComponentID, String>,
}

impl ComponentDebugComponent {
    pub fn new() -> Self {
        ComponentDebugComponent {
            labels: HashMap::new(),
            descriptions: HashMap::new(),
        }
    }

    pub fn register_component(
        &mut self,
        component_id: ComponentID,
        label: String,
        description: String,
    ) -> &mut Self {
        self.labels.insert(component_id, label);
        self.descriptions.insert(component_id, description);
        self
    }

    pub fn get_label(&self, component_id: &ComponentID) -> String {
        self.labels
            .get(component_id)
            .cloned()
            .unwrap_or(format!("Component {}", component_id))
    }
}

impl Default for ComponentDebugComponent {
    fn default() -> Self {
        ComponentDebugComponent::new()
    }
}

impl ComponentTrait for ComponentDebugComponent {}

impl ComponentDebugTrait for ComponentDebugComponent {
    fn get_name() -> String {
        "Component Debug".into()
    }

    fn get_description() -> String {
        "Container for component debug data".into()
    }
}
