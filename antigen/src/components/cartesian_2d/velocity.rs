use crate::primitive_types::Vector2I;

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
