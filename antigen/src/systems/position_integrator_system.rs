use crate::components::{PositionComponent, VelocityComponent};
use crate::ecs::{SystemTrait, ECS};

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

impl SystemTrait for PositionIntegratorSystem {
    fn run(&mut self, ecs: &mut ECS) -> Result<(), String> {
        let entities = ecs
            .build_entity_query()
            .component::<PositionComponent>()
            .component::<VelocityComponent>()
            .finish();

        for entity_id in entities {
            let velocity_component = ecs.get_entity_component::<VelocityComponent>(entity_id)?;

            let x_vel = velocity_component.x;
            let y_vel = velocity_component.y;

            let position_component = ecs.get_entity_component::<PositionComponent>(entity_id)?;

            position_component.x += x_vel;
            position_component.y += y_vel;
        }

        Ok(())
    }
}
