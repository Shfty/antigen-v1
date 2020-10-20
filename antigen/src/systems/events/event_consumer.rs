use std::{cell::RefMut, fmt::Debug, marker::PhantomData};

use store::StoreQuery;

use crate::entity_component_system::{system_interface::SystemInterface, EntityID};
use crate::{
    components::EventQueue,
    entity_component_system::{EntityComponentDirectory, SystemError, SystemTrait},
};

#[derive(Debug)]
pub struct EventConsumer<T>
where
    T: Debug,
{
    _phantom_data: PhantomData<T>,
}

impl<T> EventConsumer<T>
where
    T: Debug,
{
    pub fn new() -> Self {
        EventConsumer {
            _phantom_data: PhantomData,
        }
    }
}

impl<T> Default for EventConsumer<T>
where
    T: Debug,
{
    fn default() -> Self {
        EventConsumer::<T>::new()
    }
}

impl<CD, T> SystemTrait<CD> for EventConsumer<T>
where
    CD: EntityComponentDirectory,
    T: Debug + 'static,
{
    fn run(&mut self, db: &mut SystemInterface<CD>) -> Result<(), SystemError>
    where
        CD: EntityComponentDirectory,
    {
        StoreQuery::<(EntityID, RefMut<EventQueue<T>>)>::iter(db.component_store)
            .for_each(|(_key, mut event_queue)| event_queue.clear());

        Ok(())
    }
}
