use std::{collections::HashMap, fmt::Debug, time::Duration};

use crate::entity_component_system::{ComponentDebugTrait, ComponentTrait, SystemID};

#[derive(Clone)]
pub struct SystemDebugInfo {
    labels: HashMap<SystemID, String>,
    durations: HashMap<SystemID, Duration>,
}

impl SystemDebugInfo {
    pub fn new() -> Self {
        SystemDebugInfo {
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

impl Debug for SystemDebugInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SystemDebugComponent").finish()
    }
}

impl Default for SystemDebugInfo {
    fn default() -> Self {
        SystemDebugInfo::new()
    }
}

impl ComponentTrait for SystemDebugInfo {}

impl ComponentDebugTrait for SystemDebugInfo {
    fn get_name() -> String {
        "System Debug".into()
    }

    fn get_description() -> String {
        "Container for system debug data".into()
    }
}
