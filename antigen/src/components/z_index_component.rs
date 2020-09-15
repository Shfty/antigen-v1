use crate::entity_component_system::{ComponentDebugTrait, ComponentTrait};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ZIndexComponent {
    z: i64
}

impl ZIndexComponent {
    pub fn new(z: i64) -> Self {
        ZIndexComponent {
            z
        }
    }

    pub fn get_z(&self) -> i64 {
        self.z
    }

    pub fn set_z(&mut self, z: i64) -> &mut Self {
        self.z = z;
        self
    }
}

impl ComponentTrait for ZIndexComponent {}

impl ComponentDebugTrait for ZIndexComponent {
    fn get_name() -> String {
        "Z Index".into()
    }

    fn get_description() -> String {
        "Z index for 2D entities".into()
    }
}
