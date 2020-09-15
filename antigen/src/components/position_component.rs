use crate::{
    entity_component_system::{ComponentDebugTrait, ComponentTrait},
    primitive_types::IVector2,
};
use std::fmt::Debug;

#[derive(Debug, Copy, Clone)]
pub struct PositionComponent {
    data: IVector2,
}

impl PositionComponent {
    pub fn new(data: IVector2) -> Self {
        PositionComponent { data }
    }

    pub fn get_position(&self) -> IVector2 {
        self.data
    }

    pub fn set_position(&mut self, data: IVector2) -> &mut Self {
        self.data = data;
        self
    }
}

impl Default for PositionComponent {
    fn default() -> Self {
        PositionComponent::new(IVector2::default())
    }
}

impl ComponentTrait for PositionComponent {}

impl ComponentDebugTrait for PositionComponent {
    fn get_name() -> String {
        "Position".into()
    }

    fn get_description() -> String {
        "2D Cartesian Position".into()
    }
}
