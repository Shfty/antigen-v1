use crate::{
    ecs::{ComponentDebugTrait, ComponentTrait},
    primitive_types::IVector2,
};

#[derive(Debug, Copy, Clone)]
pub struct GlobalPositionComponent {
    data: IVector2,
}

impl GlobalPositionComponent {
    pub fn new(data: IVector2) -> Self {
        GlobalPositionComponent { data }
    }

    pub fn get_global_position(&self) -> IVector2 {
        self.data
    }

    pub fn set_global_position(&mut self, global_position: IVector2) -> &mut Self {
        self.data = global_position;
        self
    }
}

impl Default for GlobalPositionComponent {
    fn default() -> Self {
        GlobalPositionComponent::new(IVector2::default())
    }
}

impl ComponentTrait for GlobalPositionComponent {}

impl ComponentDebugTrait for GlobalPositionComponent {
    fn get_name() -> String {
        "Global Position".into()
    }

    fn get_description() -> String {
        "Hierarchical 2D Cartesian Position".into()
    }
}
