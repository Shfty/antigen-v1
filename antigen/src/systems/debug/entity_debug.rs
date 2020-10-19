use std::{cell::RefMut, fmt::Debug};

use crate::{
    components::EventQueue,
    components::IntRange,
    components::Name,
    entity_component_system::{SystemError, SystemTrait},
};
use crate::{
    components::{DebugEntityList, DebugExclude},
    entity_component_system::{
        system_interface::SystemInterface, EntityComponentDirectory, EntityID,
    },
};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum EntityInspectorEvent {
    SetInspectedEntity(Option<usize>),
}

#[derive(Debug)]
pub struct EntityDebug;

impl<CD> SystemTrait<CD> for EntityDebug
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

        // Process entity inspector events
        let entity_inspector_entity =
            db.entity_component_directory
                .get_entity_by_predicate(|entity_id| {
                    db.entity_has_component::<EventQueue<EntityInspectorEvent>>(entity_id)
                        && db.entity_has_component::<IntRange>(entity_id)
                });

        if let Some(entity_inspector_entity) = entity_inspector_entity {
            let mut events: Vec<EntityInspectorEvent> = Vec::new();
            {
                let mut event_queue: RefMut<Vec<EntityInspectorEvent>> = RefMut::map(
                    db.get_entity_component_mut::<EventQueue<EntityInspectorEvent>>(
                        entity_inspector_entity,
                    )?,
                    |event_queue| &mut **event_queue,
                );

                events.append(event_queue.as_mut());
            }

            let mut int_range = db.get_entity_component_mut::<IntRange>(entity_inspector_entity)?;
            int_range.set_range(0..debug_entities.len() as i64);
            for event in events {
                let EntityInspectorEvent::SetInspectedEntity(index) = event;
                if let Some(index) = index {
                    int_range.set_index(index as i64);
                } else {
                    int_range.set_index(-1);
                }
            }
        }

        // Populate entity list
        let entity_strings: Vec<String> = debug_entities
            .iter()
            .map(|entity_id| {
                let label: String = match db.get_entity_component::<Name>(*entity_id) {
                    Ok(name) => (**name).clone(),
                    Err(_) => "Entity".into(),
                };
                format!("{}:\t{}", entity_id, label)
            })
            .collect();

        let debug_entity_list_entities =
            db.entity_component_directory
                .get_entities_by_predicate(|entity_id| {
                    db.entity_has_component::<DebugEntityList>(entity_id)
                        && db.entity_has_component::<Vec<String>>(entity_id)
                });

        for entity_id in debug_entity_list_entities {
            *db.get_entity_component_mut::<Vec<String>>(entity_id)? = entity_strings.clone();
        }

        Ok(())
    }
}
