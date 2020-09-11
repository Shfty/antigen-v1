use std::collections::HashMap;

use crate::{
    components::DebugEntityComponentListComponent, components::DebugEntityListComponent,
    components::EntityInspectorComponent, components::IntRangeComponent, ecs::ComponentData,
    ecs::ComponentDataID, ecs::ComponentID, ecs::EntityID,
};
use crate::{
    components::DebugExcludeComponent,
    components::StringListComponent,
    ecs::{
        EntityComponentDatabaseDebug, SystemEvent, {EntityComponentDatabase, SystemTrait},
    },
};

#[derive(Debug)]
pub struct ECSDebugSystem;

impl Default for ECSDebugSystem {
    fn default() -> Self {
        ECSDebugSystem
    }
}

impl ECSDebugSystem {
    pub fn new() -> Self {
        ECSDebugSystem::default()
    }
}

impl<T> SystemTrait<T> for ECSDebugSystem
where
    T: EntityComponentDatabase + EntityComponentDatabaseDebug,
{
    fn run(&mut self, db: &mut T) -> Result<SystemEvent, String>
    where
        T: EntityComponentDatabase + EntityComponentDatabaseDebug,
    {
        {
            // Populate entity list
            let entity_debug_entities = db.get_entities_by_predicate(|entity_id| {
                db.entity_has_component::<DebugEntityListComponent>(entity_id)
                    && db.entity_has_component::<StringListComponent>(entity_id)
            });

            for entity_id in entity_debug_entities {
                let mut entities: Vec<EntityID> = db
                    .get_entities()
                    .into_iter()
                    .filter(|entity_id| {
                        !db.entity_has_component::<DebugExcludeComponent>(entity_id)
                    })
                    .copied()
                    .collect();

                entities.sort();

                let entities: Vec<String> = entities
                    .into_iter()
                    .map(|entity_id| {
                        entity_id.to_string() + ": " + &db.get_entity_label(entity_id).to_string()
                    })
                    .collect();

                let debug_entity_list_component =
                    db.get_entity_component_mut::<StringListComponent>(entity_id)?;

                debug_entity_list_component.data = entities;
            }
        }

        let inspector_entities = db.get_entities_by_predicate(|entity_id| {
            db.entity_has_component::<EntityInspectorComponent>(entity_id)
        });
        assert!(inspector_entities.len() == 1);
        let inspector_entity = inspector_entities[0];
        let int_range_component = db.get_entity_component::<IntRangeComponent>(inspector_entity)?;
        let inspected_entity = int_range_component.index;

        if inspected_entity >= 0 {
            // Populate entity component
            let entity_component_debug_entities = db.get_entities_by_predicate(|entity_id| {
                db.entity_has_component::<DebugEntityComponentListComponent>(entity_id)
                    && db.entity_has_component::<StringListComponent>(entity_id)
            });

            let mut entity_components = db.get_entity_components();
            entity_components
                .sort_by(|(lhs_entity_id, _), (rhs_entity_id, _)| lhs_entity_id.cmp(rhs_entity_id));
            let (_, entity_components) = &entity_components[inspected_entity as usize];
            let entity_components: HashMap<&ComponentID, &ComponentDataID> = entity_components.iter().copied().collect();

            let component_data: HashMap<&ComponentDataID, &ComponentData> =
                db.get_component_data().into_iter().collect();

            let component_data: HashMap<&ComponentDataID, &ComponentData> = component_data
                .into_iter()
                .filter(|(component_data_id, _)| {
                    entity_components
                        .values()
                        .any(|candidate_id|**candidate_id == **component_data_id)
                })
                .collect();

            let mut components: Vec<String> = component_data
                .into_iter()
                .map(|(ComponentDataID(component_data_id), component_data)| format!("{}: {:#?}\n", component_data_id, component_data))
                .collect();

            components.sort();

            for entity_id in entity_component_debug_entities {
                let debug_entity_list_component =
                    db.get_entity_component_mut::<StringListComponent>(entity_id)?;

                debug_entity_list_component.data = components.clone();
            }
        }

        Ok(SystemEvent::None)
    }
}
