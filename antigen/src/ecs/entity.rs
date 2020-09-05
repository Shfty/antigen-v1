use super::GUID;

pub type EntityID = GUID;

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
