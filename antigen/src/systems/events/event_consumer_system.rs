use std::{fmt::Debug, marker::PhantomData};

use crate::entity_component_system::system_interface::SystemInterface;
use crate::{
    components::EventQueue,
    entity_component_system::{
        ComponentStorage, EntityComponentDirectory, SystemDebugTrait, SystemError, SystemTrait,
    },
};

#[derive(Debug)]
pub struct EventConsumerSystem<T>
where
    T: Debug,
{
    _phantom_data: PhantomData<T>,
}

impl<T> EventConsumerSystem<T>
where
    T: Debug,
{
    pub fn new() -> Self {
        EventConsumerSystem {
            _phantom_data: PhantomData,
        }
    }
}

impl<T> Default for EventConsumerSystem<T>
where
    T: Debug,
{
    fn default() -> Self {
        EventConsumerSystem::<T>::new()
    }
}

impl<CS, CD, T> SystemTrait<CS, CD> for EventConsumerSystem<T>
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
    T: Debug + 'static,
{
    fn run(&mut self, db: &mut SystemInterface<CS, CD>) -> Result<(), SystemError>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
    {
        let event_queue_entities =
            db.entity_component_directory
                .get_entities_by_predicate(|entity_id| {
                    db.entity_component_directory
                        .entity_has_component::<EventQueue<T>>(entity_id)
                });

        for entity_id in event_queue_entities {
            let event_queue: &mut Vec<T> = db
                .get_entity_component_mut::<EventQueue<T>>(entity_id)?
                .as_mut();

            event_queue.clear();
        }

        Ok(())
    }
}

impl<T> SystemDebugTrait for EventConsumerSystem<T>
where
    T: Debug + 'static,
{
    fn get_name() -> &'static str {
        "Event Consumer"
    }
}
