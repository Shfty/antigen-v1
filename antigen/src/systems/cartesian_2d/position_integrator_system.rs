use crate::{
    components::{Position, Velocity},
    entity_component_system::system_interface::SystemInterface,
};
use crate::{
    entity_component_system::{
        ComponentStorage, EntityComponentDirectory, SystemDebugTrait, SystemError, SystemTrait,
    },
    primitive_types::Vector2I,
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
                    .entity_has_component::<Position>(entity_id)
                    && db
                        .entity_component_directory
                        .entity_has_component::<Velocity>(entity_id)
            });

        for entity_id in entities {
            let velocity: Vector2I = (*db.get_entity_component::<Velocity>(entity_id)?).into();

            let position = db.get_entity_component_mut::<Position>(entity_id)?;
            *position = (velocity + {
                let position = *position;
                position.into()
            })
            .into();
        }

        Ok(())
    }
}

impl SystemDebugTrait for PositionIntegratorSystem {
    fn get_name() -> &'static str {
        "Position Integrator"
    }
}
