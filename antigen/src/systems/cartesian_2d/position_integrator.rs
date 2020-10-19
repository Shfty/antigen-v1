use std::cell::{Ref, RefMut};

use store::StoreQuery;

use crate::{
    components::{Position, Velocity},
    entity_component_system::{
        system_interface::SystemInterface, ComponentData, EntityComponentDirectory, EntityID,
        SystemError, SystemTrait,
    },
};

#[derive(Debug)]
pub struct PositionIntegrator;

impl<CD> SystemTrait<CD> for PositionIntegrator
where
    CD: EntityComponentDirectory,
{
    fn run(&mut self, db: &mut SystemInterface<CD>) -> Result<(), SystemError>
    where
        CD: EntityComponentDirectory,
    {
        StoreQuery::<
            EntityID,
            (
                Ref<ComponentData<Velocity>>,
                RefMut<ComponentData<Position>>,
            ),
        >::iter(db.component_store)
        .for_each(|(_key, (velocity, mut position))| ***position += ***velocity);

        Ok(())
    }
}
