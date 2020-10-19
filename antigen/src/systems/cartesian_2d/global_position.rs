use std::{
    cell::{Ref, RefMut},
    collections::HashMap,
};

use store::StoreQuery;

use crate::entity_component_system::{EntityComponentDirectory, SystemError, SystemTrait};
use crate::{
    components::{GlobalPositionData, ParentEntity, Position},
    entity_component_system::system_interface::SystemInterface,
    entity_component_system::ComponentData,
    entity_component_system::EntityID,
};

#[derive(Debug)]
pub struct GlobalPosition;

impl<CD> SystemTrait<CD> for GlobalPosition
where
    CD: EntityComponentDirectory,
{
    fn run(&mut self, db: &mut SystemInterface<CD>) -> Result<(), SystemError>
    where
        CD: EntityComponentDirectory,
    {
        let entity_parent_chains: HashMap<EntityID, Vec<EntityID>> =
            StoreQuery::<
                EntityID,
                (
                    Ref<ComponentData<Position>>,
                    Ref<ComponentData<ParentEntity>>,
                    Ref<ComponentData<GlobalPositionData>>,
                ),
            >::iter(db.component_store)
            .map(|(entity_id, (_, parent_entity, _))| {
                let mut parent_chain: Vec<EntityID> = Vec::new();

                let mut candidate_id = ***parent_entity;

                loop {
                    parent_chain.push(candidate_id);

                    if !db.entity_has_component::<GlobalPositionData>(&candidate_id) {
                        break;
                    }

                    match db.get_entity_component::<ParentEntity>(candidate_id) {
                        Ok(parent_entity_component) => candidate_id = **parent_entity_component,
                        Err(_) => break,
                    }
                }

                (entity_id, parent_chain)
            })
            .collect();

        for (entity_id, parent_chain) in entity_parent_chains {
            let (position, mut global_position) =
                StoreQuery::<
                    EntityID,
                    (
                        Ref<ComponentData<Position>>,
                        RefMut<ComponentData<GlobalPositionData>>,
                    ),
                >::get(db.component_store, entity_id);

            ***global_position = ***position;

            for (_, (position,)) in
                StoreQuery::<EntityID, (Ref<ComponentData<Position>>,)>::iter_keys(
                    db.component_store,
                    parent_chain.clone(),
                )
            {
                ***global_position += ***position;
            }
        }

        Ok(())
    }
}
