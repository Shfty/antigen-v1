use store::TypeKey;

use crate::entity_component_system::SystemTrait;
use std::fmt::Debug;

#[derive(Default)]
pub struct SystemStore(Vec<(TypeKey, Box<dyn SystemTrait>)>);

impl Debug for SystemStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl SystemStore {
    pub fn insert_system<T>(&mut self, system: T) -> TypeKey
    where
        T: SystemTrait + 'static,
    {
        let id = TypeKey::of::<T>();
        self.0.push((id, Box::new(system)));
        id
    }

    pub fn iter(&mut self) -> impl Iterator<Item = (TypeKey, &mut (dyn SystemTrait + 'static))> {
        self.0
            .iter_mut()
            .map(|(system_id, system)| (*system_id, system.as_mut()))
    }
}
