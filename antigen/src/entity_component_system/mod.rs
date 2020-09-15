mod assemblage;
mod component;
mod entity;
mod system;

pub mod entity_component_database;
pub mod system_runner;

pub use assemblage::{Assemblage, AssemblageID};
pub use component::{ComponentDataID, ComponentDebugTrait, ComponentID, ComponentTrait};
pub use entity::EntityID;
pub use entity_component_database::{ComponentStorage, HeapComponentStorage};
pub use entity_component_database::EntityComponentDirectory;
pub use entity_component_database::EntityComponentDatabase;
pub use system::{SystemError, SystemTrait};
pub use system_runner::SystemRunner;

pub type EntityCreateCallback<S, D> =
    fn(&mut EntityComponentDatabase<S, D>, EntityID, Option<&str>);
pub type ComponentCreateCallback<S, D> =
    fn(&mut EntityComponentDatabase<S, D>, ComponentID, &str, &str);
pub type ComponentDropCallback = fn(&mut dyn ComponentTrait);
