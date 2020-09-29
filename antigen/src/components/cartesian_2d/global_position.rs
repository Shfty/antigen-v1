use crate::primitive_types::Vector2I;

#[derive(Debug, Default, Copy, Clone)]
pub struct GlobalPositionData(Vector2I);

impl From<Vector2I> for GlobalPositionData {
    fn from(vec: Vector2I) -> Self {
        GlobalPositionData(vec)
    }
}

impl Into<Vector2I> for GlobalPositionData {
    fn into(self) -> Vector2I {
        self.0
    }
}
