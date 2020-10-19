use std::cell::RefMut;

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
        let component_inspector_entity =
            db.entity_component_directory
                .get_entity_by_predicate(|entity_id| {
                    db.entity_has_component::<EventQueue<ComponentInspectorEvent>>(entity_id)
                        && db.entity_has_component::<IntRange>(entity_id)
                });

        if let Some(component_inspector_entity) = component_inspector_entity {
            let mut events: Vec<ComponentInspectorEvent> = Vec::new();
            {
                let mut event_queue: RefMut<Vec<ComponentInspectorEvent>> = RefMut::map(
                    db.get_entity_component_mut::<EventQueue<ComponentInspectorEvent>>(
                        component_inspector_entity,
                    )?,
                    |event_queue| &mut **event_queue,
                );

                events.append(event_queue.as_mut());
            }

            let mut int_range =
                db.get_entity_component_mut::<IntRange>(component_inspector_entity)?;
            int_range.set_range(0..debug_entities.len() as i64);
            for event in events {
                let ComponentInspectorEvent::SetInspectedComponent(index) = event;
                if let Some(index) = index {
                    int_range.set_index(index as i64);
                } else {
                    int_range.set_index(-1);
                }
            }
        }

        // Populate entity components list
        let entity_inspector_entity =
            db.entity_component_directory
                .get_entity_by_predicate(|entity_id| {
                    db.entity_has_component::<EventQueue<EntityInspectorEvent>>(entity_id)
                        && db.entity_has_component::<IntRange>(entity_id)
                });

        // Populate strings for debug component list entities
        let debug_component_list_entities = db
            .entity_component_directory
            .get_entities_by_predicate(|entity_id| {
                db.entity_has_component::<DebugComponentList>(entity_id)
                    && db.entity_has_component::<Vec<String>>(entity_id)
            });

        if let Some(entity_inspector_entity) = entity_inspector_entity {
            let int_range_index = db
                .get_entity_component::<IntRange>(entity_inspector_entity)?
                .get_index();

            if let Some(inspected_entity) = debug_entities.get(int_range_index as usize) {
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

                for entity_id in debug_component_list_entities {
                    *db.get_entity_component_mut::<Vec<String>>(entity_id)? =
                        component_strings.clone();
                }
            }
        }

        Ok(())
    }
}
