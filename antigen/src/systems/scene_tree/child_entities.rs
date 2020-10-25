use std::{
    cell::{Ref, RefMut},
    collections::HashMap,
};

use store::StoreQuery;

use crate::{
    components::ChildEntitiesData,
    entity_component_system::EntityComponentDirectory,
    entity_component_system::EntityID,
    entity_component_system::{SystemError, SystemTrait},
};
use crate::{components::ParentEntity, entity_component_system::system_interface::SystemInterface};

#[derive(Debug)]
pub struct ChildEntities;

impl Default for ChildEntities {
    fn default() -> Self {
        ChildEntities
    }
}

impl ChildEntities {
    pub fn new() -> Self {
        ChildEntities::default()
    }
}

impl<CD> SystemTrait<CD> for ChildEntities
where
    CD: EntityComponentDirectory,
{
    fn run(&mut self, db: &mut SystemInterface<CD>) -> Result<(), SystemError>
    where
        CD: EntityComponentDirectory,
    {
        // Add child entities data to parents that don't have it yet
        let mut entities_to_add: Vec<EntityID> =
            StoreQuery::<(EntityID, Ref<ParentEntity>)>::iter(db.component_store)
                .flat_map(|(_, parent_entity)| {
                    let parent_id: EntityID = **parent_entity;
                    let (_, child_entities) =
                        StoreQuery::<(EntityID, Option<Ref<ChildEntitiesData>>)>::get(
                            db.component_store,
                            &parent_id,
                        );

                    if child_entities.is_none() {
                        Some(parent_id)
                    } else {
                        None
                    }
                })
                .collect();
        
        entities_to_add.sort_unstable();
        entities_to_add.dedup();

        for entity_id in entities_to_add {
            db.insert_entity_component(entity_id, ChildEntitiesData::default())?;
        }

        // Add new child entities to parents' child entity data
        for (entity_id, parent_entity) in
            StoreQuery::<(EntityID, Ref<ParentEntity>)>::iter(db.component_store)
        {
            let parent_id: EntityID = **parent_entity;

            let (_, mut child_entities) = StoreQuery::<(EntityID, RefMut<ChildEntitiesData>)>::get(
                db.component_store,
                &parent_id,
            );

            if !child_entities.contains(&entity_id) {
                child_entities.push(entity_id);
            }
        }

        // Prune child entity data that doesn't exist anymore
        let mut entities_to_update: HashMap<EntityID, Vec<EntityID>> = HashMap::new();
        let mut entities_to_remove: Vec<EntityID> = Vec::new();
        for (_, parent_entity) in
            StoreQuery::<(EntityID, Ref<ParentEntity>)>::iter(db.component_store)
        {
            let parent_id = **parent_entity;

            let (_, child_entities) = StoreQuery::<(EntityID, Ref<ChildEntitiesData>)>::get(
                db.component_store,
                &parent_id,
            );

            let valid_entities: Vec<EntityID> = child_entities
                .iter()
                .filter(|entity_id| db.is_valid_entity(entity_id))
                .copied()
                .collect();
            
            if valid_entities.is_empty() {
                entities_to_remove.push(parent_id);
            } else {
                entities_to_update.insert(parent_id, valid_entities);
            }
        }
        
        for entity_id in entities_to_remove {
            db.remove_component_from_entity::<ChildEntitiesData>(entity_id)?;
        }

        let (entity_ids_to_update, mut valid_keys_to_update): (Vec<EntityID>, Vec<Vec<EntityID>>) =
            entities_to_update.into_iter().unzip();

        valid_keys_to_update.sort_unstable();

        for ((_, mut child_entities), valid_entities) in
            StoreQuery::<(EntityID, RefMut<ChildEntitiesData>)>::iter_keys(
                db.component_store,
                &entity_ids_to_update,
            )
            .zip(valid_keys_to_update.into_iter())
        {
            **child_entities = valid_entities
        }

        Ok(())
    }
}
