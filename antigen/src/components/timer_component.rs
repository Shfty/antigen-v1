use crate::ecs::ComponentTrait;

#[derive(Debug, Copy, Clone)]
pub struct TimerComponent {
    pub duration: i64,
    pub running: bool,
    pub timer: i64,
}

impl TimerComponent {
    pub fn new(duration: i64, running: bool) -> Self {
        TimerComponent {
            duration,
            running,
            timer: duration,
        }
    }
}

impl Default for TimerComponent {
    fn default() -> Self {
        TimerComponent::new(1, false)
    }
}

impl ComponentTrait for TimerComponent {}
