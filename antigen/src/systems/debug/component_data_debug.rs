use std::cell::{Ref, RefMut};

use store::{NoField, StoreQuery};

use crate::{
    components::DebugComponentDataList, components::DebugExclude, components::IntRange,
    entity_component_system::ComponentStore, entity_component_system::EntityID,
};
use crate::{
    components::EventQueue,
    entity_component_system::{SystemError, SystemTrait},
};

use super::EntityInspectorEvent;

type DebugEntities = (EntityID, NoField<DebugExclude>);
type DebugComponentDataListEntity<'a> = (
    EntityID,
    Ref<'a, DebugComponentDataList>,
    RefMut<'a, Vec<String>>,
);
type EntityInspectorEntity<'a> = (
    EntityID,
    Ref<'a, EventQueue<EntityInspectorEvent>>,
    Ref<'a, IntRange>,
);

#[derive(Debug)]
pub struct ComponentDataDebug;

impl SystemTrait for ComponentDataDebug {
    fn run(&mut self, db: &mut ComponentStore) -> Result<(), SystemError> {
        let mut debug_entities: Vec<EntityID> = StoreQuery::<DebugEntities>::iter(db.as_ref())
            .map(|(entity_id, _)| entity_id)
            .collect();
        debug_entities.sort();

        // Populate strings for debug component list entities
        let (_, _, int_range) = StoreQuery::<EntityInspectorEntity>::iter(db.as_ref())
        .next()
        .expect("No entity inspector present");

        let index = int_range.get_index();
        if index >= 0 {
            let inspected_entity = &debug_entities[int_range.get_index() as usize];

            let mut component_strings: Vec<String> = db
                .iter_key_untyped(inspected_entity)
                .map(|(key, value)| format!("{:?}: {:#?}", key, value))
                .collect();

            component_strings.sort_unstable();

            let (_, _, mut strings) = StoreQuery::<DebugComponentDataListEntity>::iter(db.as_ref())
                .next()
                .expect("No debug component data list entity");

            *strings = component_strings;

            /*
            let (_, _, int_range) = StoreQuery::<(
                EntityID,
                Ref<EventQueue<ComponentInspectorEvent>>,
                Ref<IntRange>,
            )>::iter(db.as_ref())
            .next()
            .expect("No component inspector present");

            let index = int_range.get_index();
            if index >= 0 {
                let component_strings: Vec<String> = db
                    .component_store
                    .iter_key_untyped(inspected_entity)
                    .map(|(key, value)| format!("{:?}: {:#?}", key, value))
                    .collect();

                for (_, _, mut strings) in StoreQuery::<(
                    EntityID,
                    Ref<DebugComponentDataList>,
                    RefMut<Vec<String>>,
                )>::iter(db.as_ref())
                {
                    *strings = component_strings.clone();
                }
            }
            */
        }

        Ok(())
    }
}
