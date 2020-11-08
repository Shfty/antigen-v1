use store::{MapStoreBuilder, StoreBuilder};

use crate::entity_component_system::EntityID;

pub type EntityBuilder = StoreBuilder<EntityID>;

pub trait MapEntityBuilder: MapStoreBuilder<EntityID> {}
impl<T> MapEntityBuilder for T where T: MapStoreBuilder<EntityID> {}
