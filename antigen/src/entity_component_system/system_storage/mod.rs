mod heap_system_storage;

use std::collections::HashMap;

pub use heap_system_storage::HeapSystemStorage;

use super::{EntityComponentDirectory, SystemID, SystemTrait};

/// Trait for handling systems execution for a given EntityComponentSystem
pub trait SystemStorage<CD> {
    fn insert_system<T>(&mut self, system: T) -> SystemID
    where
        CD: EntityComponentDirectory,
        T: SystemTrait<CD> + 'static;

    fn get_systems(&mut self) -> HashMap<SystemID, &mut (dyn SystemTrait<CD> + 'static)>
    where
        CD: EntityComponentDirectory + 'static;
}
