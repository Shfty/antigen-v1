use crate::{primitive_types::IVector2, entity_component_system::{ComponentDebugTrait, ComponentTrait}};

#[derive(Debug, Copy, Clone)]
pub struct VelocityComponent {
    data: IVector2
}

impl VelocityComponent {
    pub fn new(data: IVector2) -> Self {
        VelocityComponent { data }
    }

    pub fn get_velocity(&self) -> IVector2 {
        self.data
    }

    pub fn set_velocity(&mut self, velocity: IVector2) -> &mut Self {
        self.data = velocity;
        self
    }
}

impl Default for VelocityComponent {
    fn default() -> Self {
        VelocityComponent::new(IVector2::default())
    }
}

impl ComponentTrait for VelocityComponent {}

impl ComponentDebugTrait for VelocityComponent {
    fn get_name() -> String {
        "Velocity".into()
    }

    fn get_description() -> String {
        "2D cartesian velocity".into()
    }
}