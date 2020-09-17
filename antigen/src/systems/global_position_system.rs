use crate::entity_component_system::{
    entity_component_database::ComponentStorage,
    entity_component_database::EntityComponentDirectory, get_entity_component, SystemError,
    SystemTrait,
get_entity_component_mut};
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
        let entities = db.get_entities_by_predicate(|entity_id| {
            db.entity_has_component::<PositionComponent>(entity_id)
                && db.entity_has_component::<ParentEntityComponent>(entity_id)
                && db.entity_has_component::<GlobalPositionComponent>(entity_id)
        });

        for entity_id in entities {
            let parent_entity = get_entity_component::<CS, CD, ParentEntityComponent>(
                &mut db.component_storage,
                &mut db.entity_component_directory,
                entity_id,
            )?
            .get_parent_id();

            let position_component = get_entity_component::<CS, CD, PositionComponent>(
                &mut db.component_storage,
                &mut db.entity_component_directory,
                entity_id,
            )?;
            let mut global_position = position_component.get_position();
            let mut candidate_id = parent_entity;

            loop {
                let parent_position_component = get_entity_component::<CS, CD, PositionComponent>(
                    &mut db.component_storage,
                    &mut db.entity_component_directory,
                    candidate_id,
                )?;
                global_position += parent_position_component.get_position();

                if get_entity_component::<CS, CD, GlobalPositionComponent>(
                    &mut db.component_storage,
                    &mut db.entity_component_directory,
                    candidate_id,
                )
                .is_err()
                {
                    break;
                }

                match get_entity_component::<CS, CD, ParentEntityComponent>(
                    &mut db.component_storage,
                    &mut db.entity_component_directory,
                    candidate_id,
                ) {
                    Ok(parent_entity_component) => {
                        candidate_id = parent_entity_component.get_parent_id()
                    }
                    Err(_) => break,
                }
            }

            get_entity_component_mut::<CS, CD, GlobalPositionComponent>(
                &mut db.component_storage,
                &mut db.entity_component_directory,
                entity_id,
            )?
            .set_global_position(global_position);
        }

        Ok(())
    }
}
