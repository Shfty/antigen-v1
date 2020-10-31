use std::cell::Ref;

use antigen::{
    components::EventQueue,
    core::events::AntigenInputEvent,
    entity_component_system::ComponentStore,
    entity_component_system::EntityID,
    entity_component_system::{SystemError, SystemTrait},
};

use store::StoreQuery;

type ReadAntigenEventQueue<'a> = (EntityID, Ref<'a, EventQueue<AntigenInputEvent>>);

#[derive(Debug)]
pub struct QuitKey {
    key: antigen::core::keyboard::Key,
}

impl QuitKey {
    pub fn new(key: antigen::core::keyboard::Key) -> Self {
        QuitKey { key }
    }
}

impl SystemTrait for QuitKey {
    fn run(&mut self, db: &mut ComponentStore) -> Result<(), SystemError> {
        for (_key, event_queue) in StoreQuery::<ReadAntigenEventQueue>::iter(db.as_ref()) {
            for event in event_queue.iter() {
                if let AntigenInputEvent::KeyPress { key_code } = event {
                    if *key_code == self.key {
                        return Err(SystemError::Quit);
                    }
                }
            }
        }

        Ok(())
    }
}
