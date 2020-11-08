use std::{cell::Ref, cell::RefMut, fmt::Debug, time::Duration};

use store::StoreQuery;

use crate::{
    components::{DebugSystemList, EventQueue, IntRange, SystemProfilingData},
    entity_component_system::{ComponentStore, EntityID, SystemError, SystemTrait},
};

type SystemProfilingEntity<'a> = (EntityID, RefMut<'a, SystemProfilingData>);
type SystemInspectorEventQueue<'a> = (
    EntityID,
    RefMut<'a, EventQueue<SystemInspectorEvent>>,
    RefMut<'a, IntRange>,
);
type DebugSystemListEntity<'a> = (EntityID, Ref<'a, DebugSystemList>, RefMut<'a, Vec<String>>);

#[derive(Debug)]
pub struct SystemDebug;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum SystemInspectorEvent {
    SetInspectedSystem(Option<usize>),
}

impl SystemTrait for SystemDebug {
    fn run(&mut self, db: &mut ComponentStore) -> Result<(), SystemError> {
        if let Some((_, mut system_profiling_data)) =
            StoreQuery::<SystemProfilingEntity>::iter(db.as_ref()).next()
        {
            // Populate strings for debug system list entities
            let system_durations = system_profiling_data.get_durations();
            let total_duration: Duration =
                system_durations.iter().map(|(_, duration)| duration).sum();

            // Process system inspector events
            if let Some((_, mut event_queue, mut int_range)) =
                StoreQuery::<SystemInspectorEventQueue>::iter(db.as_ref()).next()
            {
                int_range.set_range(0..system_durations.len() as i64);

                for event in event_queue.drain(..) {
                    let SystemInspectorEvent::SetInspectedSystem(index) = event;
                    if let Some(index) = index {
                        int_range.set_index(index as i64);
                    } else {
                        int_range.set_index(-1);
                    }
                }
            }

            // Compile system strings
            let mut system_strings: Vec<String> = system_durations
                .iter()
                .flat_map(|(system_id, duration)| {
                    Some(format!(
                        "{} ({}ms / {}us / {}ns)",
                        system_id.get_name(),
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
                StoreQuery::<DebugSystemListEntity>::iter(db.as_ref()).next()
            {
                *strings = system_strings;
            }

            system_profiling_data.clear_durations();
        }

        Ok(())
    }
}
