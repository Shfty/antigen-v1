use std::ops::{Deref, DerefMut};

use antigen::core::keyboard::Key;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct DestructionTestInputData(pub Key);

impl Deref for DestructionTestInputData {
    type Target = Key;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DestructionTestInputData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
