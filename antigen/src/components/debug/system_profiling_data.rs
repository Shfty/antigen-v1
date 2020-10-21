use std::{collections::{BTreeMap}, fmt::Debug, time::Duration};

use crate::entity_component_system::SystemID;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct SystemProfilingData {
    durations: BTreeMap<SystemID, Duration>,
}

impl SystemProfilingData {
    pub fn new() -> Self {
        SystemProfilingData {
            durations: BTreeMap::new(),
        }
    }

    pub fn get_durations(&self) -> &BTreeMap<SystemID, Duration> {
        &self.durations
    }

    pub fn set_duration(&mut self, system_id: SystemID, duration: Duration) {
        self.durations.insert(system_id, duration);
    }
}
