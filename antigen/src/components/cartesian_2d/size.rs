use crate::{
    entity_component_system::{ComponentDebugTrait, ComponentTrait},
    primitive_types::Vector2I,
};

#[derive(Debug, Default, Copy, Clone)]
pub struct Size(Vector2I);

impl From<Vector2I> for Size {
    fn from(vec: Vector2I) -> Self {
        Size(vec)
    }
}

impl Into<Vector2I> for Size {
    fn into(self) -> Vector2I {
        self.0
    }
}

impl ComponentTrait for Size {}

impl ComponentDebugTrait for Size {
    fn get_name() -> String {
        "Size".into()
    }

    fn get_description() -> String {
        "2D cartesian size".into()
    }
}
