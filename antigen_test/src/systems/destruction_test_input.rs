use std::cell::Ref;

use antigen::{
    components::EventQueue,
    core::events::AntigenInputEvent,
    entity_component_system::ComponentData,
    entity_component_system::EntityComponentDirectory,
    entity_component_system::EntityID,
    entity_component_system::SystemError,
    entity_component_system::{system_interface::SystemInterface, SystemTrait},
};
use store::StoreQuery;

use crate::components::DestructionTestInputData;

#[derive(Debug)]
pub struct DestructionTestInput;

impl DestructionTestInput {
    pub fn new() -> Self {
        DestructionTestInput
    }
}

impl<CD> SystemTrait<CD> for DestructionTestInput
where
    CD: EntityComponentDirectory,
{
    fn run(&mut self, db: &mut SystemInterface<CD>) -> Result<(), SystemError>
    where
        CD: EntityComponentDirectory,
    {
        let entities_to_destroy: Vec<EntityID>;
        {
            let (_key, (event_queue,)) = StoreQuery::<
                EntityID,
                (Ref<ComponentData<EventQueue<AntigenInputEvent>>>,),
            >::iter(db.component_store)
            .next()
            .expect("No antigen input event queue");

            entities_to_destroy = StoreQuery::<
                EntityID,
                (Ref<ComponentData<DestructionTestInputData>>,),
            >::iter(db.component_store)
            .flat_map(|(entity_id, (destruction_test,))| {
                event_queue.iter().flat_map(move |event| {
                    if let AntigenInputEvent::KeyPress { key_code } = event {
                        if *key_code == ***destruction_test {
                            return Some(entity_id);
                        }
                    }

                    None
                })
            })
            .collect();
        }

        for entity_id in entities_to_destroy {
            db.destroy_entity(entity_id)?;
        }

        Ok(())
    }
}
