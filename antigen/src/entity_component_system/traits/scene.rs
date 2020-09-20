use crate::entity_component_system::{
    system_storage::SystemStorage,
    SystemInterface, EntityComponentSystem, SystemRunner,
ComponentStorage, EntityComponentDirectory};

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
        let mut entity_component_database = ecs.get_system_interface();
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

    fn create_entities<CS, CD>(db: &mut SystemInterface<CS, CD>) -> Result<(), String>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory;
}
