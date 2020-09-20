mod heap_system_storage;

use std::collections::HashMap;

pub use heap_system_storage::HeapSystemStorage;

use super::{ComponentStorage, EntityComponentDirectory, SystemID, SystemTrait};

/// Trait for handling systems execution for a given EntityComponentSystem
pub trait SystemStorage<CS, CD> {
    fn insert_system<T>(&mut self, system: T) -> SystemID
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
        T: SystemTrait<CS, CD> + 'static;

    fn get_systems(&mut self) -> HashMap<SystemID, &mut (dyn SystemTrait<CS, CD> + 'static)>
    where
        CS: ComponentStorage + 'static,
        CD: EntityComponentDirectory + 'static;
}
