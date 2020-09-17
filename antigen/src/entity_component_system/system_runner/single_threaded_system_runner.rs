use crate::{
    entity_component_system::entity_component_database::ComponentStorage,
    entity_component_system::entity_component_database::EntityComponentDirectory,
    entity_component_system::system_storage::SystemStorage,
    entity_component_system::EntityComponentDatabase, entity_component_system::SystemError,
    profiler::Profiler,
};

use super::SystemRunner;

#[derive(Default)]
pub struct SingleThreadedSystemRunner;

impl SystemRunner for SingleThreadedSystemRunner {
    fn run<SS, CS, CD>(
        &mut self,
        system_storage: &mut SS,
        entity_component_database: &mut EntityComponentDatabase<CS, CD>,
    ) -> Result<(), SystemError>
    where
        SS: SystemStorage<CS, CD>,
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
    {
        for name in system_storage.get_system_names() {
            let system = system_storage.get_system(&name)?;
            let profiler = Profiler::start(&format!("\tRun {} system", name));
            system.run(entity_component_database)?;
            profiler.finish();
        }

        Ok(())
    }
}
