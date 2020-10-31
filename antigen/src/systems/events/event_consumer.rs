use std::{cell::RefMut, fmt::Debug, marker::PhantomData};

use store::StoreQuery;

use crate::entity_component_system::{ComponentStore, EntityID};
use crate::{
    components::EventQueue,
    entity_component_system::{SystemError, SystemTrait},
};

#[derive(Debug)]
pub struct EventConsumer<T>
where
    T: Debug,
{
    _phantom_data: PhantomData<T>,
}

impl<T> Default for EventConsumer<T>
where
    T: Debug,
{
    fn default() -> Self {
        EventConsumer {
            _phantom_data: PhantomData,
        }
    }
}

impl<T> SystemTrait for EventConsumer<T>
where
    T: Debug + 'static,
{
    fn run(&mut self, db: &mut ComponentStore) -> Result<(), SystemError> {
        StoreQuery::<(EntityID, RefMut<EventQueue<T>>)>::iter(db.as_ref())
            .for_each(|(_key, mut event_queue)| event_queue.clear());

        Ok(())
    }
}
