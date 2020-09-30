use std::ops::{Deref, DerefMut};

use crate::primitive_types::Vector2I;

#[derive(Debug, Default, Copy, Clone)]
pub struct Velocity(Vector2I);

impl Deref for Velocity {
    type Target = Vector2I;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Velocity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
