use crate::ecs::ComponentTrait;

#[derive(Debug, Copy, Clone)]
pub struct TimerComponent {
    duration: i64,
    running: bool,
    time_remaining: i64,
}

impl TimerComponent {
    pub fn new(duration: i64) -> Self {
        TimerComponent {
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

impl Default for TimerComponent {
    fn default() -> Self {
        TimerComponent::new(1)
    }
}

impl ComponentTrait for TimerComponent {}
