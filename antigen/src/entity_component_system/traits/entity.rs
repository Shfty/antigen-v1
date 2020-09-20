use crate::uid::UID;
use std::{fmt::Display, sync::atomic::AtomicUsize, sync::atomic::Ordering};

#[derive(Debug, Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct EntityID(pub UID);

impl EntityID {
    pub fn next() -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(1);
        EntityID(COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

impl Default for EntityID {
    fn default() -> Self {
        EntityID::next()
    }
}

impl Display for EntityID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let EntityID(entity_id) = self;
        write!(f, "{}", entity_id)
    }
}
