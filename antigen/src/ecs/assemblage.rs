use super::{
    component::{ComponentID},
    entity_component_database::EntityComponentDatabase,
    ComponentDebugTrait, ComponentTrait, EntityID,
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

pub struct AssemblageBuilder<'a, T>
where
    T: EntityComponentDatabase,
{
    db: &'a mut T,
    assemblage: Assemblage,
    component_data: HashMap<ComponentID, Box<dyn ComponentTrait>>,
}

impl<'a, T> AssemblageBuilder<'a, T>
where
    T: EntityComponentDatabase,
{
    pub fn new(db: &'a mut T, assemblage: Assemblage) -> Self {
        AssemblageBuilder {
            db,
            assemblage,
            component_data: HashMap::new(),
        }
    }

    pub fn add_component<C>(mut self, component: C) -> Result<Self, String>
    where
        C: ComponentTrait + ComponentDebugTrait + 'static,
    {
        if !self.db.is_valid_component::<C>() {
            self.db.register_component::<C>()?;
        }

        self.component_data
            .insert(ComponentID::get::<C>(), Box::new(component));
        Ok(self)
    }

    pub fn finish(mut self) -> Assemblage {
        self.assemblage.component_data = self.component_data;
        self.assemblage
    }
}

#[derive(Debug, Clone)]
pub struct Assemblage {
    pub name: String,
    pub description: String,
    component_data: HashMap<ComponentID, Box<dyn ComponentTrait>>,
}

impl Assemblage {
    pub fn new(name: &str, description: &str) -> Self {
        Assemblage {
            name: name.into(),
            description: description.into(),
            component_data: HashMap::new(),
        }
    }

    pub fn build<'a, T>(db: &'a mut T, name: &str, description: &str) -> AssemblageBuilder<'a, T>
    where
        T: EntityComponentDatabase,
    {
        AssemblageBuilder::new(db, Assemblage::new(name, description))
    }

    pub fn create_and_assemble_entity(
        &self,
        db: &mut impl EntityComponentDatabase,
        debug_label: Option<&str>,
    ) -> Result<EntityID, String> {
        let entity_id = db.create_entity(debug_label)?;
        self.assemble_entity(db, entity_id)
    }

    pub fn assemble_entity(
        &self,
        db: &mut impl EntityComponentDatabase,
        entity_id: EntityID,
    ) -> Result<EntityID, String> {
        for (component_id, component_data) in &self.component_data {
            db.add_registered_component_to_entity(
                entity_id,
                *component_id,
                component_data.clone(),
            )?;
        }

        Ok(entity_id)
    }
}
