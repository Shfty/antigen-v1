use crate::primitive_types::Vector2I;
use std::fmt::Debug;

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Position(pub Vector2I);

impl From<Vector2I> for Position {
    fn from(vec: Vector2I) -> Self {
        Position(vec)
    }
}

impl Into<Vector2I> for Position {
    fn into(self) -> Vector2I {
        self.0
    }
}
