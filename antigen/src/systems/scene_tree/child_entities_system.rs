use crate::{
    components::ChildEntitiesComponent,
    entity_component_system::ComponentStorage,
    entity_component_system::EntityComponentDirectory,
    entity_component_system::EntityID,
    entity_component_system::{SystemError, SystemTrait},
entity_component_system::SystemDebugTrait};
use crate::{
    components::ParentEntityComponent, entity_component_system::system_interface::SystemInterface,
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
                        .entity_has_component::<ParentEntityComponent>(entity_id)
                });

        for entity_id in entities_with_parents {
            let parent_id = db
                .get_entity_component::<ParentEntityComponent>(entity_id)?
                .get_parent_id();

            let child_entities_component = match db
                .get_entity_component_mut::<ChildEntitiesComponent>(parent_id)
            {
                Ok(child_entities_component) => child_entities_component,
                Err(_) => db.insert_entity_component(parent_id, ChildEntitiesComponent::new())?,
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
            let valid_entities: Vec<EntityID> = db
                .get_entity_component::<ChildEntitiesComponent>(entity_id)?
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
                db.remove_component_from_entity::<ChildEntitiesComponent>(entity_id)?;
            } else {
                db.get_entity_component_mut::<ChildEntitiesComponent>(entity_id)?
                    .set_child_ids(valid_entities);
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
