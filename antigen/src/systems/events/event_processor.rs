use std::{cell::Ref, cell::RefMut, fmt::Debug};

use store::StoreQuery;

use crate::{
    components::EventQueue,
    entity_component_system::{
        ComponentData, EntityComponentDirectory, EntityID, SystemError, SystemTrait,
    },
};
use crate::{components::EventTargets, entity_component_system::system_interface::SystemInterface};

#[derive(Debug)]
pub struct EventProcessor<O, I>
where
    O: Debug,
    I: Debug,
{
    convert: fn(O) -> Option<I>,
}

impl<O, I> EventProcessor<O, I>
where
    O: Debug,
    I: Debug,
{
    pub fn new(convert: fn(O) -> Option<I>) -> Self {
        EventProcessor { convert }
    }
}

impl<CD, O, I> SystemTrait<CD> for EventProcessor<O, I>
where
    CD: EntityComponentDirectory,
    O: Debug + Copy + 'static,
    I: Debug + Copy + 'static,
{
    fn run(&mut self, db: &mut SystemInterface<CD>) -> Result<(), SystemError>
    where
        CD: EntityComponentDirectory,
    {
        for (_, (out_event_queue, event_targets)) in StoreQuery::<
            EntityID,
            (
                Ref<ComponentData<EventQueue<O>>>,
                Ref<ComponentData<EventTargets>>,
            ),
        >::iter(db.component_store)
        {
            let mut events: Vec<I> = out_event_queue
                .iter()
                .copied()
                .flat_map(self.convert)
                .collect();

            let keys = (***event_targets)
                .iter()
                .copied()
                .filter(|entity_id| db.entity_has_component::<EventQueue<I>>(entity_id))
                .collect();

            for (_, (mut in_event_queue,)) in StoreQuery::<
                EntityID,
                (RefMut<ComponentData<EventQueue<I>>>,),
            >::iter_keys(db.component_store, keys)
            {
                in_event_queue.append(&mut events);
            }
        }

        Ok(())
    }
}
