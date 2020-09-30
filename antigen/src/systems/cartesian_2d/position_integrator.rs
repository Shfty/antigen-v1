use crate::{
    components::{Position, Velocity},
    entity_component_system::system_interface::SystemInterface,
};
use crate::{
    entity_component_system::{
        ComponentStorage, EntityComponentDirectory, SystemError, SystemTrait,
    },
    primitive_types::Vector2I,
};

#[derive(Debug)]
pub struct PositionIntegrator;

impl Default for PositionIntegrator {
    fn default() -> Self {
        PositionIntegrator
    }
}

impl PositionIntegrator {
    pub fn new() -> Self {
        PositionIntegrator::default()
    }
}

impl<CS, CD> SystemTrait<CS, CD> for PositionIntegrator
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
                    .entity_has_component::<Position>(entity_id)
                    && db
                        .entity_component_directory
                        .entity_has_component::<Velocity>(entity_id)
            });

        for entity_id in entities {
            let velocity: Vector2I = **db.get_entity_component::<Velocity>(entity_id)?;

            let position = db.get_entity_component_mut::<Position>(entity_id)?;
            **position += velocity;
        }

        Ok(())
    }
}
