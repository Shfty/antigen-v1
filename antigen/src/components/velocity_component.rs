use crate::{primitive_types::IVector2, ecs::{ComponentMetadataTrait, ComponentTrait}};

#[derive(Debug, Copy, Clone)]
pub struct VelocityComponent {
    pub data: IVector2
}

impl VelocityComponent {
    pub fn new(data: IVector2) -> Self {
        VelocityComponent { data }
    }
}

impl Default for VelocityComponent {
    fn default() -> Self {
        VelocityComponent::new(IVector2::default())
    }
}

impl ComponentTrait for VelocityComponent {}

impl ComponentMetadataTrait for VelocityComponent {
    fn get_name() -> &'static str {
        "Velocity"
    }

    fn get_description() -> &'static str {
        "2D cartesian velocity"
    }
}