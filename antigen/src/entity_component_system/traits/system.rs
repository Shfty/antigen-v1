use std::{
    fmt::{Debug, Display},
    sync::atomic::AtomicUsize,
    sync::atomic::Ordering,
};

use crate::{
    core::uid::UID, entity_component_system::ComponentStorage,
    entity_component_system::EntityComponentDirectory, entity_component_system::SystemInterface,
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
pub struct SystemID {
    uid: UID,
    type_name: &'static str,
}

impl SystemID {
    pub fn next<T>() -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        SystemID {
            uid: COUNTER.fetch_add(1, Ordering::Relaxed),
            type_name: std::any::type_name::<T>(),
        }
    }

    pub fn get_name(&self) -> String {
        crate::core::type_name::strip_crate_names(self.type_name)
    }
}

impl Display for SystemID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:\t{}",
            self.uid,
            self.get_name()
        )
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
