use std::cell::{Ref, RefMut};

use store::StoreQuery;

use crate::{
    components::{Position, Velocity},
    entity_component_system::{ComponentStore, EntityID, SystemError, SystemTrait},
};

type IntegratePosition<'a> = (EntityID, Ref<'a, Velocity>, RefMut<'a, Position>);

#[derive(Debug)]
pub struct PositionIntegrator;

impl SystemTrait for PositionIntegrator {
    fn run(&mut self, db: &mut ComponentStore) -> Result<(), SystemError> {
        for (_key, velocity, mut position) in StoreQuery::<IntegratePosition>::iter(db.as_ref()) {
            **position += **velocity;
        }

        Ok(())
    }
}
