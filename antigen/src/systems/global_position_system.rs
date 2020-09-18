use crate::entity_component_system::{
    entity_component_database::ComponentStorage,
    entity_component_database::EntityComponentDirectory, SystemError, SystemTrait,
};
use crate::{
    components::{GlobalPositionComponent, ParentEntityComponent, PositionComponent},
    entity_component_system::entity_component_database::EntityComponentDatabase,
};

#[derive(Debug)]
pub struct GlobalPositionSystem;

impl Default for GlobalPositionSystem {
    fn default() -> Self {
        GlobalPositionSystem
    }
}

impl GlobalPositionSystem {
    pub fn new() -> Self {
        GlobalPositionSystem::default()
    }
}

impl<CS, CD> SystemTrait<CS, CD> for GlobalPositionSystem
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
{
    fn run(&mut self, db: &mut EntityComponentDatabase<CS, CD>) -> Result<(), SystemError>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
    {
        let entities = db
            .entity_component_directory
            .get_entities_by_predicate(|entity_id| {
                db.entity_component_directory
                    .entity_has_component::<PositionComponent>(entity_id)
                    && db
                        .entity_component_directory
                        .entity_has_component::<ParentEntityComponent>(entity_id)
                    && db
                        .entity_component_directory
                        .entity_has_component::<GlobalPositionComponent>(entity_id)
            });

        for entity_id in entities {
            let parent_entity = db
                .get_entity_component::<ParentEntityComponent>(entity_id)?
                .get_parent_id();

            let position_component = db.get_entity_component::<PositionComponent>(entity_id)?;
            let mut global_position = position_component.get_position();
            let mut candidate_id = parent_entity;

            loop {
                let parent_position_component =
                    db.get_entity_component::<PositionComponent>(candidate_id)?;
                global_position += parent_position_component.get_position();

                if db
                    .get_entity_component::<GlobalPositionComponent>(candidate_id)
                    .is_err()
                {
                    break;
                }

                match db.get_entity_component::<ParentEntityComponent>(candidate_id) {
                    Ok(parent_entity_component) => {
                        candidate_id = parent_entity_component.get_parent_id()
                    }
                    Err(_) => break,
                }
            }

            db.get_entity_component_mut::<GlobalPositionComponent>(entity_id)?
                .set_global_position(global_position);
        }

        Ok(())
    }
}
