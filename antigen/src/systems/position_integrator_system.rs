use crate::{components::{PositionComponent, VelocityComponent}, ecs::EntityComponentDatabaseDebug};
use crate::{
    ecs::{SystemTrait, EntityComponentDatabase, SystemEvent},
    primitive_types::IVector2,
};

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

impl<T> SystemTrait<T> for PositionIntegratorSystem where T: EntityComponentDatabase + EntityComponentDatabaseDebug {
    fn run(&mut self, db: &mut T) -> Result<SystemEvent, String> where T: EntityComponentDatabase + EntityComponentDatabaseDebug {
        let entities = db.get_entities_by_predicate(|entity_id| {
            db.entity_has_component::<PositionComponent>(entity_id)
                && db.entity_has_component::<VelocityComponent>(entity_id)
        });

        for entity_id in entities {
            let velocity_component = db.get_entity_component::<VelocityComponent>(entity_id)?;

            let IVector2(x_vel, y_vel) = velocity_component.data;

            let position_component = db.get_entity_component_mut::<PositionComponent>(entity_id)?;

            let IVector2(x, y) = &mut position_component.data;

            *x += x_vel;
            *y += y_vel;
        }

        Ok(SystemEvent::None)
    }
}
