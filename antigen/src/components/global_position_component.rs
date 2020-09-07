use crate::{
    ecs::{ComponentMetadataTrait, ComponentTrait},
    primitive_types::IVector2,
};

#[derive(Debug, Copy, Clone)]
pub struct GlobalPositionComponent {
    pub data: IVector2,
}

impl GlobalPositionComponent {
    pub fn new(data: IVector2) -> Self {
        GlobalPositionComponent { data }
    }
}

impl Default for GlobalPositionComponent {
    fn default() -> Self {
        GlobalPositionComponent::new(IVector2::default())
    }
}

impl ComponentTrait for GlobalPositionComponent {}

impl ComponentMetadataTrait for GlobalPositionComponent {
    fn get_name() -> &'static str {
        "Global Position"
    }

    fn get_description() -> &'static str {
        "Hierarchical 2D Cartesian Position"
    }
}
