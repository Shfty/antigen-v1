use std::collections::HashMap;

use crate::entity_component_system::{EntityComponentDirectory, SystemID, SystemTrait};

use super::SystemStorage;

pub struct HeapSystemStorage<D>
where
    D: EntityComponentDirectory,
{
    systems: HashMap<SystemID, Box<dyn SystemTrait<D>>>,
}

impl<'a, D> HeapSystemStorage<D>
where
    D: EntityComponentDirectory,
{
    pub fn new() -> HeapSystemStorage<D> {
        HeapSystemStorage {
            systems: HashMap::new(),
        }
    }
}

impl<'a, D> Default for HeapSystemStorage<D>
where
    D: EntityComponentDirectory,
{
    fn default() -> Self {
        HeapSystemStorage::new()
    }
}

impl<'a, CD> SystemStorage<CD> for HeapSystemStorage<CD>
where
    CD: EntityComponentDirectory + 'static,
{
    fn insert_system<T>(&mut self, system: T) -> SystemID
    where
        T: SystemTrait<CD> + 'static,
    {
        let id = SystemID::next::<T>();
        self.systems.insert(id, Box::new(system));
        id
    }

    fn get_systems(&mut self) -> HashMap<SystemID, &mut (dyn SystemTrait<CD> + 'static)> {
        self.systems
            .iter_mut()
            .map(|(system_id, system)| (*system_id, system.as_mut()))
            .collect()
    }
}
