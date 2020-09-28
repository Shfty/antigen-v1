use crate::primitive_types::Vector2I;

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
