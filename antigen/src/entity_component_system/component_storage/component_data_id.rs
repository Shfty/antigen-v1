use std::{fmt::Display, sync::atomic::AtomicUsize, sync::atomic::Ordering};

use crate::uid::UID;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ComponentDataID(pub UID);

impl ComponentDataID {
    pub fn next() -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(1);
        ComponentDataID(COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

impl Display for ComponentDataID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ComponentDataID(component_data_id) = self;
        write!(f, "{}", component_data_id)
    }
}
