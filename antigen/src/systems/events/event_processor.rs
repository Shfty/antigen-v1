use std::{cell::Ref, fmt::Debug};

use store::StoreQuery;

use crate::{
    entity_component_system::{EntityID, SystemError, SystemTrait},
};
use crate::{components::Connection, entity_component_system::ComponentStore};

#[derive(Debug)]
pub struct EventProcessor;

impl SystemTrait for EventProcessor {
    fn run(&mut self, db: &mut ComponentStore) -> Result<(), SystemError> {
        for (entity_id, event_targets) in
            StoreQuery::<(EntityID, Ref<Connection>)>::iter(db.as_ref())
        {
            event_targets.run(db, entity_id);
        }

        Ok(())
    }
}
