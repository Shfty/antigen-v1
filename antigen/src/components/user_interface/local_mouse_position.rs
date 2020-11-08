use std::ops::Deref;

use crate::primitive_types::Vector2I;

#[derive(Debug, Default, Copy, Clone)]
pub struct LocalMousePositionData(Vector2I);

impl From<Vector2I> for LocalMousePositionData {
    fn from(vec: Vector2I) -> Self {
        LocalMousePositionData(vec)
    }
}

impl Into<Vector2I> for LocalMousePositionData {
    fn into(self) -> Vector2I {
        self.0
    }
}

impl Deref for LocalMousePositionData {
    type Target = Vector2I;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}