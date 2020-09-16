mod traits;

pub mod entity_component_database;
pub mod system_runner;

pub use entity_component_database::{
    Assemblage, AssemblageID, ComponentDataID, EntityComponentDatabase,
};
pub use system_runner::SystemRunner;
pub use traits::{
    ComponentDebugTrait, ComponentID, ComponentTrait, EntityID, SystemError, SystemTrait,
};

pub type EntityCreateCallback<S, D> =
    fn(&mut EntityComponentDatabase<S, D>, EntityID, Option<&str>);
pub type ComponentCreateCallback<S, D> =
    fn(&mut EntityComponentDatabase<S, D>, ComponentID, &str, &str);
pub type ComponentDropCallback = fn(&mut dyn ComponentTrait);
