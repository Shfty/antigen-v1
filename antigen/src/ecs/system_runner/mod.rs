use super::{
    entity_component_database::EntityComponentDatabase,
    entity_component_database::EntityComponentDatabaseDebug, SystemEvent, SystemTrait,
};

mod single_threaded_system_runner;

pub use single_threaded_system_runner::SingleThreadedSystemRunner;

pub trait SystemRunner<'a, T>
where
    T: EntityComponentDatabase + EntityComponentDatabaseDebug,
{
    fn new(db: &'a mut T) -> Self;

    fn register_system(&mut self, name: &str, system: &'a mut dyn SystemTrait<T>);

    fn run(&mut self) -> Result<SystemEvent, String>;
}