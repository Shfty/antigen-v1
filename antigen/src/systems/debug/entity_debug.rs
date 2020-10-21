use std::{cell::Ref, cell::RefMut, fmt::Debug};

use store::StoreQuery;

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

        if let Some((_, mut event_queue, mut int_range)) = StoreQuery::<(
            EntityID,
            RefMut<EventQueue<EntityInspectorEvent>>,
            RefMut<IntRange>,
        )>::iter(db.component_store)
        .next()
        {
            int_range.set_range(0..debug_entities.len() as i64);

            let event_queue: &mut Vec<EntityInspectorEvent> = event_queue.as_mut();
            for event in event_queue.drain(..) {
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

        if let Some((_, _, mut strings)) =
            StoreQuery::<(EntityID, Ref<DebugEntityList>, RefMut<Vec<String>>)>::iter(
                db.component_store,
            )
            .next()
        {
            *strings = entity_strings;
        }

        Ok(())
    }
}
