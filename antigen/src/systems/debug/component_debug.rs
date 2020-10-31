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

type DebugEntities = (EntityID, NoField<DebugExclude>);
type EntityInspectorEventQueue<'a> = (
    EntityID,
    Ref<'a, EventQueue<EntityInspectorEvent>>,
    Ref<'a, IntRange>,
);
type ComponentInspectorEntity<'a> = (
    EntityID,
    RefMut<'a, EventQueue<ComponentInspectorEvent>>,
    RefMut<'a, IntRange>,
);
type DebugComponentListEntity<'a> = (
    EntityID,
    Ref<'a, DebugComponentList>,
    RefMut<'a, Vec<String>>,
);

#[derive(Debug)]
pub struct ComponentDebug;

impl SystemTrait for ComponentDebug {
    fn run(&mut self, db: &mut ComponentStore) -> Result<(), SystemError> {
        self.process_events(db);
        self.populate_strings(db);
        Ok(())
    }
}

impl ComponentDebug {
    fn process_events(&mut self, db: &mut ComponentStore) {
        let debug_entity_count = StoreQuery::<DebugEntities>::iter(db.as_ref()).count();
        let (_, mut event_queue, mut int_range) =
            StoreQuery::<ComponentInspectorEntity>::iter(db.as_ref())
                .next()
                .expect("No component inspector entity");
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

    fn populate_strings(&mut self, db: &mut ComponentStore) {
        // Populate strings for debug component list entities
        let mut debug_entities: Vec<EntityID> = StoreQuery::<DebugEntities>::iter(db.as_ref())
            .map(|(entity_id, _)| entity_id)
            .collect();
        debug_entities.sort();

        let (_, _, int_range) = StoreQuery::<EntityInspectorEventQueue>::iter(db.as_ref())
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

            let (_, _, mut strings) = StoreQuery::<DebugComponentListEntity>::iter(db.as_ref())
                .next()
                .expect("No debug component list entity");

            *strings = component_strings;
        }
    }
}
