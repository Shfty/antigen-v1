use std::{fmt::Debug, marker::PhantomData};

use crate::entity_component_system::system_interface::SystemInterface;
use crate::{
    components::EventQueueComponent,
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
        let entities = db
            .entity_component_directory
            .get_entities_by_predicate(|entity_id| {
                db.entity_component_directory
                    .entity_has_component::<EventQueueComponent<T>>(entity_id)
            });

        for entity_id in entities {
            db.get_entity_component_mut::<EventQueueComponent<T>>(entity_id)?
                .clear_events();
        }

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
