use std::{fmt::Debug, marker::PhantomData};

use crate::{
    components::ChildEntitiesComponent, components::ComponentDebugComponent,
    components::ComponentInspectorComponent, components::DebugComponentDataListComponent,
    components::DebugComponentListComponent, components::DebugEntityListComponent,
    components::DebugExcludeComponent, components::DebugSceneTreeComponent,
    components::EntityDebugComponent, components::EntityInspectorComponent,
    components::IntRangeComponent, components::ParentEntityComponent,
    entity_component_system::entity_component_database::ComponentStorage,
    entity_component_system::entity_component_database::EntityComponentDatabase,
    entity_component_system::entity_component_database::EntityComponentDirectory,
    entity_component_system::ComponentID, entity_component_system::EntityID,
};
use crate::{
    components::StringListComponent,
    entity_component_system::{SystemError, SystemTrait},
};

#[derive(Debug)]
pub struct ECSDebugSystem<T> {
    phantom: PhantomData<T>,
}

impl<T> ECSDebugSystem<T>
where
    T: EntityComponentDirectory,
{
    pub fn new<S, D>(db: &mut EntityComponentDatabase<S, D>) -> Self
    where
        S: ComponentStorage,
        D: EntityComponentDirectory,
    {
        fn entity_created<S, D>(
            db: &mut EntityComponentDatabase<S, D>,
            entity_id: EntityID,
            debug_label: Option<&str>,
        ) where
            S: ComponentStorage,
            D: EntityComponentDirectory,
        {
            if let Some(debug_label) = debug_label {
                if let Some(entity_debug_entity) = db.get_entity_by_predicate(|entity_id| {
                    db.entity_has_component::<EntityDebugComponent>(entity_id)
                }) {
                    if let Ok(entity_debug_component) =
                        db.get_entity_component_mut::<EntityDebugComponent>(entity_debug_entity)
                    {
                        entity_debug_component.register_entity(entity_id, debug_label.into());
                    }
                }
            }
        };

        fn component_created<S, D>(
            db: &mut EntityComponentDatabase<S, D>,
            component_id: ComponentID,
            label: &str,
            description: &str,
        ) where
            S: ComponentStorage,
            D: EntityComponentDirectory,
        {
            if let Some(component_debug_entity) = db.get_entity_by_predicate(|entity_id| {
                db.entity_has_component::<ComponentDebugComponent>(entity_id)
            }) {
                if let Ok(component_debug_component) =
                    db.get_entity_component_mut::<ComponentDebugComponent>(component_debug_entity)
                {
                    component_debug_component.register_component(
                        component_id,
                        label.into(),
                        description.into(),
                    );
                }
            }
        }

        db.register_entity_create_callback(entity_created);
        db.register_component_create_callback(component_created);

        ECSDebugSystem {
            phantom: PhantomData,
        }
    }
}

impl<S, D> SystemTrait<S, D> for ECSDebugSystem<D>
where
    S: ComponentStorage,
    D: EntityComponentDirectory,
{
    fn run(&mut self, db: &mut EntityComponentDatabase<S, D>) -> Result<(), SystemError>
    where
        S: ComponentStorage,
        D: EntityComponentDirectory,
    {
        let mut debug_entities: Vec<EntityID> = db.get_entities_by_predicate(|entity_id| {
            !db.entity_has_component::<DebugExcludeComponent>(entity_id)
        });
        debug_entities.sort();

        if let Some(entity_debug_entity) = db.get_entity_by_predicate(|entity_id| {
            db.entity_has_component::<EntityDebugComponent>(entity_id)
        }) {
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

            let debug_entity_list_entities = db.get_entities_by_predicate(|entity_id| {
                db.entity_has_component::<DebugEntityListComponent>(entity_id)
                    && db.entity_has_component::<StringListComponent>(entity_id)
            });

            for entity_id in debug_entity_list_entities {
                db.get_entity_component_mut::<StringListComponent>(entity_id)?
                    .set_data(entity_strings.clone());
            }

            // Populate strings for debug scene tree entities
            let entity_debug_component =
                match db.get_entity_component::<EntityDebugComponent>(entity_debug_entity) {
                    Ok(entity_debug_component) => entity_debug_component,
                    Err(err) => return Err(err.into()),
                };

            let root_entities: Vec<EntityID> = debug_entities
                .iter()
                .filter(|entity_id| !db.entity_has_component::<ParentEntityComponent>(entity_id))
                .copied()
                .collect();

            let mut scene_tree_strings: Vec<String> = Vec::new();

            fn traverse_tree<S, D>(
                db: &EntityComponentDatabase<S, D>,
                entity_debug_component: &EntityDebugComponent,
                entity_id: &EntityID,
                scene_tree_strings: &mut Vec<String>,
                mut padding: Vec<String>,
            ) where
                S: ComponentStorage,
                D: EntityComponentDirectory,
            {
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
                    let child_ids: Vec<EntityID> = child_entities_component
                        .get_child_ids()
                        .iter()
                        .filter(|child_id| {
                            !db.entity_has_component::<DebugExcludeComponent>(child_id)
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
                            entity_debug_component,
                            child_entity,
                            scene_tree_strings,
                            padding,
                        );
                    }
                }
            }

            for root_entity in &root_entities {
                traverse_tree(
                    db,
                    entity_debug_component,
                    root_entity,
                    &mut scene_tree_strings,
                    Vec::new(),
                );
            }

            let debug_scene_tree_entities = db.get_entities_by_predicate(|entity_id| {
                db.entity_has_component::<DebugSceneTreeComponent>(entity_id)
                    && db.entity_has_component::<StringListComponent>(entity_id)
            });

            for entity_id in debug_scene_tree_entities {
                db.get_entity_component_mut::<StringListComponent>(entity_id)?
                    .set_data(scene_tree_strings.clone());
            }
        }

        // Populate entity components list
        let entity_inspector_entity = db.get_entity_by_predicate(|entity_id| {
            db.entity_has_component::<EntityInspectorComponent>(entity_id)
        });

        // Populate strings for debug entity list entities
        let debug_component_list_entities = db.get_entities_by_predicate(|entity_id| {
            db.entity_has_component::<DebugComponentListComponent>(entity_id)
                && db.entity_has_component::<StringListComponent>(entity_id)
        });

        if let Some(entity_inspector_entity) = entity_inspector_entity {
            let int_range_component =
                db.get_entity_component::<IntRangeComponent>(entity_inspector_entity)?;

            if let Some(inspected_entity) =
                debug_entities.get(int_range_component.get_index() as usize)
            {
                let component_debug_entity = db.get_entity_by_predicate(|entity_id| {
                    db.entity_has_component::<ComponentDebugComponent>(entity_id)
                });

                if let Some(component_debug_entity) = component_debug_entity {
                    let mut components = db.get_components_by_predicate(|component_id| {
                        db.entity_has_component_by_id(inspected_entity, component_id)
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

                    let component_inspector_entity = db.get_entity_by_predicate(|entity_id| {
                        db.entity_has_component::<ComponentInspectorComponent>(entity_id)
                    });

                    if let Some(component_inspector_entity) = component_inspector_entity {
                        let int_range_component = db.get_entity_component::<IntRangeComponent>(
                            component_inspector_entity,
                        )?;

                        if let Some(inspected_component) =
                            components.get(int_range_component.get_index() as usize)
                        {
                            let component_data_id = db.get_entity_component_data_id(
                                inspected_entity,
                                inspected_component,
                            )?;

                            let component_data_string =
                                db.get_component_data_string(&component_data_id)?;
                            let component_data_string =
                                format!("{}: {}", component_data_id, component_data_string);

                            let entity_component_debug_entities =
                                db.get_entities_by_predicate(|entity_id| {
                                    db.entity_has_component::<DebugComponentDataListComponent>(
                                        entity_id,
                                    ) && db.entity_has_component::<StringListComponent>(entity_id)
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

        Ok(())
    }
}
