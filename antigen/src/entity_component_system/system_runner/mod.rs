mod single_threaded_system_runner;
pub use single_threaded_system_runner::SingleThreadedSystemRunner;

use super::{
    entity_component_database::ComponentStorage,
    entity_component_database::EntityComponentDirectory, EntityComponentDatabase, SystemError,
    SystemTrait,
};

/// Trait for handling systems execution for a given EntityComponentSystem
pub trait SystemRunner<S, D>
where
    S: ComponentStorage,
    D: EntityComponentDirectory,
{
    fn new() -> Self;

    fn register_system<T>(&mut self, name: &str, system: T)
    where
        T: SystemTrait<S, D> + 'static;

    fn run(&mut self, ecs: &mut EntityComponentDatabase<S, D>) -> Result<(), SystemError>;
}
