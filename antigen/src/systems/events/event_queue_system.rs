use std::{borrow::BorrowMut, fmt::Debug, marker::PhantomData};

use crate::entity_component_system::system_interface::SystemInterface;
use crate::{
    components::EventQueue,
    entity_component_system::{
        ComponentStorage, EntityComponentDirectory, SystemDebugTrait, SystemError, SystemTrait,
    },
};

#[derive(Debug)]
pub struct EventQueueSystem<T>
where
    T: Debug,
{
    _phantom_data: PhantomData<T>,
}

impl<T> EventQueueSystem<T>
where
    T: Debug,
{
    pub fn new() -> Self {
        EventQueueSystem {
            _phantom_data: PhantomData,
        }
    }
}

impl<T> Default for EventQueueSystem<T>
where
    T: Debug,
{
    fn default() -> Self {
        EventQueueSystem::<T>::new()
    }
}

impl<CS, CD, T> SystemTrait<CS, CD> for EventQueueSystem<T>
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
        let event_queue_component =
            db.entity_component_directory
                .get_entity_by_predicate(|entity_id| {
                    db.entity_component_directory
                        .entity_has_component::<EventQueue<T>>(entity_id)
                });

        if event_queue_component.is_none() {
            let event_queue_entity = db.create_entity(Some(&format!(
                "Event Queue ({})",
                std::any::type_name::<T>()
            )))?;
            db.insert_entity_component(event_queue_entity, EventQueue::<T>::default())?;
        }

        let event_queue_entity = db
            .entity_component_directory
            .get_entity_by_predicate(|entity_id| {
                db.entity_component_directory
                    .entity_has_component::<EventQueue<T>>(entity_id)
            })
            .ok_or(format!(
                "Failed to get event queue entity for type {}",
                std::any::type_name::<T>()
            ))?;

        let event_queue: &mut Vec<T> = db
            .get_entity_component_mut::<EventQueue<T>>(event_queue_entity)?
            .borrow_mut();
        event_queue.clear();

        Ok(())
    }
}

impl<T> SystemDebugTrait for EventQueueSystem<T>
where
    T: Debug + 'static,
{
    fn get_name() -> &'static str {
        "Event Queue"
    }
}
