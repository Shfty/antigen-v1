use crate::primitive_types::HashMap;
use std::cell::{Ref, RefMut};

use store::StoreQuery;

use crate::entity_component_system::{SystemError, SystemTrait};
use crate::{
    components::{GlobalPositionData, ParentEntity, Position},
    entity_component_system::ComponentStore,
    entity_component_system::EntityID,
};

type ReadParentEntity<'a> = (
    EntityID,
    Ref<'a, Position>,
    Ref<'a, ParentEntity>,
    Ref<'a, GlobalPositionData>,
);

type SumGlobalPosition<'a> = (EntityID, Ref<'a, Position>, RefMut<'a, GlobalPositionData>);

type ReadPosition<'a> = (EntityID, Ref<'a, Position>);

#[derive(Debug)]
pub struct GlobalPosition;

impl SystemTrait for GlobalPosition {
    fn run(&mut self, db: &mut ComponentStore) -> Result<(), SystemError> {
        let entity_parent_chains: HashMap<EntityID, Vec<EntityID>> =
            StoreQuery::<ReadParentEntity>::iter(db.as_ref())
                .map(|(entity_id, _, parent_entity, _)| {
                    let mut parent_chain: Vec<EntityID> = Vec::new();

                    let mut candidate_id = **parent_entity;

                    loop {
                        parent_chain.push(candidate_id);

                        if !db.contains_type_key::<GlobalPositionData>(&candidate_id) {
                            break;
                        }

                        match db.get::<ParentEntity>(&candidate_id) {
                            Some(parent_entity_component) => {
                                candidate_id = **parent_entity_component
                            }
                            None => break,
                        }
                    }

                    (entity_id, parent_chain)
                })
                .collect();

        for (entity_id, parent_chain) in entity_parent_chains {
            let (_, position, mut global_position) =
                StoreQuery::<SumGlobalPosition>::get(db.as_ref(), &entity_id);

            **global_position = **position;

            for (_, position) in StoreQuery::<ReadPosition>::iter_keys(db.as_ref(), &parent_chain) {
                **global_position += **position;
            }
        }

        Ok(())
    }
}
