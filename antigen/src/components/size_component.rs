use crate::{
    ecs::{ComponentDebugTrait, ComponentTrait},
    primitive_types::IVector2,
};

#[derive(Debug, Copy, Clone)]
pub struct SizeComponent {
    data: IVector2,
}

impl SizeComponent {
    pub fn new(data: IVector2) -> Self {
        SizeComponent { data }
    }

    pub fn get_size(&self) -> IVector2 {
        self.data
    }

    pub fn set_size(&mut self, size: IVector2) -> &mut Self {
        self.data = size;
        self
    }
}

impl Default for SizeComponent {
    fn default() -> Self {
        SizeComponent::new(IVector2::default())
    }
}

impl ComponentTrait for SizeComponent {}

impl ComponentDebugTrait for SizeComponent {
    fn get_name() -> String {
        "Size".into()
    }

    fn get_description() -> String {
        "2D cartesian size".into()
    }
}
