use crate::{
    components::ChildEntitiesData,
    entity_component_system::ComponentStorage,
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

impl<CS, CD> SystemTrait<CS, CD> for ChildEntities
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
            let parent_id: EntityID = **db.get_entity_component::<ParentEntity>(entity_id)?;

            let child_entities: &mut Vec<EntityID> =
                match db.get_entity_component_mut::<ChildEntitiesData>(parent_id) {
                    Ok(child_entities) => child_entities.as_mut(),
                    Err(_) => db
                        .insert_entity_component(parent_id, ChildEntitiesData::default())?
                        .as_mut(),
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
                        .entity_has_component::<ChildEntitiesData>(entity_id)
                });

        for entity_id in entities_with_children {
            let valid_entities: Vec<EntityID> = db
                .get_entity_component::<ChildEntitiesData>(entity_id)?
                .iter()
                .filter(|entity_id| db.entity_component_directory.is_valid_entity(entity_id))
                .copied()
                .collect();

            if valid_entities.is_empty() {
                println!("No valid children, removing component");
                db.remove_component_from_entity::<ChildEntitiesData>(entity_id)?;
            } else {
                let child_entities: &mut Vec<EntityID> = db
                    .get_entity_component_mut::<ChildEntitiesData>(entity_id)?
                    .as_mut();
                *child_entities = valid_entities;
            }
        }

        Ok(())
    }
}
