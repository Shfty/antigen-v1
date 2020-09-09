mod assemblage;
mod component;
mod entity;
mod system;

pub mod entity_component_database;
pub mod system_runner;

pub use assemblage::{Assemblage, AssemblageID};
pub use component::{ComponentDataID, ComponentMetadataTrait, ComponentTrait};
pub use entity::EntityID;
pub use entity_component_database::{EntityComponentDatabase, EntityComponentDatabaseDebug};
pub use system::{SystemEvent, SystemTrait};
pub use system_runner::SystemRunner;
