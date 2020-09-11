use crate::ecs::{EntityComponentDatabase, SystemEvent, SystemTrait};
use crate::{
    components::{GlobalPositionComponent, ParentEntityComponent, PositionComponent},
    ecs::EntityComponentDatabaseDebug,
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

impl<T> SystemTrait<T> for GlobalPositionSystem
where
    T: EntityComponentDatabase + EntityComponentDatabaseDebug,
{
    fn run(&mut self, db: &mut T) -> Result<SystemEvent, String>
    where
        T: EntityComponentDatabase + EntityComponentDatabaseDebug,
    {
        let entities = db.get_entities_by_predicate(|entity_id| {
            db.entity_has_component::<PositionComponent>(entity_id)
                && db.entity_has_component::<ParentEntityComponent>(entity_id)
                && db.entity_has_component::<GlobalPositionComponent>(entity_id)
        });

        for entity_id in entities {
            let parent_entity_component =
                db.get_entity_component::<ParentEntityComponent>(entity_id)?;
            let parent_entity = parent_entity_component.parent_id;

            let position_component = db.get_entity_component::<PositionComponent>(entity_id)?;
            let mut global_position = position_component.data;
            let mut candidate_id = parent_entity;

            loop {
                let parent_position_component =
                    db.get_entity_component::<PositionComponent>(candidate_id)?;
                global_position += parent_position_component.data;

                if db
                    .get_entity_component::<GlobalPositionComponent>(candidate_id)
                    .is_err()
                {
                    break;
                }

                match db.get_entity_component::<ParentEntityComponent>(candidate_id) {
                    Ok(parent_entity_component) => candidate_id = parent_entity_component.parent_id,
                    Err(_) => break,
                }
            }

            let global_position_component =
                db.get_entity_component_mut::<GlobalPositionComponent>(entity_id)?;
            global_position_component.data = global_position;
        }

        Ok(SystemEvent::None)
    }
}
