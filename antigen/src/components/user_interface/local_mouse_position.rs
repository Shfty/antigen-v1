use crate::primitive_types::Vector2I;

#[derive(Debug, Default, Copy, Clone)]
pub struct LocalMousePosition(Vector2I);

impl From<Vector2I> for LocalMousePosition {
    fn from(vec: Vector2I) -> Self {
        LocalMousePosition(vec)
    }
}

impl Into<Vector2I> for LocalMousePosition {
    fn into(self) -> Vector2I {
        self.0
    }
}
