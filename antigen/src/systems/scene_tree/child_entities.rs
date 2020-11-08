use crate::{
    assemblage::EntityBuilder,
    components::ChildEntitiesData,
    entity_component_system::EntityID,
    entity_component_system::{SystemError, SystemTrait},
};
use crate::{components::ParentEntity, entity_component_system::ComponentStore};

use store::{Disassemble, StoreQuery};

use std::{
    cell::{Ref, RefMut},
    collections::BTreeMap,
};

type ReadParentEntities<'a> = (EntityID, Ref<'a, ParentEntity>);
type MaybeReadChildEntities<'a> = (EntityID, Option<Ref<'a, ChildEntitiesData>>);
type WriteChildEntities<'a> = (EntityID, RefMut<'a, ChildEntitiesData>);

#[derive(Debug)]
pub struct ChildEntities;

impl SystemTrait for ChildEntities {
    fn run(&mut self, db: &mut ComponentStore) -> Result<(), SystemError> {
        // Add child entities data to parents that don't have it yet
        let entities_to_add: BTreeMap<EntityID, EntityBuilder> =
            StoreQuery::<ReadParentEntities>::iter(db.as_ref())
                .flat_map(|(_, parent_entity)| {
                    let parent_id: EntityID = **parent_entity;
                    let (_, child_entities) =
                        StoreQuery::<MaybeReadChildEntities>::get(db.as_ref(), &parent_id);

                    if child_entities.is_none() {
                        Some((
                            parent_id,
                            EntityBuilder::new().key_field(parent_id, ChildEntitiesData::default()),
                        ))
                    } else {
                        None
                    }
                })
                .collect();

        for (_, builder) in entities_to_add.into_iter() {
            builder.finish(db);
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

        let parent_child_counts: BTreeMap<EntityID, usize> =
            StoreQuery::<ReadParentEntities>::iter(db.as_ref())
                .map(|(_, parent_entity)| {
                    let parent_id: EntityID = **parent_entity;

                    let (_, child_entities) =
                        StoreQuery::<WriteChildEntities>::get(db.as_ref(), &parent_id);

                    (
                        parent_id,
                        child_entities
                            .iter()
                            .filter(|entity_id| db.contains_key(entity_id))
                            .count(),
                    )
                })
                .collect();

        for (parent_id, child_count) in parent_child_counts {
            if child_count == 0 {
                // Prune unneeded child entity data
                <(ChildEntitiesData,)>::disassemble(db, &parent_id);
            } else {
                // Prune invalid entities from existing child entitity data
                let (_, mut child_entities) =
                    StoreQuery::<WriteChildEntities>::get(db.as_ref(), &parent_id);

                **child_entities = child_entities
                    .iter()
                    .filter(|entity_id| db.contains_key(entity_id))
                    .copied()
                    .collect()
            }
        }

        Ok(())
    }
}
