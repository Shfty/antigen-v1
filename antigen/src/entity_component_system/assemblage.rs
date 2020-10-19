use std::{
    collections::HashMap,
    ops::{Add, AddAssign},
};

use crate::{
    core::uid::UID, entity_component_system::ComponentID, entity_component_system::ComponentTrait,
    entity_component_system::EntityID,
};

use super::{EntityComponentDirectory, SystemInterface};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct AssemblageID(pub UID);

impl Add<UID> for AssemblageID {
    type Output = AssemblageID;

    fn add(self, rhs: usize) -> Self::Output {
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

pub struct AssemblageBuilder<CD>
where
    CD: EntityComponentDirectory + 'static,
{
    assemblage: Assemblage<CD>,
    component_data: HashMap<ComponentID, ComponentConstructor<CD>>,
}

impl<CD> AssemblageBuilder<CD>
where
    CD: EntityComponentDirectory,
{
    pub fn new(assemblage: Assemblage<CD>) -> Self {
        AssemblageBuilder {
            assemblage,
            component_data: HashMap::new(),
        }
    }

    pub fn add_component<C>(mut self, component_data: C) -> Result<Self, String>
    where
        C: ComponentTrait + Clone + 'static,
    {
        self.component_data.insert(
            ComponentID::get::<C>(),
            Box::new(
                move |db: &mut SystemInterface<CD>, entity_id| -> Result<(), String> {
                    match db.insert_entity_component(entity_id, component_data.clone()) {
                        Ok(_) => Ok(()),
                        Err(err) => Err(err),
                    }
                },
            ),
        );
        Ok(self)
    }

    pub fn finish(mut self) -> Assemblage<CD> {
        self.assemblage.component_constructors = self.component_data;
        self.assemblage
    }
}

type ComponentConstructor<CD> =
    Box<dyn FnMut(&mut SystemInterface<CD>, EntityID) -> Result<(), String>>;

/// An object template as defined by a set of components with given default values
pub struct Assemblage<D>
where
    D: EntityComponentDirectory + 'static,
{
    pub name: String,
    pub description: String,
    component_constructors: HashMap<ComponentID, ComponentConstructor<D>>,
}

impl<CD> Assemblage<CD>
where
    CD: EntityComponentDirectory,
{
    pub fn new(name: &str, description: &str) -> Self {
        Assemblage {
            name: name.into(),
            description: description.into(),
            component_constructors: HashMap::new(),
        }
    }

    pub fn build(name: &str, description: &str) -> AssemblageBuilder<CD>
    where
        CD: EntityComponentDirectory,
    {
        AssemblageBuilder::new(Assemblage::new(name, description))
    }

    pub fn create_and_assemble_entity<'a>(
        &mut self,
        db: &'a mut SystemInterface<CD>,
        debug_label: Option<&str>,
    ) -> Result<EntityID, String>
    where
        CD: EntityComponentDirectory,
    {
        let entity_id = db.create_entity(debug_label)?;
        self.assemble_entity(db, entity_id)
    }

    pub fn assemble_entity<'a>(
        &mut self,
        db: &'a mut SystemInterface<CD>,
        entity_id: EntityID,
    ) -> Result<EntityID, String>
    where
        CD: EntityComponentDirectory,
    {
        for component_constructor in &mut self.component_constructors.values_mut() {
            component_constructor(db, entity_id)?;
        }

        Ok(entity_id)
    }
}
