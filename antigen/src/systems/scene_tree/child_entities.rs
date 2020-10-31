use crate::primitive_types::HashMap;
use crate::{
    components::ChildEntitiesData,
    entity_component_system::EntityID,
    entity_component_system::{SystemError, SystemTrait},
};
use crate::{components::ParentEntity, entity_component_system::ComponentStore};

use store::{Assembler, Disassemble, StoreQuery};

use std::cell::{Ref, RefMut};

type ReadParentEntities<'a> = (EntityID, Ref<'a, ParentEntity>);
type MaybeReadChildEntities<'a> = (EntityID, Option<Ref<'a, ChildEntitiesData>>);
type WriteChildEntities<'a> = (EntityID, RefMut<'a, ChildEntitiesData>);

#[derive(Debug)]
pub struct ChildEntities;

impl SystemTrait for ChildEntities {
    fn run(&mut self, db: &mut ComponentStore) -> Result<(), SystemError> {
        // Add child entities data to parents that don't have it yet
        let mut entities_to_add: Vec<EntityID> =
            StoreQuery::<ReadParentEntities>::iter(db.as_ref())
                .flat_map(|(_, parent_entity)| {
                    let parent_id: EntityID = **parent_entity;
                    let (_, child_entities) =
                        StoreQuery::<MaybeReadChildEntities>::get(db.as_ref(), &parent_id);

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
            Assembler::new()
                .key(entity_id)
                .field(ChildEntitiesData::default())
                .finish(db);
        }

        // Add new child entities to parents' child entity data
        for (entity_id, parent_entity) in StoreQuery::<ReadParentEntities>::iter(db.as_ref()) {
            let parent_id: EntityID = **parent_entity;

            let (_, mut child_entities) =
                StoreQuery::<WriteChildEntities>::get(db.as_ref(), &parent_id);

            if !child_entities.contains(&entity_id) {
                child_entities.push(entity_id);
            }
        }

        // Prune child entity data that doesn't exist anymore
        let mut entities_to_update: HashMap<EntityID, Vec<EntityID>> = HashMap::default();
        let mut entities_to_remove: Vec<EntityID> = Vec::new();
        for (_, parent_entity) in StoreQuery::<ReadParentEntities>::iter(db.as_ref()) {
            let parent_id = **parent_entity;

            let (_, child_entities) =
                StoreQuery::<WriteChildEntities>::get(db.as_ref(), &parent_id);

            let valid_entities: Vec<EntityID> = child_entities
                .iter()
                .filter(|entity_id| db.contains_key(entity_id))
                .copied()
                .collect();

            if valid_entities.is_empty() {
                entities_to_remove.push(parent_id);
            } else {
                entities_to_update.insert(parent_id, valid_entities);
            }
        }

        for entity_id in entities_to_remove {
            <(ChildEntitiesData,)>::disassemble(db, &entity_id);
        }

        let (entity_ids_to_update, mut valid_keys_to_update): (Vec<EntityID>, Vec<Vec<EntityID>>) =
            entities_to_update.into_iter().unzip();

        valid_keys_to_update.sort_unstable();

        for ((_, mut child_entities), valid_entities) in
            StoreQuery::<WriteChildEntities>::iter_keys(db.as_ref(), &entity_ids_to_update)
                .zip(valid_keys_to_update.into_iter())
        {
            **child_entities = valid_entities
        }

        Ok(())
    }
}
