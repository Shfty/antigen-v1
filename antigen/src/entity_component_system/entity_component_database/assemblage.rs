use std::{
    collections::HashMap,
    ops::{Add, AddAssign},
};

use crate::{
    entity_component_system::create_entity, entity_component_system::insert_entity_component,
    entity_component_system::ComponentDebugTrait, entity_component_system::ComponentID,
    entity_component_system::ComponentTrait, entity_component_system::EntityComponentSystem,
    entity_component_system::EntityID, uid::UID,
};

use super::{ComponentStorage, EntityComponentDatabase, EntityComponentDirectory};

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

pub struct AssemblageBuilder<CS, CD>
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
{
    assemblage: Assemblage<CS, CD>,
    component_data: HashMap<ComponentID, ComponentConstructor<CS, CD>>,
}

impl<CS, CD> AssemblageBuilder<CS, CD>
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
{
    pub fn new(assemblage: Assemblage<CS, CD>) -> Self {
        AssemblageBuilder {
            assemblage,
            component_data: HashMap::new(),
        }
    }

    pub fn add_component<C>(mut self, component_data: C) -> Result<Self, String>
    where
        C: ComponentTrait + ComponentDebugTrait + Clone + 'static,
    {
        self.component_data.insert(
            ComponentID::get::<C>(),
            Box::new(
                move |component_storage: &mut CS,
                      entity_component_directory: &mut CD,
                      entity_id|
                      -> Result<(), String> {
                    match insert_entity_component(
                        component_storage,
                        entity_component_directory,
                        entity_id,
                        component_data.clone(),
                    ) {
                        Ok(_) => Ok(()),
                        Err(err) => Err(err),
                    }
                },
            ),
        );
        Ok(self)
    }

    pub fn finish(mut self) -> Assemblage<CS, CD> {
        self.assemblage.component_constructors = self.component_data;
        self.assemblage
    }
}

type ComponentConstructor<CS, CD> =
    Box<dyn FnMut(&mut CS, &mut CD, EntityID) -> Result<(), String>>;

/// An object template as defined by a set of components with given default values
pub struct Assemblage<S, D>
where
    S: ComponentStorage,
    D: EntityComponentDirectory,
{
    pub name: String,
    pub description: String,
    component_constructors: HashMap<ComponentID, ComponentConstructor<S, D>>,
}

// TODO: Rework using closures instead of component IDs for instantiation
impl<CS, CD> Assemblage<CS, CD>
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
{
    pub fn new(name: &str, description: &str) -> Self {
        Assemblage {
            name: name.into(),
            description: description.into(),
            component_constructors: HashMap::new(),
        }
    }

    pub fn build(name: &str, description: &str) -> AssemblageBuilder<CS, CD>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
    {
        AssemblageBuilder::new(Assemblage::new(name, description))
    }

    pub fn create_and_assemble_entity<'a>(
        &mut self,
        db: &'a mut EntityComponentDatabase<CS, CD>,
        debug_label: Option<&str>,
    ) -> Result<EntityID, String>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
    {
        let entity_id = create_entity(
            &mut db.component_storage,
            &mut db.entity_component_directory,
            &mut db.callback_manager,
            debug_label,
        )?;
        self.assemble_entity(db, entity_id)
    }

    pub fn assemble_entity<'a>(
        &mut self,
        db: &'a mut EntityComponentDatabase<CS, CD>,
        entity_id: EntityID,
    ) -> Result<EntityID, String>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
    {
        for component_constructor in &mut self.component_constructors.values_mut() {
            component_constructor(
                &mut db.component_storage,
                &mut db.entity_component_directory,
                entity_id,
            )?;
        }

        Ok(entity_id)
    }
}
