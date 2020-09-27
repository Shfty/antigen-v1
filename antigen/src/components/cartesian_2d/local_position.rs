use crate::{entity_component_system::ComponentDebugTrait, primitive_types::Vector2I};

#[derive(Debug, Default, Copy, Clone)]
pub struct LocalPosition(Vector2I);

impl From<Vector2I> for LocalPosition {
    fn from(vec: Vector2I) -> Self {
        LocalPosition(vec)
    }
}

impl Into<Vector2I> for LocalPosition {
    fn into(self) -> Vector2I {
        self.0
    }
}

impl ComponentDebugTrait for LocalPosition {
    fn get_name() -> String {
        "Local Mouse Position".into()
    }

    fn get_description() -> String {
        "Local-space mouse position".into()
    }
}
