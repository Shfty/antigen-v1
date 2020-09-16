mod single_threaded_system_runner;

pub use single_threaded_system_runner::SingleThreadedSystemRunner;

use super::{
    entity_component_database::ComponentStorage,
    entity_component_database::EntityComponentDirectory, system_storage::SystemStorage,
    EntityComponentDatabase, SystemError,
};

/// Trait for handling systems execution for a given EntityComponentSystem
pub trait SystemRunner {
    fn run<SS, CS, CD>(
        &mut self,
        system_storage: &mut SS,
        entity_component_database: &mut EntityComponentDatabase<CS, CD>,
    ) -> Result<(), SystemError>
    where
        SS: SystemStorage<CS, CD>,
        CS: ComponentStorage,
        CD: EntityComponentDirectory;
}
