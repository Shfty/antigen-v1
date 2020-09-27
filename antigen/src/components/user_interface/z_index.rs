use crate::entity_component_system::{ComponentDebugTrait, ComponentTrait};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ZIndex(pub i64);

impl From<i64> for ZIndex {
    fn from(z: i64) -> Self {
        ZIndex(z)
    }
}

impl Into<i64> for ZIndex {
    fn into(self) -> i64 {
        self.0
    }
}

impl ComponentTrait for ZIndex {}

impl ComponentDebugTrait for ZIndex {
    fn get_name() -> String {
        "Z Index".into()
    }

    fn get_description() -> String {
        "Z index for 2D entities".into()
    }
}
