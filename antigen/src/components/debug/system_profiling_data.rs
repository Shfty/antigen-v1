use std::{fmt::Debug, time::Duration};

use store::TypeKey;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct SystemProfilingData {
    durations: Vec<(TypeKey, Duration)>,
}

impl SystemProfilingData {
    pub fn new() -> Self {
        SystemProfilingData {
            durations: Vec::new(),
        }
    }

    pub fn get_durations(&self) -> &Vec<(TypeKey, Duration)> {
        &self.durations
    }

    pub fn set_duration(&mut self, system_id: TypeKey, duration: Duration) {
        self.durations.push((system_id, duration));
    }

    pub fn clear_durations(&mut self) {
        self.durations.clear()
    }
}
