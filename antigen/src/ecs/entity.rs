use std::{ops::{AddAssign, Add}, fmt::Display};
use crate::primitive_types::UID;

#[derive(Debug, Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct EntityID(pub UID);

impl Display for EntityID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let EntityID(entity_id) = self;
        write!(f, "{}", entity_id)
    }
}

impl Add<UID> for EntityID {
    type Output = EntityID;

    fn add(self, rhs: i64) -> Self::Output {
        let EntityID(self_id) = self;
        EntityID(self_id + rhs)
    }
}

impl AddAssign<UID> for EntityID {
    fn add_assign(&mut self, rhs: UID) {
        let EntityID(self_id) = self;
        *self_id = *self_id + rhs;
    }
}

#[derive(Debug)]
pub struct EntityDebug {
    pub label: String,
}

impl EntityDebug {
    pub fn new(label: &str) -> EntityDebug {
        EntityDebug {
            label: label.into(),
        }
    }
}
