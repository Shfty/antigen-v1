use crate::{
    entity_component_system::{ComponentDebugTrait, ComponentTrait},
    primitive_types::Vector2I,
};
use std::fmt::Debug;

#[derive(Debug, Copy, Clone)]
pub struct PositionComponent {
    data: Vector2I,
}

impl PositionComponent {
    pub fn new(data: Vector2I) -> Self {
        PositionComponent { data }
    }

    pub fn get_position(&self) -> Vector2I {
        self.data
    }

    pub fn set_position(&mut self, data: Vector2I) -> &mut Self {
        self.data = data;
        self
    }
}

impl Default for PositionComponent {
    fn default() -> Self {
        PositionComponent::new(Vector2I::default())
    }
}

impl ComponentTrait for PositionComponent {}

impl ComponentDebugTrait for PositionComponent {
    fn get_name() -> String {
        "Position".into()
    }

    fn get_description() -> String {
        "2D Cartesian Position".into()
    }
}
