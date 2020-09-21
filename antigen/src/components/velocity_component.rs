use crate::{primitive_types::Vector2I, entity_component_system::{ComponentDebugTrait, ComponentTrait}};

#[derive(Debug, Copy, Clone)]
pub struct VelocityComponent {
    data: Vector2I
}

impl VelocityComponent {
    pub fn new(data: Vector2I) -> Self {
        VelocityComponent { data }
    }

    pub fn get_velocity(&self) -> Vector2I {
        self.data
    }

    pub fn set_velocity(&mut self, velocity: Vector2I) -> &mut Self {
        self.data = velocity;
        self
    }
}

impl Default for VelocityComponent {
    fn default() -> Self {
        VelocityComponent::new(Vector2I::default())
    }
}

impl ComponentTrait for VelocityComponent {}

impl ComponentDebugTrait for VelocityComponent {
    fn get_name() -> String {
        "Velocity".into()
    }

    fn get_description() -> String {
        "2D cartesian velocity".into()
    }
}