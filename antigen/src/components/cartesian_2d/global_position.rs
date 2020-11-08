use std::ops::{Deref, DerefMut};

use crate::primitive_types::Vector2I;

use super::Position;

#[derive(Debug, Default, Copy, Clone)]
pub struct GlobalPosition(Vector2I);

impl Deref for GlobalPosition {
    type Target = Vector2I;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GlobalPosition {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Position> for GlobalPosition {
    fn from(position: Position) -> Self {
        GlobalPosition(*position)
    }
}