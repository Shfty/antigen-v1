use std::{fmt::Debug, time::Duration};

use crate::{
    components::ChildEntitiesComponent, components::ComponentDebugComponent,
    components::ComponentInspectorComponent, components::DebugComponentDataListComponent,
    components::DebugComponentListComponent, components::DebugEntityListComponent,
    components::DebugExcludeComponent, components::DebugSceneTreeComponent,
    components::DebugSystemListComponent, components::EntityDebugComponent,
    components::EntityInspectorComponent, components::IntRangeComponent,
    components::ParentEntityComponent, components::SystemDebugComponent,
    entity_component_system::system_interface::SystemInterface,
    entity_component_system::ComponentStorage, entity_component_system::EntityComponentDirectory,
    entity_component_system::EntityID, entity_component_system::SystemDebugTrait,
    entity_component_system::SystemID,
};
use crate::{
    components::StringListComponent,
    entity_component_system::{SystemError, SystemTrait},
};

#[derive(Debug)]
pub struct ECSDebugSystem;

impl<CS, CD> SystemTrait<CS, CD> for ECSDebugSystem
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

            let root_entities: Vec<EntityID> = debug_entities
                .iter()
                .filter(|entity_id| {
                    !db.entity_component_directory
                        .entity_has_component::<ParentEntityComponent>(entity_id)
                })
                .copied()
                .collect();

            let mut scene_tree_strings: Vec<String> = Vec::new();

            fn traverse_tree<CS, CD>(
                db: &mut SystemInterface<CS, CD>,
                entity_id: &EntityID,
                entity_debug_entity: EntityID,
                scene_tree_strings: &mut Vec<String>,
                mut padding: Vec<String>,
            ) -> Result<(), String>
            where
                CS: ComponentStorage,
                CD: EntityComponentDirectory,
            {
                let entity_debug_component =
                    db.get_entity_component::<EntityDebugComponent>(entity_debug_entity)?;

                let depth = padding.len();

                let prefix: String = if depth == 0 {
                    "".to_string()
                } else {
                    padding.iter().cloned().collect::<String>() + " "
                };

                for string in padding.iter_mut() {
                    match string.as_str() {
                        "└" => *string = "  ".into(),
                        "├" => *string = "│ ".into(),
                        _ => (),
                    };
                }

                let label = entity_debug_component.get_label(entity_id);
                let label = format!("{}:\t{}{}", entity_id, &prefix, label);
                scene_tree_strings.push(label);

                if let Ok(child_entities_component) =
                    db.get_entity_component::<ChildEntitiesComponent>(*entity_id)
                {
                    let child_ids: Vec<EntityID> = child_entities_component.get_child_ids().clone();
                    let child_ids: Vec<EntityID> = child_ids
                        .iter()
                        .filter(|child_id| {
                            !db.entity_component_directory
                                .entity_has_component::<DebugExcludeComponent>(child_id)
                        })
                        .copied()
                        .collect();

                    for (i, child_entity) in child_ids.iter().enumerate() {
                        let mut padding = padding.clone();
                        padding.push(
                            if child_ids.len() == 1 || i == child_ids.len() - 1 {
                                "└"
                            } else {
                                "├"
                            }
                            .into(),
                        );
                        traverse_tree(
                            db,
                            child_entity,
                            entity_debug_entity,
                            scene_tree_strings,
                            padding,
                        )?;
                    }
                }

                Ok(())
            }

            // Populate strings for debug scene tree entities
            for root_entity in &root_entities {
                traverse_tree(
                    db,
                    root_entity,
                    entity_debug_entity,
                    &mut scene_tree_strings,
                    Vec::new(),
                )?;
            }

            let debug_scene_tree_entities = db
                .entity_component_directory
                .get_entities_by_predicate(|entity_id| {
                    db.entity_component_directory
                        .entity_has_component::<DebugSceneTreeComponent>(entity_id)
                        && db
                            .entity_component_directory
                            .entity_has_component::<StringListComponent>(entity_id)
                });

            for entity_id in debug_scene_tree_entities {
                db.get_entity_component_mut::<StringListComponent>(entity_id)?
                    .set_data(scene_tree_strings.clone());
            }
        }

        // Populate entity components list
        let entity_inspector_entity =
            db.entity_component_directory
                .get_entity_by_predicate(|entity_id| {
                    db.entity_component_directory
                        .entity_has_component::<EntityInspectorComponent>(entity_id)
                });

        // Populate strings for debug entity list entities
        let debug_component_list_entities = db
            .entity_component_directory
            .get_entities_by_predicate(|entity_id| {
                db.entity_component_directory
                    .entity_has_component::<DebugComponentListComponent>(entity_id)
                    && db
                        .entity_component_directory
                        .entity_has_component::<StringListComponent>(entity_id)
            });

        if let Some(entity_inspector_entity) = entity_inspector_entity {
            let int_range_component =
                db.get_entity_component::<IntRangeComponent>(entity_inspector_entity)?;

            if let Some(inspected_entity) =
                debug_entities.get(int_range_component.get_index() as usize)
            {
                let component_debug_entity =
                    db.entity_component_directory
                        .get_entity_by_predicate(|entity_id| {
                            db.entity_component_directory
                                .entity_has_component::<ComponentDebugComponent>(entity_id)
                        });

                if let Some(component_debug_entity) = component_debug_entity {
                    let mut components =
                        db.entity_component_directory
                            .get_components_by_predicate(|component_id| {
                                db.entity_component_directory
                                    .entity_has_component_by_id(inspected_entity, component_id)
                            });

                    let component_debug_component =
                        db.get_entity_component::<ComponentDebugComponent>(component_debug_entity)?;

                    components.sort_by(|lhs, rhs| {
                        let lhs_label = component_debug_component.get_label(lhs);
                        let rhs_label = component_debug_component.get_label(rhs);

                        lhs_label.cmp(&rhs_label)
                    });

                    let component_strings: Vec<String> = components
                        .iter()
                        .map(|component_id| component_debug_component.get_label(component_id))
                        .collect();

                    for entity_id in debug_component_list_entities {
                        db.get_entity_component_mut::<StringListComponent>(entity_id)?
                            .set_data(component_strings.clone());
                    }

                    let component_inspector_entity = db
                        .entity_component_directory
                        .get_entity_by_predicate(|entity_id| {
                            db.entity_component_directory
                                .entity_has_component::<ComponentInspectorComponent>(entity_id)
                        });

                    if let Some(component_inspector_entity) = component_inspector_entity {
                        let int_range_component = db.get_entity_component::<IntRangeComponent>(
                            component_inspector_entity,
                        )?;

                        if let Some(inspected_component) =
                            components.get(int_range_component.get_index() as usize)
                        {
                            let component_data_id =
                                db.entity_component_directory.get_entity_component_data_id(
                                    inspected_entity,
                                    inspected_component,
                                )?;

                            let component_data_string = db
                                .component_storage
                                .get_component_data_string(&component_data_id)?;
                            let component_data_string =
                                format!("{}: {}", component_data_id, component_data_string);

                            let entity_component_debug_entities = db
                                .entity_component_directory
                                .get_entities_by_predicate(|entity_id| {
                                    db.entity_component_directory
                                        .entity_has_component::<DebugComponentDataListComponent>(
                                            entity_id,
                                        )
                                        && db
                                            .entity_component_directory
                                            .entity_has_component::<StringListComponent>(entity_id)
                                });

                            for entity_id in entity_component_debug_entities {
                                db.get_entity_component_mut::<StringListComponent>(entity_id)?
                                    .set_data(vec![component_data_string.clone()]);
                            }
                        }
                    }
                }
            }
        }

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

impl SystemDebugTrait for ECSDebugSystem {
    fn get_name() -> &'static str {
        "ECS Debug"
    }
}
