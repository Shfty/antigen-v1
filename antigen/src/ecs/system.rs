use std::fmt::Debug;

use crate::primitive_types::UID;

use super::{EntityComponentDatabase, EntityComponentDatabaseDebug};

#[derive(Debug, Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct SystemID(pub UID);

pub enum SystemEvent {
    None,
    Input,
    Quit,
}

pub trait SystemTrait<T>: Debug where T: EntityComponentDatabase + EntityComponentDatabaseDebug {
    fn run(&mut self, db: &mut T) -> Result<SystemEvent, String> where T: EntityComponentDatabase + EntityComponentDatabaseDebug;
}
