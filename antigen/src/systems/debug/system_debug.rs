use std::{collections::HashMap, fmt::Debug, time::Duration};

use crate::{
    components::{DebugSystemList, EventQueue, IntRange, SystemProfilingData},
    entity_component_system::{
        system_interface::SystemInterface, ComponentStorage, EntityComponentDirectory, SystemError,
        SystemID, SystemTrait,
    },
};

#[derive(Debug)]
pub struct SystemDebug;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum SystemInspectorEvent {
    SetInspectedSystem(Option<usize>),
}

impl<CS, CD> SystemTrait<CS, CD> for SystemDebug
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
{
    fn run(&mut self, db: &mut SystemInterface<CS, CD>) -> Result<(), SystemError>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
    {
        if let Some(system_debug_entity) =
            db.entity_component_directory
                .get_entity_by_predicate(|entity_id| {
                    db.entity_component_directory
                        .entity_has_component::<SystemProfilingData>(entity_id)
                })
        {
            // Populate strings for debug system list entities
            let system_durations: HashMap<SystemID, Duration>;
            {
                let system_debug_component =
                    match db.get_entity_component::<SystemProfilingData>(system_debug_entity) {
                        Ok(system_debug_component) => system_debug_component,
                        Err(err) => return Err(err.into()),
                    };
                system_durations = system_debug_component.get_durations().clone();
            }
            let total_duration: Duration = system_durations.values().sum();

            // Process system inspector events
            let system_inspector_entity =
                db.entity_component_directory
                    .get_entity_by_predicate(|entity_id| {
                        db.entity_component_directory
                            .entity_has_component::<EventQueue<SystemInspectorEvent>>(entity_id)
                            && db
                                .entity_component_directory
                                .entity_has_component::<IntRange>(entity_id)
                    });

            if let Some(system_inspector_entity) = system_inspector_entity {
                let mut events: Vec<SystemInspectorEvent> = Vec::new();
                {
                    let event_queue: &mut Vec<SystemInspectorEvent> = db
                        .get_entity_component_mut::<EventQueue<SystemInspectorEvent>>(
                            system_inspector_entity,
                        )?;

                    events.append(event_queue);
                }

                let int_range = db.get_entity_component_mut::<IntRange>(system_inspector_entity)?;
                int_range.set_range(0..system_durations.len() as i64);
                for event in events {
                    let SystemInspectorEvent::SetInspectedSystem(index) = event;
                    if let Some(index) = index {
                        int_range.set_index(index as i64);
                    } else {
                        int_range.set_index(-1);
                    }
                }
            }

            // Compile system strings
            let mut system_ids: Vec<&SystemID> = system_durations.keys().collect();
            system_ids.sort();
            let mut system_strings: Vec<String> = system_ids
                .iter()
                .flat_map(|system_id| {
                    let duration = system_durations.get(system_id)?;

                    Some(format!(
                        "{} ({}ms / {}us / {}ns)",
                        system_id,
                        duration.as_millis(),
                        duration.as_micros(),
                        duration.as_nanos(),
                    ))
                })
                .collect();

            system_strings.push("".into());

            system_strings.push(format!(
                "Total: {}ms / {}us / {}ns",
                total_duration.as_millis(),
                total_duration.as_micros(),
                total_duration.as_nanos(),
            ));

            let debug_system_list_entities = db
                .entity_component_directory
                .get_entities_by_predicate(|entity_id| {
                    db.entity_component_directory
                        .entity_has_component::<DebugSystemList>(entity_id)
                        && db
                            .entity_component_directory
                            .entity_has_component::<Vec<String>>(entity_id)
                });

            for entity_id in debug_system_list_entities {
                *db.get_entity_component_mut::<Vec<String>>(entity_id)? = system_strings.clone();
            }
        }

        Ok(())
    }
}
