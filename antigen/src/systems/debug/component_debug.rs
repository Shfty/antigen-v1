use std::cell::{Ref, RefMut};

use store::{NoField, StoreQuery};

use crate::{
    components::DebugComponentList, components::DebugExclude, components::IntRange,
    entity_component_system::ComponentStore, entity_component_system::EntityID,
};
use crate::{
    components::EventQueue,
    entity_component_system::{SystemError, SystemTrait},
};

use super::EntityInspectorEvent;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ComponentInspectorEvent {
    SetInspectedComponent(Option<usize>),
}

#[derive(Debug)]
pub struct ComponentDebug;

impl SystemTrait for ComponentDebug {
    fn run(&mut self, db: &mut ComponentStore) -> Result<(), SystemError> {
        // Process component inspector events
        let debug_entity_count =
            StoreQuery::<(EntityID, NoField<DebugExclude>)>::iter(db.as_ref()).count();

        if let Some((_, mut event_queue, mut int_range)) = StoreQuery::<(
            EntityID,
            RefMut<EventQueue<ComponentInspectorEvent>>,
            RefMut<IntRange>,
        )>::iter(db.as_ref())
        .next()
        {
            int_range.set_range(0..debug_entity_count as i64);

            let events: &mut Vec<ComponentInspectorEvent> = event_queue.as_mut();
            for event in events.drain(..) {
                let ComponentInspectorEvent::SetInspectedComponent(index) = event;
                if let Some(index) = index {
                    int_range.set_index(index as i64);
                } else {
                    int_range.set_index(-1);
                }
            }
        }

        // Populate strings for debug component list entities
        let mut debug_entities: Vec<EntityID> =
            StoreQuery::<(EntityID, NoField<DebugExclude>)>::iter(db.as_ref())
                .map(|(entity_id, _)| entity_id)
                .collect();
        debug_entities.sort();

        let (_, _, int_range) = StoreQuery::<(
            EntityID,
            Ref<EventQueue<EntityInspectorEvent>>,
            Ref<IntRange>,
        )>::iter(db.as_ref())
        .next()
        .expect("No entity inspector present");

        let index = int_range.get_index();
        if index >= 0 {
            let inspected_entity = &debug_entities[int_range.get_index() as usize];

            let mut component_strings: Vec<String> = db
                .iter_key_untyped(inspected_entity)
                .map(|(key, _)| key.get_name().to_string())
                .collect();

            component_strings.sort_unstable();

            for (_, _, mut strings) in
                StoreQuery::<(EntityID, Ref<DebugComponentList>, RefMut<Vec<String>>)>::iter(
                    db.as_ref(),
                )
            {
                *strings = component_strings.clone();
            }
        }

        Ok(())
    }
}
