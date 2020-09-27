use crate::{
    entity_component_system::{ComponentDebugTrait, ComponentTrait},
    primitive_types::Vector2I,
};

#[derive(Debug, Default, Copy, Clone)]
pub struct GlobalPosition(Vector2I);

impl From<Vector2I> for GlobalPosition {
    fn from(vec: Vector2I) -> Self {
        GlobalPosition(vec)
    }
}

impl Into<Vector2I> for GlobalPosition {
    fn into(self) -> Vector2I {
        self.0
    }
}

impl ComponentTrait for GlobalPosition {}

impl ComponentDebugTrait for GlobalPosition {
    fn get_name() -> String {
        "Global Position".into()
    }

    fn get_description() -> String {
        "Hierarchical 2D Cartesian Position".into()
    }
}
