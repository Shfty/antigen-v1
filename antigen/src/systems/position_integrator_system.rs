use crate::components::{PositionComponent, VelocityComponent};
use crate::ecs::{EntityComponentDatabase, SystemError, SystemTrait};

#[derive(Debug)]
pub struct PositionIntegratorSystem;

impl Default for PositionIntegratorSystem {
    fn default() -> Self {
        PositionIntegratorSystem
    }
}

impl PositionIntegratorSystem {
    pub fn new() -> Self {
        PositionIntegratorSystem::default()
    }
}

impl<T> SystemTrait<T> for PositionIntegratorSystem
where
    T: EntityComponentDatabase,
{
    fn run(&mut self, db: &mut T) -> Result<(), SystemError>
    where
        T: EntityComponentDatabase,
    {
        let entities = db.get_entities_by_predicate(|entity_id| {
            db.entity_has_component::<PositionComponent>(entity_id)
                && db.entity_has_component::<VelocityComponent>(entity_id)
        });

        for entity_id in entities {
            let velocity = db
                .get_entity_component::<VelocityComponent>(entity_id)?
                .get_velocity();

            let position_component = db.get_entity_component_mut::<PositionComponent>(entity_id)?;
            position_component.set_position(position_component.get_position() + velocity);
        }

        Ok(())
    }
}
