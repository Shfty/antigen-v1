use crate::entity_component_system::{
    entity_component_database::{ComponentStorage, EntityComponentDirectory},
    system_storage::SystemStorage,
    EntityComponentDatabase, EntityComponentSystem, SystemRunner,
};

/// A collection of systems and assembled entities
pub trait Scene {
    fn load<'a, CS, CD, SS, SR>(
        ecs: &'a mut EntityComponentSystem<CS, CD, SS, SR>,
    ) -> Result<(), String>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory + 'static,
        SS: SystemStorage<CS, CD> + 'static,
        SR: SystemRunner + 'static,
    {
        Self::register_systems(ecs)?;
        let mut entity_component_database = ecs.get_entity_component_database();
        Self::create_entities(&mut entity_component_database)?;
        Ok(())
    }

    fn register_systems<CS, CD, SS, SR>(
        db: &mut EntityComponentSystem<CS, CD, SS, SR>,
    ) -> Result<(), String>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory + 'static,
        SS: SystemStorage<CS, CD> + 'static,
        SR: SystemRunner + 'static;

    fn create_entities<CS, CD>(db: &mut EntityComponentDatabase<CS, CD>) -> Result<(), String>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory;
}
