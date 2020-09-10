use crate::{components::DebugEntityListComponent, ecs::EntityID};
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
        let entity_debug_entities = db.get_entities_by_predicate(|entity_id| {
            db.entity_has_component::<DebugEntityListComponent>(entity_id)
                && db.entity_has_component::<StringListComponent>(entity_id)
        });

        for entity_id in entity_debug_entities {
            let mut entities: Vec<EntityID> = db
                .get_entities()
                .into_iter()
                .filter(|entity_id| !db.entity_has_component::<DebugExcludeComponent>(entity_id))
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

        Ok(SystemEvent::None)
    }
}
