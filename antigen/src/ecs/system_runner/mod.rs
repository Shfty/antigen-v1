use super::{entity_component_database::EntityComponentDatabase, SystemError, SystemTrait};

mod single_threaded_system_runner;

pub use single_threaded_system_runner::SingleThreadedSystemRunner;

pub trait SystemRunner<'a, T>
where
    T: EntityComponentDatabase,
{
    fn new(db: &'a mut T) -> Self;

    fn register_system(&mut self, name: &str, system: &'a mut dyn SystemTrait<T>);

    fn run(&mut self) -> Result<(), SystemError>;
}
