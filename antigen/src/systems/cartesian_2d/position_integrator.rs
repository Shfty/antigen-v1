use std::cell::{Ref, RefMut};

use store::StoreQuery;

use crate::{
    components::{Position, Velocity},
    entity_component_system::{
        ComponentStore, EntityID, SystemError, SystemTrait,
    },
};

#[derive(Debug)]
pub struct PositionIntegrator;

impl SystemTrait for PositionIntegrator {
    fn run(&mut self, db: &mut ComponentStore) -> Result<(), SystemError> {
        StoreQuery::<(EntityID, Ref<Velocity>, RefMut<Position>)>::iter(db.as_ref())
            .for_each(|(_key, velocity, mut position)| **position += **velocity);

        Ok(())
    }
}
