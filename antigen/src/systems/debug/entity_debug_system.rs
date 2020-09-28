use std::fmt::Debug;

use crate::{
    components::Name,
    entity_component_system::{SystemError, SystemTrait},
};
use crate::{
    components::{DebugEntityList, DebugExclude},
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
                    .entity_has_component::<DebugExclude>(entity_id)
            });
        debug_entities.sort();

        let entity_strings: Vec<String> = debug_entities
            .iter()
            .map(|entity_id| {
                let label: String = match db.get_entity_component::<Name>(*entity_id) {
                    Ok(name) => name.clone().into(),
                    Err(_) => "Entity".into(),
                };
                format!("{}:\t{}", entity_id, label)
            })
            .collect();

        let debug_entity_list_entities =
            db.entity_component_directory
                .get_entities_by_predicate(|entity_id| {
                    db.entity_component_directory
                        .entity_has_component::<DebugEntityList>(entity_id)
                        && db
                            .entity_component_directory
                            .entity_has_component::<Vec<String>>(entity_id)
                });

        for entity_id in debug_entity_list_entities {
            *db.get_entity_component_mut::<Vec<String>>(entity_id)? = entity_strings.clone();
        }

        Ok(())
    }
}

impl SystemDebugTrait for EntityDebugSystem {
    fn get_name() -> &'static str {
        "Entity Debug"
    }
}
