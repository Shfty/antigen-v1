use super::{
    component::{get_component_id, CloneComponentTrait, ComponentData, ComponentID},
    ComponentMetadataTrait, ComponentTrait, ECS,
};
use crate::primitive_types::UID;
use std::{
    collections::HashMap,
    ops::{Add, AddAssign},
};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct AssemblageID(pub UID);

impl Add<UID> for AssemblageID {
    type Output = AssemblageID;

    fn add(self, rhs: i64) -> Self::Output {
        let AssemblageID(self_id) = self;
        AssemblageID(self_id + rhs)
    }
}

impl AddAssign<UID> for AssemblageID {
    fn add_assign(&mut self, rhs: UID) {
        let AssemblageID(self_id) = self;
        *self_id = *self_id + rhs;
    }
}

#[derive(Debug, Clone)]
pub struct Assemblage {
    pub official_name: String,
    pub description: String,
}

impl Assemblage {
    pub fn new(official_name: &str, description: &str) -> Assemblage {
        Assemblage {
            official_name: official_name.into(),
            description: description.into(),
        }
    }
}

pub struct AssemblageBuilder<'a, T>
where
    T: ECS,
{
    ecs: &'a mut T,
    assemblage: Assemblage,
    component_data: HashMap<ComponentID, ComponentData>,
}

impl<'a, T> AssemblageBuilder<'a, T>
where
    T: ECS,
{
    pub fn new(ecs: &'a mut T, official_name: &str, description: &str) -> AssemblageBuilder<'a, T> {
        AssemblageBuilder {
            ecs,
            assemblage: Assemblage::new(official_name, description),
            component_data: HashMap::new(),
        }
    }

    pub fn component<J: ComponentTrait + CloneComponentTrait + ComponentMetadataTrait + 'static>(
        mut self,
        data: J,
    ) -> AssemblageBuilder<'a, T>
    where
        T: ECS,
    {
        if !self.ecs.is_component_registered::<J>() {
            self.ecs.register_component::<J>();
        }
        self.component_data
            .insert(get_component_id::<J>(), Box::new(data));
        self
    }

    pub fn finish(self) -> AssemblageID {
        self.ecs
            .register_assemblage(self.assemblage, self.component_data)
    }
}
