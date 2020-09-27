use crate::entity_component_system::ComponentTrait;

#[derive(Debug, Default, Copy, Clone)]
pub struct Timer {
    duration: i64,
    running: bool,
    time_remaining: i64,
}

impl Timer {
    pub fn new(duration: i64) -> Self {
        Timer {
            duration,
            running: false,
            time_remaining: 0,
        }
    }

    pub fn get_duration(&self) -> i64 {
        self.duration
    }

    pub fn get_running(&self) -> bool {
        self.running
    }

    pub fn get_time_remaining(&self) -> i64 {
        self.time_remaining
    }

    pub fn start(&mut self) -> &mut Self {
        self.time_remaining = self.duration;
        self.running = true;
        self
    }
}

impl ComponentTrait for Timer {}
