use std::ops::{Deref, DerefMut};

use store::Store;

use crate::entity_component_system::EntityID;

#[derive(Debug, Default)]
pub struct ComponentStore(pub Store<EntityID>);

impl Deref for ComponentStore {
    type Target = Store<EntityID>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ComponentStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<Store<EntityID>> for ComponentStore {
    fn as_ref(&self) -> &Store<EntityID> {
        &self.0
    }
}

impl<'a> AsMut<Store<EntityID>> for ComponentStore {
    fn as_mut(&mut self) -> &mut Store<EntityID> {
        &mut self.0
    }
}
