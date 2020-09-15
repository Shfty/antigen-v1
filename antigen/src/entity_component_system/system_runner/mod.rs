mod single_threaded_system_runner;
pub use single_threaded_system_runner::SingleThreadedSystemRunner;

use super::{ComponentStorage, EntityComponentDirectory, SystemError, EntityComponentDatabase, SystemTrait};

/// Trait for handling systems execution for a given EntityComponentSystem
pub trait SystemRunner<'a, S, D>
where
    S: ComponentStorage,
    D: EntityComponentDirectory,
{
    fn new() -> Self;

    fn register_system(&mut self, name: &str, system: &'a mut dyn SystemTrait<S, D>);

    fn run(
        &mut self,
        ecs: &mut EntityComponentDatabase<S, D>,
    ) -> Result<(), SystemError>;
}
