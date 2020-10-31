use std::fmt::Debug;

use crate::entity_component_system::ComponentStore;

/// A monolithic set of logic that runs on sets of entities with specific component layouts
pub trait SystemTrait: Debug {
    fn run(&mut self, db: &mut ComponentStore) -> Result<(), SystemError>;
}

// Error type for systems, Err represents an error, Quit represents a successful exit
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
