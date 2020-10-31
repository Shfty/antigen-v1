use std::{cell::Ref, cell::RefMut, fmt::Debug};

use store::StoreQuery;

use crate::{
    components::EventQueue,
    entity_component_system::{EntityID, SystemError, SystemTrait},
};
use crate::{components::EventTargets, entity_component_system::ComponentStore};

type ReadOutputQueues<'a, T> = (EntityID, Ref<'a, EventQueue<T>>, Ref<'a, EventTargets>);
type WriteInputQueues<'a, T> = (EntityID, RefMut<'a, EventQueue<T>>);

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

impl<O, I> SystemTrait for EventProcessor<O, I>
where
    O: Debug + Copy + 'static,
    I: Debug + Copy + 'static,
{
    fn run(&mut self, db: &mut ComponentStore) -> Result<(), SystemError> {
        for (_, out_event_queue, event_targets) in
            StoreQuery::<ReadOutputQueues<O>>::iter(db.as_ref())
        {
            let mut events: Vec<I> = out_event_queue
                .iter()
                .copied()
                .flat_map(self.convert)
                .collect();

            let keys: Vec<EntityID> = (**event_targets)
                .iter()
                .copied()
                .filter(|entity_id| db.contains_type_key::<EventQueue<I>>(entity_id))
                .collect();

            for (_, mut in_event_queue) in
                StoreQuery::<WriteInputQueues<I>>::iter_keys(db.as_ref(), &keys)
            {
                in_event_queue.append(&mut events);
            }
        }

        Ok(())
    }
}
