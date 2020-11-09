use std::cell::Ref;

use antigen::{
    components::EventQueue,
    core::events::KeyPress,
    entity_component_system::EntityID,
    entity_component_system::SystemError,
    entity_component_system::{ComponentStore, SystemTrait},
};
use store::StoreQuery;

use crate::components::DestructionTestInputData;

type ReadDestructionTestInput<'a> = (EntityID, Ref<'a, DestructionTestInputData>);

#[derive(Debug)]
pub struct DestructionTestInput;

impl SystemTrait for DestructionTestInput {
    fn run(&mut self, db: &mut ComponentStore) -> Result<(), SystemError> {
        let entities_to_destroy: Vec<EntityID>;
        {
            let (_key, event_queue) =
                StoreQuery::<(EntityID, Ref<EventQueue<KeyPress>>)>::iter(db.as_ref())
                    .next()
                    .expect("No antigen input event queue");

            entities_to_destroy = StoreQuery::<ReadDestructionTestInput>::iter(db.as_ref())
                .flat_map(|(entity_id, destruction_test)| {
                    event_queue.iter().flat_map(move |event| {
                        if event.key_code == **destruction_test {
                            return Some(entity_id);
                        }
                        None
                    })
                })
                .collect();
        }

        for entity_id in entities_to_destroy {
            db.remove_key(&entity_id);
        }

        Ok(())
    }
}
