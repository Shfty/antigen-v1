use crate::ecs::ComponentTrait;

#[derive(Debug, Copy, Clone)]
pub struct TickMoveComponent {
    pub interval: i64,
    pub timer: i64,
}

impl TickMoveComponent {
    pub fn new(interval: i64) -> Self {
        TickMoveComponent { interval, timer: 0 }
    }
}

impl Default for TickMoveComponent {
    fn default() -> Self {
        TickMoveComponent::new(1)
    }
}

impl ComponentTrait for TickMoveComponent {}
