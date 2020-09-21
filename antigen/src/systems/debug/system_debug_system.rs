use std::{fmt::Debug, time::Duration};

use crate::{
    components::DebugSystemListComponent, components::SystemDebugComponent,
    entity_component_system::system_interface::SystemInterface,
    entity_component_system::ComponentStorage, entity_component_system::EntityComponentDirectory,
    entity_component_system::SystemDebugTrait, entity_component_system::SystemID,
};
use crate::{
    components::StringListComponent,
    entity_component_system::{SystemError, SystemTrait},
};

#[derive(Debug)]
pub struct SystemDebugSystem;

impl<CS, CD> SystemTrait<CS, CD> for SystemDebugSystem
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
                        .entity_has_component::<SystemDebugComponent>(entity_id)
                })
        {
            // Populate strings for debug system list entities
            let system_debug_component =
                match db.get_entity_component::<SystemDebugComponent>(system_debug_entity) {
                    Ok(system_debug_component) => system_debug_component,
                    Err(err) => return Err(err.into()),
                };

            let system_strings = system_debug_component.get_labels();
            let system_durations = system_debug_component.get_durations();
            let total_duration: Duration = system_durations.values().sum();

            let mut system_strings: Vec<(&SystemID, &String)> = system_strings.iter().collect();
            system_strings.sort_by(|(lhs_id, _), (rhs_id, _)| lhs_id.cmp(rhs_id));
            let mut system_strings: Vec<String> = system_strings
                .iter()
                .flat_map(|(system_id, system_name)| {
                    let duration = system_durations.get(system_id)?;

                    Some(format!(
                        "{}:\t{} ({}ms / {}us / {}ns)",
                        system_id,
                        system_name,
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
                        .entity_has_component::<DebugSystemListComponent>(entity_id)
                        && db
                            .entity_component_directory
                            .entity_has_component::<StringListComponent>(entity_id)
                });

            for entity_id in debug_system_list_entities {
                db.get_entity_component_mut::<StringListComponent>(entity_id)?
                    .set_data(system_strings.clone());
            }
        }

        Ok(())
    }
}

impl SystemDebugTrait for SystemDebugSystem {
    fn get_name() -> &'static str {
        "System Debug"
    }
}
