use store::{Assembler, MapAssembler};

use super::EntityID;

pub type EntityAssembler = Assembler<EntityID>;

pub trait MapEntityAssembler: MapAssembler<EntityID> {}

impl<T> MapEntityAssembler for T where T: MapAssembler<EntityID> {}
