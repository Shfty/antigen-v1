use std::collections::HashMap;

use crate::entity_component_system::{
    ComponentStorage, EntityComponentDirectory, SystemID, SystemTrait,
};

use super::SystemStorage;

pub struct HeapSystemStorage<S, D>
where
    S: ComponentStorage,
    D: EntityComponentDirectory,
{
    systems: HashMap<SystemID, Box<dyn SystemTrait<S, D>>>,
}

impl<'a, S, D> HeapSystemStorage<S, D>
where
    S: ComponentStorage,
    D: EntityComponentDirectory,
{
    pub fn new() -> HeapSystemStorage<S, D> {
        HeapSystemStorage {
            systems: HashMap::new(),
        }
    }
}

impl<'a, S, D> Default for HeapSystemStorage<S, D>
where
    S: ComponentStorage,
    D: EntityComponentDirectory,
{
    fn default() -> Self {
        HeapSystemStorage::new()
    }
}

impl<'a, CS, CD> SystemStorage<CS, CD> for HeapSystemStorage<CS, CD>
where
    CS: ComponentStorage + 'static,
    CD: EntityComponentDirectory + 'static,
{
    fn insert_system<T>(&mut self, system: T) -> SystemID
    where
        T: SystemTrait<CS, CD> + 'static,
    {
        let id = SystemID::next();
        self.systems.insert(id, Box::new(system));
        id
    }

    fn get_systems(&mut self) -> HashMap<SystemID, &mut (dyn SystemTrait<CS, CD> + 'static)> {
        self.systems
            .iter_mut()
            .map(|(system_id, system)| (*system_id, system.as_mut()))
            .collect()
    }
}
