use crate::{
    entity_component_system::{ComponentDebugTrait, ComponentTrait},
    primitive_types::Vector2I,
};

#[derive(Debug, Copy, Clone)]
pub struct SizeComponent {
    data: Vector2I,
}

impl SizeComponent {
    pub fn new(data: Vector2I) -> Self {
        SizeComponent { data }
    }

    pub fn get_size(&self) -> Vector2I {
        self.data
    }

    pub fn set_size(&mut self, size: Vector2I) -> &mut Self {
        self.data = size;
        self
    }
}

impl Default for SizeComponent {
    fn default() -> Self {
        SizeComponent::new(Vector2I::default())
    }
}

impl ComponentTrait for SizeComponent {}

impl ComponentDebugTrait for SizeComponent {
    fn get_name() -> String {
        "Size".into()
    }

    fn get_description() -> String {
        "2D cartesian size".into()
    }
}
