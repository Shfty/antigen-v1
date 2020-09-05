use super::{
    component::{ComponentData, ComponentID},
    ComponentTrait, ECS, GUID,
};
use std::collections::HashMap;

pub type AssemblageID = GUID;

#[derive(Debug, Clone)]
pub struct Assemblage {
    official_name: String,
    description: String,
}

impl Assemblage {
    pub fn new(official_name: &str, description: &str) -> Assemblage {
        Assemblage {
            official_name: official_name.into(),
            description: description.into(),
        }
    }
}

pub struct AssemblageBuilder<'a> {
    ecs: &'a mut ECS,
    assemblage: Assemblage,
    component_data: HashMap<ComponentID, ComponentData>,
}

impl<'a> AssemblageBuilder<'a> {
    pub fn new(ecs: &'a mut ECS, official_name: &str, description: &str) -> AssemblageBuilder<'a> {
        AssemblageBuilder {
            ecs,
            assemblage: Assemblage::new(official_name, description),
            component_data: HashMap::new(),
        }
    }

    pub fn component<T: ComponentTrait + 'static>(mut self, data: T) -> AssemblageBuilder<'a> {
        self.component_data
            .insert(ECS::get_component_id::<T>(), Box::new(data));
        self
    }

    pub fn finish(self) -> AssemblageID {
        self.ecs
            .register_assemblage(self.assemblage, self.component_data)
    }
}
