use crate::ecs::ComponentTrait;

#[derive(Debug, Copy, Clone)]
pub struct VelocityComponent {
    pub x: i64,
    pub y: i64,
}

impl VelocityComponent {
    pub fn new(x: i64, y: i64) -> Self {
        VelocityComponent { x, y }
    }
}

impl Default for VelocityComponent {
    fn default() -> Self {
        VelocityComponent::new(0, 0)
    }
}

impl ComponentTrait for VelocityComponent {}
