use crate::components::{PositionComponent, VelocityComponent};
use crate::{
    ecs::{SystemTrait, ECS},
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

impl<T> SystemTrait<T> for PositionIntegratorSystem where T: ECS {
    fn run(&mut self, ecs: &mut T) -> Result<(), String> {
        let entities = ecs.get_entities_by_predicate(|entity_id| {
            ecs.entity_has_component::<PositionComponent>(entity_id)
                && ecs.entity_has_component::<VelocityComponent>(entity_id)
        });

        for entity_id in entities {
            let velocity_component = ecs.get_entity_component::<VelocityComponent>(entity_id)?;

            let IVector2(x_vel, y_vel) = velocity_component.data;

            let position_component = ecs.get_entity_component::<PositionComponent>(entity_id)?;

            let IVector2(x, y) = &mut position_component.data;

            *x += x_vel;
            *y += y_vel;
        }

        Ok(())
    }
}
