use crate::entity_component_system::{
    entity_component_database::{ComponentStorage, EntityComponentDirectory},
    system_storage::SystemStorage,
    EntityComponentSystem, SystemRunner,
};

/// A collection of systems and assembled entities
pub trait Scene {
    fn load<CS, CD, SS, SR>(ecs: &mut EntityComponentSystem<CS, CD, SS, SR>) -> Result<(), String>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory + 'static,
        SS: SystemStorage<CS, CD> + 'static,
        SR: SystemRunner + 'static,
    {
        Self::register_systems(ecs)?;
        Self::create_entities(ecs)?;
        Ok(())
    }

    fn register_systems<CS, CD, SS, SR>(
        ecs: &mut EntityComponentSystem<CS, CD, SS, SR>,
    ) -> Result<(), String>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory + 'static,
        SS: SystemStorage<CS, CD> + 'static,
        SR: SystemRunner + 'static;

    fn create_entities<CS, CD, SS, SR>(
        ecs: &mut EntityComponentSystem<CS, CD, SS, SR>,
    ) -> Result<(), String>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
        SS: SystemStorage<CS, CD>,
        SR: SystemRunner;
}
