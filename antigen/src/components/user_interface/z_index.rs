use std::ops::{Deref, DerefMut};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ZIndex(pub i64);

impl Deref for ZIndex {
    type Target = i64;

    fn deref(&self) -> &i64 {
        &self.0
    }
}

impl DerefMut for ZIndex {
    fn deref_mut(&mut self) -> &mut i64 {
        &mut self.0
    }
}
