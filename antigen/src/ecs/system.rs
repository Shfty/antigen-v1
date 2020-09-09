use std::fmt::Debug;

use crate::primitive_types::UID;

use super::{EntityComponentSystem, EntityComponentSystemDebug};

#[derive(Debug, Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct SystemID(pub UID);

pub enum SystemEvent {
    None,
    Input,
    Quit,
}

pub trait SystemTrait<T>: Debug where T: EntityComponentSystem + EntityComponentSystemDebug {
    fn run(&mut self, ecs: &mut T) -> Result<SystemEvent, String> where T: EntityComponentSystem + EntityComponentSystemDebug;
}
