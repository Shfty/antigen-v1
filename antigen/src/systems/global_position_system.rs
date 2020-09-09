use crate::{components::{GlobalPositionComponent, ParentEntityComponent, PositionComponent}, ecs::EntityComponentSystemDebug};
use crate::ecs::{SystemTrait, EntityComponentSystem, SystemEvent};

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

impl<T> SystemTrait<T> for GlobalPositionSystem where T: EntityComponentSystem + EntityComponentSystemDebug
{
    fn run(&mut self, ecs: &mut T) -> Result<SystemEvent, String> where T: EntityComponentSystem + EntityComponentSystemDebug {
        let entities = ecs.get_entities_by_predicate(|entity_id| {
            ecs.entity_has_component::<PositionComponent>(entity_id)
                && ecs.entity_has_component::<ParentEntityComponent>(entity_id)
                && ecs.entity_has_component::<GlobalPositionComponent>(entity_id)
        });

        for entity_id in entities {
            let parent_entity_component =
                ecs.get_entity_component::<ParentEntityComponent>(entity_id)?;
            let parent_entity = parent_entity_component.parent_id;

            let position_component = ecs.get_entity_component::<PositionComponent>(entity_id)?;
            let mut global_position = position_component.data;
            let mut candidate_id = parent_entity;

            loop {
                let parent_position_component =
                    ecs.get_entity_component::<PositionComponent>(candidate_id)?;
                global_position += parent_position_component.data;

                match ecs.get_entity_component::<ParentEntityComponent>(candidate_id) {
                    Ok(parent_entity_component) => candidate_id = parent_entity_component.parent_id,
                    Err(_) => break,
                }
            }

            let global_position_component =
                ecs.get_entity_component::<GlobalPositionComponent>(entity_id)?;
            global_position_component.data = global_position;
        }

        Ok(SystemEvent::None)
    }
}
