use store::{KeyBuilder, MapKeyBuilder};

use crate::entity_component_system::EntityID;

pub type ComponentBuilder = KeyBuilder<EntityID>;

pub trait MapComponentBuilder: MapKeyBuilder<EntityID> {}
impl<T> MapComponentBuilder for T where T: MapKeyBuilder<EntityID> {}
