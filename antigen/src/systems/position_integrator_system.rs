use crate::entity_component_system::{
    ComponentStorage, EntityComponentDirectory, SystemError, SystemTrait,
SystemDebugTrait};
use crate::{
    components::{PositionComponent, VelocityComponent},
    entity_component_system::system_interface::SystemInterface,
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

impl<CS, CD> SystemTrait<CS, CD> for PositionIntegratorSystem
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
{
    fn run(&mut self, db: &mut SystemInterface<CS, CD>) -> Result<(), SystemError>
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
                        .entity_has_component::<VelocityComponent>(entity_id)
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

impl SystemDebugTrait for PositionIntegratorSystem {
    fn get_name() -> &'static str {
        "Position Integrator"
    }
}
