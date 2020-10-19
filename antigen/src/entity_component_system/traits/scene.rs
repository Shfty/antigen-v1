use crate::entity_component_system::{
    system_storage::SystemStorage, EntityComponentDirectory, EntityComponentSystem,
    SystemInterface, SystemRunner,
};

/// A collection of systems and assembled entities
pub trait Scene {
    fn load<'a, CD, SS, SR>(ecs: &'a mut EntityComponentSystem<CD, SS, SR>) -> Result<(), String>
    where
        CD: EntityComponentDirectory + 'static,
        SS: SystemStorage<CD> + 'static,
        SR: SystemRunner + 'static,
    {
        Self::register_systems(ecs)?;
        let mut system_interface = ecs.get_system_interface();
        Self::create_entities(&mut system_interface)?;
        Ok(())
    }

    fn register_systems<CD, SS, SR>(
        db: &mut EntityComponentSystem<CD, SS, SR>,
    ) -> Result<(), String>
    where
        CD: EntityComponentDirectory + 'static,
        SS: SystemStorage<CD> + 'static,
        SR: SystemRunner + 'static;

    fn create_entities<CD>(db: &mut SystemInterface<CD>) -> Result<(), String>
    where
        CD: EntityComponentDirectory;
}
