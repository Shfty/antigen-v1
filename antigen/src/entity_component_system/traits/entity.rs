use std::{
    fmt::Display,
    sync::atomic::{AtomicU32, Ordering},
};

#[derive(Debug, Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct EntityID(pub u32);

impl EntityID {
    pub fn next() -> Self {
        static COUNTER: AtomicU32 = AtomicU32::new(0);
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

impl From<u32> for EntityID {
    fn from(val: u32) -> Self {
        EntityID(val)
    }
}

impl Into<u32> for EntityID {
    fn into(self) -> u32 {
        self.0
    }
}
