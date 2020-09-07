use crate::{
    ecs::{ComponentMetadataTrait, ComponentTrait},
    primitive_types::IVector2,
};
use std::fmt::Debug;

#[derive(Debug, Copy, Clone)]
pub struct PositionComponent {
    pub data: IVector2,
}

impl PositionComponent {
    pub fn new(data: IVector2) -> Self {
        PositionComponent { data }
    }
}

impl Default for PositionComponent {
    fn default() -> Self {
        PositionComponent::new(IVector2::default())
    }
}

impl ComponentTrait for PositionComponent {}

impl ComponentMetadataTrait for PositionComponent {
    fn get_name() -> &'static str {
        "Position"
    }

    fn get_description() -> &'static str {
        "2D Cartesian Position"
    }
}
