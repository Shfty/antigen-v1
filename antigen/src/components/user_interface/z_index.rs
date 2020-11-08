use std::ops::{Add, Deref, DerefMut};

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

impl Add for ZIndex {
    type Output = ZIndex;

    fn add(self, rhs: Self) -> Self::Output {
        ZIndex(self.0 + rhs.0)
    }
}