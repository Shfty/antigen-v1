use crate::components::ParentEntityComponent;
use crate::{
    components::ChildEntitiesComponent,
    ecs::EntityID,
    ecs::{EntityComponentDatabase, SystemError, SystemTrait},
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

impl<T> SystemTrait<T> for ChildEntitiesSystem
where
    T: EntityComponentDatabase,
{
    fn run(&mut self, db: &mut T) -> Result<(), SystemError>
    where
        T: EntityComponentDatabase,
    {
        // Add existing children to their parent entities' children component
        let entities_with_parents = db.get_entities_by_predicate(|entity_id| {
            db.entity_has_component::<ParentEntityComponent>(entity_id)
        });

        for entity_id in entities_with_parents {
            let parent_id = db
                .get_entity_component::<ParentEntityComponent>(entity_id)?
                .get_parent_id();

            let child_entities_component = match db
                .get_entity_component_mut::<ChildEntitiesComponent>(parent_id)
            {
                Ok(child_entities_component) => child_entities_component,
                Err(_) => db.add_component_to_entity(parent_id, ChildEntitiesComponent::new())?,
            };

            if !child_entities_component.has_child_id(&entity_id) {
                child_entities_component.add_child_id(entity_id);
            }
        }

        // Prune destroyed entities from existing children components
        let entities_with_children = db.get_entities_by_predicate(|entity_id| {
            db.entity_has_component::<ChildEntitiesComponent>(entity_id)
        });

        for entity_id in entities_with_children {
            let valid_entities: Vec<EntityID> = db
                .get_entity_component::<ChildEntitiesComponent>(entity_id)?
                .get_child_ids()
                .iter()
                .copied()
                .filter(|entity_id| db.is_valid_entity(entity_id))
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
