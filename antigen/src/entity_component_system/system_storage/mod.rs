mod heap_system_storage;

pub use heap_system_storage::HeapSystemStorage;

use super::{
    entity_component_database::ComponentStorage,
    entity_component_database::EntityComponentDirectory, SystemTrait,
};

/// Trait for handling systems execution for a given EntityComponentSystem
pub trait SystemStorage<CS, CD> {
    fn insert_system<T>(&mut self, name: &str, system: T)
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
        T: SystemTrait<CS, CD> + 'static;

    fn get_system_names(&self) -> Vec<String>;
    fn get_system(&mut self, name: &str) -> Result<&mut dyn SystemTrait<CS, CD>, String>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory;
}
