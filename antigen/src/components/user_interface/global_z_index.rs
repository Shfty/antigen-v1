use std::ops::{Deref, DerefMut};

use super::ZIndex;

#[derive(Debug, Default, Copy, Clone)]
pub struct GlobalZIndex(i64);

impl Deref for GlobalZIndex {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GlobalZIndex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<ZIndex> for GlobalZIndex {
    fn from(z_index: ZIndex) -> Self {
        GlobalZIndex(*z_index)
    }
}