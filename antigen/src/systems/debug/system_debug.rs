use std::{cell::Ref, cell::RefMut, fmt::Debug, time::Duration};

use store::StoreQuery;

use crate::{
    components::{DebugSystemList, EventQueue, IntRange, SystemProfilingData},
    entity_component_system::{
        system_interface::SystemInterface, EntityComponentDirectory, EntityID, SystemError,
        SystemTrait,
    },
};

#[derive(Debug)]
pub struct SystemDebug;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum SystemInspectorEvent {
    SetInspectedSystem(Option<usize>),
}

impl<CD> SystemTrait<CD> for SystemDebug
where
    CD: EntityComponentDirectory,
{
    fn run(&mut self, db: &mut SystemInterface<CD>) -> Result<(), SystemError>
    where
        CD: EntityComponentDirectory,
    {
        if let Some((_, system_profiling_data)) =
            StoreQuery::<(EntityID, Ref<SystemProfilingData>)>::iter(db.component_store).next()
        {
            // Populate strings for debug system list entities
            let system_durations = system_profiling_data.get_durations();
            let total_duration: Duration = system_durations.values().sum();

            // Process system inspector events
            if let Some((_, event_queue, mut int_range)) = StoreQuery::<(
                EntityID,
                RefMut<EventQueue<SystemInspectorEvent>>,
                RefMut<IntRange>,
            )>::iter(db.component_store)
            .next()
            {
                int_range.set_range(0..system_durations.len() as i64);

                let events: &Vec<SystemInspectorEvent> = event_queue.as_ref();
                for event in events {
                    let SystemInspectorEvent::SetInspectedSystem(index) = event;
                    if let Some(index) = index {
                        int_range.set_index(*index as i64);
                    } else {
                        int_range.set_index(-1);
                    }
                }
            }

            // Compile system strings
            let mut system_strings: Vec<String> = system_durations
                .iter()
                .flat_map(|(system_id, _)| {
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

            if let Some((_, _, mut strings)) =
                StoreQuery::<(EntityID, Ref<DebugSystemList>, RefMut<Vec<String>>)>::iter(
                    db.component_store,
                )
                .next()
            {
                *strings = system_strings;
            }
        }

        Ok(())
    }
}
