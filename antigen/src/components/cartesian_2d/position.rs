use crate::primitive_types::Vector2I;
use std::{
    fmt::Debug,
    ops::Deref,
    ops::{Add, DerefMut},
};

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Position(pub Vector2I);

impl Deref for Position {
    type Target = Vector2I;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Position {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vector2I> for Position {
    fn from(vector: Vector2I) -> Self {
        Position(vector)
    }
}

impl Into<Vector2I> for Position {
    fn into(self) -> Vector2I {
        self.0
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2I::add(*self, *rhs).into()
    }
}
