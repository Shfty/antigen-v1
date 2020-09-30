use crate::primitive_types::Vector2I;
use std::{fmt::Debug, ops::Deref, ops::DerefMut};

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
