use std::fmt::Debug;

use crate::{
    components::StringListComponent,
    entity_component_system::{SystemError, SystemTrait},
};
use crate::{
    components::{DebugEntityListComponent, DebugExcludeComponent, EntityDebugComponent},
    entity_component_system::{
        system_interface::SystemInterface, ComponentStorage, EntityComponentDirectory, EntityID,
        SystemDebugTrait,
    },
};

#[derive(Debug)]
pub struct EntityDebugSystem;

impl<CS, CD> SystemTrait<CS, CD> for EntityDebugSystem
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
{
    fn run(&mut self, db: &mut SystemInterface<CS, CD>) -> Result<(), SystemError>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
    {
        let mut debug_entities: Vec<EntityID> = db
            .entity_component_directory
            .get_entities_by_predicate(|entity_id| {
                !db.entity_component_directory
                    .entity_has_component::<DebugExcludeComponent>(entity_id)
            });
        debug_entities.sort();

        if let Some(entity_debug_entity) =
            db.entity_component_directory
                .get_entity_by_predicate(|entity_id| {
                    db.entity_component_directory
                        .entity_has_component::<EntityDebugComponent>(entity_id)
                })
        {
            // Populate strings for debug entity list entities
            let entity_debug_component =
                match db.get_entity_component::<EntityDebugComponent>(entity_debug_entity) {
                    Ok(entity_debug_component) => entity_debug_component,
                    Err(err) => return Err(err.into()),
                };

            let entity_strings: Vec<String> = debug_entities
                .iter()
                .map(|entity_id| {
                    let label = entity_debug_component.get_label(entity_id);
                    format!("{}:\t{}", entity_id, label)
                })
                .collect();

            let debug_entity_list_entities = db
                .entity_component_directory
                .get_entities_by_predicate(|entity_id| {
                    db.entity_component_directory
                        .entity_has_component::<DebugEntityListComponent>(entity_id)
                        && db
                            .entity_component_directory
                            .entity_has_component::<StringListComponent>(entity_id)
                });

            for entity_id in debug_entity_list_entities {
                db.get_entity_component_mut::<StringListComponent>(entity_id)?
                    .set_data(entity_strings.clone());
            }
        }

        Ok(())
    }
}

impl SystemDebugTrait for EntityDebugSystem {
    fn get_name() -> &'static str {
        "Entity Debug"
    }
}
