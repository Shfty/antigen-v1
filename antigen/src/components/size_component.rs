use crate::{
    ecs::{ComponentMetadataTrait, ComponentTrait},
    primitive_types::IVector2,
};

#[derive(Debug, Copy, Clone)]
pub struct SizeComponent {
    pub data: IVector2,
}

impl SizeComponent {
    pub fn new(data: IVector2) -> Self {
        SizeComponent { data }
    }
}

impl Default for SizeComponent {
    fn default() -> Self {
        SizeComponent::new(IVector2::default())
    }
}

impl ComponentTrait for SizeComponent {}

impl ComponentMetadataTrait for SizeComponent {
    fn get_name() -> &'static str {
        "Size"
    }

    fn get_description() -> &'static str {
        "2D cartesian size"
    }
}
