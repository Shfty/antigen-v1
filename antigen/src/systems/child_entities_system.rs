use crate::{
    components::ChildEntitiesComponent,
    entity_component_system::entity_component_database::ComponentStorage,
    entity_component_system::entity_component_database::EntityComponentDirectory,
    entity_component_system::get_entity_component,
    entity_component_system::get_entity_component_mut,
    entity_component_system::insert_entity_component,
    entity_component_system::remove_component_from_entity,
    entity_component_system::EntityID,
    entity_component_system::{SystemError, SystemTrait},
};
use crate::{
    components::ParentEntityComponent,
    entity_component_system::entity_component_database::EntityComponentDatabase,
};

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
    fn run(&mut self, db: &mut EntityComponentDatabase<CS, CD>) -> Result<(), SystemError>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
    {
        // Add existing children to their parent entities' children component
        let entities_with_parents =
            db.entity_component_directory
                .get_entities_by_predicate(|entity_id| {
                    db.entity_component_directory
                        .entity_has_component::<ParentEntityComponent>(entity_id)
                });

        for entity_id in entities_with_parents {
            let parent_id = get_entity_component::<CS, CD, ParentEntityComponent>(
                &db.component_storage,
                &db.entity_component_directory,
                entity_id,
            )?
            .get_parent_id();

            let child_entities_component =
                match get_entity_component_mut::<CS, CD, ChildEntitiesComponent>(
                    &mut db.component_storage,
                    &mut db.entity_component_directory,
                    parent_id,
                ) {
                    Ok(child_entities_component) => child_entities_component,
                    Err(_) => insert_entity_component(
                        &mut db.component_storage,
                        &mut db.entity_component_directory,
                        parent_id,
                        ChildEntitiesComponent::new(),
                    )?,
                };

            if !child_entities_component.has_child_id(&entity_id) {
                child_entities_component.add_child_id(entity_id);
            }
        }

        // Prune destroyed entities from existing children components
        let entities_with_children =
            db.entity_component_directory
                .get_entities_by_predicate(|entity_id| {
                    db.entity_component_directory
                        .entity_has_component::<ChildEntitiesComponent>(entity_id)
                });

        for entity_id in entities_with_children {
            let valid_entities: Vec<EntityID> =
                get_entity_component::<CS, CD, ChildEntitiesComponent>(
                    &db.component_storage,
                    &db.entity_component_directory,
                    entity_id,
                )?
                .get_child_ids()
                .iter()
                .copied()
                .collect();

            let valid_entities: Vec<EntityID> = valid_entities
                .iter()
                .filter(|entity_id| db.entity_component_directory.is_valid_entity(entity_id))
                .copied()
                .collect();

            if valid_entities.is_empty() {
                println!("No valid children, removing component");
                remove_component_from_entity::<CS, CD, ChildEntitiesComponent>(
                    &mut db.component_storage,
                    &mut db.entity_component_directory,
                    entity_id,
                )?;
            } else {
                get_entity_component_mut::<CS, CD, ChildEntitiesComponent>(
                    &mut db.component_storage,
                    &mut db.entity_component_directory,
                    entity_id,
                )?
                .set_child_ids(valid_entities);
            }
        }

        Ok(())
    }
}
