use crate::{
    entity_component_system::{ComponentDebugTrait, ComponentTrait},
    primitive_types::Vector2I,
};

#[derive(Debug, Default, Copy, Clone)]
pub struct Velocity(Vector2I);

impl From<Vector2I> for Velocity {
    fn from(vec: Vector2I) -> Self {
        Velocity(vec)
    }
}

impl Into<Vector2I> for Velocity {
    fn into(self) -> Vector2I {
        self.0
    }
}

impl ComponentTrait for Velocity {}

impl ComponentDebugTrait for Velocity {
    fn get_name() -> String {
        "Velocity".into()
    }

    fn get_description() -> String {
        "2D cartesian velocity".into()
    }
}
