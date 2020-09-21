use crate::{
    entity_component_system::{ComponentDebugTrait, ComponentTrait},
    primitive_types::Vector2I,
};

#[derive(Debug, Copy, Clone)]
pub struct GlobalPositionComponent {
    data: Vector2I,
}

impl GlobalPositionComponent {
    pub fn new(data: Vector2I) -> Self {
        GlobalPositionComponent { data }
    }

    pub fn get_global_position(&self) -> Vector2I {
        self.data
    }

    pub fn set_global_position(&mut self, global_position: Vector2I) -> &mut Self {
        self.data = global_position;
        self
    }
}

impl Default for GlobalPositionComponent {
    fn default() -> Self {
        GlobalPositionComponent::new(Vector2I::default())
    }
}

impl ComponentTrait for GlobalPositionComponent {}

impl ComponentDebugTrait for GlobalPositionComponent {
    fn get_name() -> String {
        "Global Position".into()
    }

    fn get_description() -> String {
        "Hierarchical 2D Cartesian Position".into()
    }
}
