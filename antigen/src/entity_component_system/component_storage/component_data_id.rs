use std::fmt::Display;

use crate::uid::UID;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ComponentDataID(pub UID);

impl ComponentDataID {
    pub fn next() -> Self {
        ComponentDataID(crate::uid::new())
    }
}

impl Display for ComponentDataID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ComponentDataID(component_data_id) = self;
        write!(f, "{}", component_data_id)
    }
}
