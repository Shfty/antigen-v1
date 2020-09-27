use std::borrow::{Borrow, BorrowMut};

use crate::{
    components::ChildEntities,
    entity_component_system::ComponentStorage,
    entity_component_system::EntityComponentDirectory,
    entity_component_system::EntityID,
    entity_component_system::SystemDebugTrait,
    entity_component_system::{SystemError, SystemTrait},
};
use crate::{components::ParentEntity, entity_component_system::system_interface::SystemInterface};

#[derive(Debug)]
pub struct ChildEntitiesSystem;

impl Default for ChildEntitiesSystem {
    fn default() -> Self {
        ChildEntitiesSystem
    }
}

impl ChildEntitiesSystem {
    pub fn new() -> Self {
        ChildEntitiesSystem::default()
    }
}

impl<CS, CD> SystemTrait<CS, CD> for ChildEntitiesSystem
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
{
    fn run(&mut self, db: &mut SystemInterface<CS, CD>) -> Result<(), SystemError>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
    {
        // Add existing children to their parent entities' children component
        let entities_with_parents =
            db.entity_component_directory
                .get_entities_by_predicate(|entity_id| {
                    db.entity_component_directory
                        .entity_has_component::<ParentEntity>(entity_id)
                });

        for entity_id in entities_with_parents {
            let parent_id: EntityID = (*db.get_entity_component::<ParentEntity>(entity_id)?).into();

            let child_entities: &mut Vec<EntityID> =
                match db.get_entity_component_mut::<ChildEntities>(parent_id) {
                    Ok(child_entities) => child_entities.borrow_mut(),
                    Err(_) => db
                        .insert_entity_component(parent_id, ChildEntities::default())?
                        .borrow_mut(),
                };

            if !child_entities.contains(&entity_id) {
                child_entities.push(entity_id);
            }
        }

        // Prune destroyed entities from existing children components
        let entities_with_children =
            db.entity_component_directory
                .get_entities_by_predicate(|entity_id| {
                    db.entity_component_directory
                        .entity_has_component::<ChildEntities>(entity_id)
                });

        for entity_id in entities_with_children {
            let valid_entities: &Vec<EntityID> = db
                .get_entity_component::<ChildEntities>(entity_id)?
                .borrow();

            let valid_entities: Vec<EntityID> = valid_entities
                .iter()
                .filter(|entity_id| db.entity_component_directory.is_valid_entity(entity_id))
                .copied()
                .collect();

            if valid_entities.is_empty() {
                println!("No valid children, removing component");
                db.remove_component_from_entity::<ChildEntities>(entity_id)?;
            } else {
                let child_entities: &mut Vec<EntityID> = db
                    .get_entity_component_mut::<ChildEntities>(entity_id)?
                    .borrow_mut();
                *child_entities = valid_entities;
            }
        }

        Ok(())
    }
}

impl SystemDebugTrait for ChildEntitiesSystem {
    fn get_name() -> &'static str {
        "Child Entities"
    }
}
