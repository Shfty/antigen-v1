use crate::{
    components::SystemDebugComponent, entity_component_system::system_storage::SystemStorage,
    entity_component_system::ComponentStorage, entity_component_system::EntityComponentDirectory,
    entity_component_system::SystemError, entity_component_system::SystemID,
    entity_component_system::SystemInterface, entity_component_system::SystemTrait,
    profiler::Profiler,
};

use super::SystemRunner;

#[derive(Default)]
pub struct SingleThreadedSystemRunner;

impl SystemRunner for SingleThreadedSystemRunner {
    fn run<'a, SS, CS, CD>(
        &mut self,
        system_storage: &'a mut SS,
        entity_component_database: &'a mut SystemInterface<'a, CS, CD>,
    ) -> Result<(), SystemError>
    where
        SS: SystemStorage<CS, CD>,
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
    {
        let systems = system_storage.get_systems();
        let mut systems: Vec<(SystemID, &mut dyn SystemTrait<CS, CD>)> =
            systems.into_iter().collect();
        systems.sort_by(|(lhs_id, _), (rhs_id, _)| lhs_id.cmp(rhs_id));

        for (system_id, system) in systems {
            let profiler = Profiler::start();
            system.run(entity_component_database)?;
            let duration = profiler.finish();

            if let Some(system_debug_entity) = entity_component_database
                .entity_component_directory
                .get_entity_by_predicate(|entity_id| {
                    entity_component_database
                        .entity_component_directory
                        .entity_has_component::<SystemDebugComponent>(entity_id)
                })
            {
                if let Ok(system_debug_component) = entity_component_database
                    .get_entity_component_mut::<SystemDebugComponent>(system_debug_entity)
                {
                    system_debug_component.set_duration(system_id, duration);
                }
            }
        }

        Ok(())
    }
}
