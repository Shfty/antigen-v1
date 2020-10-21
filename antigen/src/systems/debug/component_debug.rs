use std::cell::{Ref, RefMut};

use store::StoreQuery;

use crate::{
    components::DebugComponentList, components::DebugExclude, components::IntRange,
    entity_component_system::system_interface::SystemInterface,
    entity_component_system::ComponentID, entity_component_system::EntityComponentDirectory,
    entity_component_system::EntityID,
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

impl<CD> SystemTrait<CD> for ComponentDebug
where
    CD: EntityComponentDirectory,
{
    fn run(&mut self, db: &mut SystemInterface<CD>) -> Result<(), SystemError>
    where
        CD: EntityComponentDirectory,
    {
        // Fetch debugged entities
        let mut debug_entities: Vec<EntityID> = db
            .entity_component_directory
            .get_entities_by_predicate(|entity_id| {
                !db.entity_has_component::<DebugExclude>(entity_id)
            });
        debug_entities.sort();

        // Process component inspector events
        if let Some((_, mut event_queue, mut int_range)) = StoreQuery::<(
            EntityID,
            RefMut<EventQueue<ComponentInspectorEvent>>,
            RefMut<IntRange>,
        )>::iter(db.component_store)
        .next()
        {
            int_range.set_range(0..debug_entities.len() as i64);

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
        if let Some((_, _, int_range)) = StoreQuery::<(
            EntityID,
            Ref<EventQueue<EntityInspectorEvent>>,
            Ref<IntRange>,
        )>::iter(db.component_store)
        .next()
        {
            if let Some(inspected_entity) = debug_entities.get(int_range.get_index() as usize) {
                let mut components =
                    db.entity_component_directory
                        .get_components_by_predicate(|component_id| {
                            db.entity_component_directory
                                .entity_has_component_by_id(inspected_entity, component_id)
                        });

                components.sort_by_key(ComponentID::get_name);

                let component_strings: Vec<String> = components
                    .iter()
                    .map(|component_id| component_id.get_name())
                    .collect();

                for (_, _, mut strings) in
                    StoreQuery::<(EntityID, Ref<DebugComponentList>, RefMut<Vec<String>>)>::iter(
                        db.component_store,
                    )
                {
                    *strings = component_strings.clone();
                }
            }
        }

        Ok(())
    }
}
