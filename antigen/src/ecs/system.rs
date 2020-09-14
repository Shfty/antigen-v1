use std::fmt::Debug;

use crate::primitive_types::UID;

use super::EntityComponentDatabase;

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

pub trait SystemTrait<T>
where
    T: EntityComponentDatabase,
{
    fn run(&mut self, db: &mut T) -> Result<(), SystemError>
    where
        T: EntityComponentDatabase;
}
