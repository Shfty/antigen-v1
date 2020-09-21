use std::{
    fmt::{Debug, Display},
    sync::atomic::AtomicUsize,
    sync::atomic::Ordering,
};

use crate::{
    entity_component_system::ComponentStorage, entity_component_system::EntityComponentDirectory,
    entity_component_system::SystemInterface, uid::UID,
};

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

#[derive(Debug, Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct SystemID(pub UID);

impl SystemID {
    pub fn next() -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        SystemID(COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

impl Default for SystemID {
    fn default() -> Self {
        SystemID::next()
    }
}

impl Display for SystemID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let SystemID(entity_id) = self;
        write!(f, "{}", entity_id)
    }
}

/// A monolithic set of logic that runs on sets of entities with specific component layouts
pub trait SystemTrait<CS, CD>
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
{
    fn run(&mut self, db: &mut SystemInterface<CS, CD>) -> Result<(), SystemError>;
}

/// Debug implementation for Systems
pub trait SystemDebugTrait {
    fn get_name() -> &'static str;
}
