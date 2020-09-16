use std::{fmt::Debug, marker::PhantomData};

use crate::{
    components::ComponentDebugComponent, components::ComponentInspectorComponent,
    components::DebugComponentDataListComponent, components::DebugComponentListComponent,
    components::DebugEntityListComponent, components::DebugExcludeComponent,
    components::EntityDebugComponent, components::EntityInspectorComponent,
    components::IntRangeComponent,
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
        let mut entities: Vec<EntityID> = db.get_entities_by_predicate(|entity_id| {
            !db.entity_has_component::<DebugExcludeComponent>(entity_id)
        });
        entities.sort();

        if let Some(entity_debug_entity) = db.get_entity_by_predicate(|entity_id| {
            db.entity_has_component::<EntityDebugComponent>(entity_id)
        }) {
            let entity_debug_component =
                match db.get_entity_component::<EntityDebugComponent>(entity_debug_entity) {
                    Ok(entity_debug_component) => entity_debug_component,
                    Err(err) => return Err(err.into()),
                };

            let entity_strings: Vec<String> = entities
                .iter()
                .map(|entity_id| {
                    let label = entity_debug_component.get_label(entity_id);
                    format!("{}:\t{}", entity_id, label)
                })
                .collect();

            // Populate strings for debug entity list entities
            let debug_entity_list_entities = db.get_entities_by_predicate(|entity_id| {
                db.entity_has_component::<DebugEntityListComponent>(entity_id)
                    && db.entity_has_component::<StringListComponent>(entity_id)
            });

            for entity_id in debug_entity_list_entities {
                db.get_entity_component_mut::<StringListComponent>(entity_id)?
                    .set_data(entity_strings.clone());
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

            if let Some(inspected_entity) = entities.get(int_range_component.get_index() as usize) {
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

                            let component_data = db.get_component_data_dyn(&component_data_id)?;
                            let component_data_string =
                                format!("{}: {:#?}", component_data_id, component_data);

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
