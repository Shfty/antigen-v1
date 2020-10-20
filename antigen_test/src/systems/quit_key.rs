use std::cell::Ref;

use antigen::{
    components::EventQueue,
    core::events::AntigenInputEvent,
    entity_component_system::system_interface::SystemInterface,
    entity_component_system::EntityComponentDirectory,
    entity_component_system::EntityID,
    entity_component_system::{SystemError, SystemTrait},
};

use store::StoreQuery;

#[derive(Debug)]
pub struct QuitKey {
    key: antigen::core::keyboard::Key,
}

impl QuitKey {
    pub fn new(key: antigen::core::keyboard::Key) -> Self {
        QuitKey { key }
    }
}

impl<CD> SystemTrait<CD> for QuitKey
where
    CD: EntityComponentDirectory,
{
    fn run(&mut self, db: &mut SystemInterface<CD>) -> Result<(), SystemError>
    where
        CD: EntityComponentDirectory,
    {
        for (_key, event_queue) in
            StoreQuery::<(EntityID, Ref<EventQueue<AntigenInputEvent>>)>::iter(db.component_store)
        {
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
