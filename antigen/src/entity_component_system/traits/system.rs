use std::fmt::Debug;

use crate::{
    entity_component_system::entity_component_database::ComponentStorage,
    entity_component_system::entity_component_database::EntityComponentDirectory,
    entity_component_system::EntityComponentDatabase, uid::UID,
};

#[derive(Debug, Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct SystemID(pub UID);

#[derive(Debug, Clone)]
pub enum SystemError {
    Err(String),
    Quit,
}

impl From<String> for SystemError {
    fn from(string: String) -> Self {
        SystemError::Err(string)
    }
}

impl From<&str> for SystemError {
    fn from(string: &str) -> Self {
        SystemError::Err(string.into())
    }
}

/// A monolithic set of logic that runs on sets of entities with specific component layouts
pub trait SystemTrait<S, D>
where
    S: ComponentStorage,
    D: EntityComponentDirectory,
{
    fn run<'a>(&mut self, db: &'a mut EntityComponentDatabase<S, D>) -> Result<(), SystemError>;
}
