mod single_threaded_system_runner;

pub use single_threaded_system_runner::SingleThreadedSystemRunner;

use super::{
    system_storage::SystemStorage, EntityComponentDirectory, SystemError, SystemInterface,
};

/// Trait for handling systems execution for a given EntityComponentSystem
pub trait SystemRunner {
    fn run<'a, SS, CD>(
        &mut self,
        system_storage: &'a mut SS,
        system_interface: &'a mut SystemInterface<'a, CD>,
    ) -> Result<(), SystemError>
    where
        SS: SystemStorage<CD>,

        CD: EntityComponentDirectory;
}
