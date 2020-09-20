use std::{collections::HashMap, time::Duration};

use crate::entity_component_system::{ComponentDebugTrait, ComponentTrait, SystemID};

#[derive(Debug, Clone)]
pub struct SystemDebugComponent {
    labels: HashMap<SystemID, String>,
    durations: HashMap<SystemID, Duration>,
}

impl SystemDebugComponent {
    pub fn new() -> Self {
        SystemDebugComponent {
            labels: HashMap::new(),
            durations: HashMap::new(),
        }
    }

    pub fn register_system(&mut self, system_id: SystemID, label: &str) -> &mut Self {
        self.labels.insert(system_id, label.into());
        self
    }

    pub fn get_labels(&self) -> &HashMap<SystemID, String> {
        &self.labels
    }

    pub fn get_durations(&self) -> &HashMap<SystemID, Duration> {
        &self.durations
    }

    pub fn set_duration(&mut self, system_id: SystemID, duration: Duration) {
        self.durations.insert(system_id, duration);
    }
}

impl Default for SystemDebugComponent {
    fn default() -> Self {
        SystemDebugComponent::new()
    }
}

impl ComponentTrait for SystemDebugComponent {}

impl ComponentDebugTrait for SystemDebugComponent {
    fn get_name() -> String {
        "System Debug".into()
    }

    fn get_description() -> String {
        "Container for system debug data".into()
    }
}
