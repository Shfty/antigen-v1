use crate::ecs::ComponentTrait;

#[derive(Debug, Copy, Clone)]
pub struct PositionComponent {
    pub x: i64,
    pub y: i64,
}

impl PositionComponent {
    pub fn new(x: i64, y: i64) -> Self {
        PositionComponent { x, y }
    }
}

impl Default for PositionComponent {
    fn default() -> Self {
        PositionComponent::new(0, 0)
    }
}

impl ComponentTrait for PositionComponent {}
