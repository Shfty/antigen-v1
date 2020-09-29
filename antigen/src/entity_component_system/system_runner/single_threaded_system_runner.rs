use crate::{
    components::SystemProfilingData, core::profiler::Profiler,
    entity_component_system::system_storage::SystemStorage,
    entity_component_system::ComponentStorage, entity_component_system::EntityComponentDirectory,
    entity_component_system::SystemError, entity_component_system::SystemID,
    entity_component_system::SystemInterface, entity_component_system::SystemTrait,
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
        superluminal_perf::begin_event("System Runner");

        if let Some(system_debug_entity) = entity_component_database
            .entity_component_directory
            .get_entity_by_predicate(|entity_id| {
                entity_component_database
                    .entity_component_directory
                    .entity_has_component::<SystemProfilingData>(entity_id)
            })
        {
            let systems = system_storage.get_systems();
            let mut systems: Vec<(SystemID, &mut dyn SystemTrait<CS, CD>)> =
                systems.into_iter().collect();
            systems.sort_by(|(lhs_id, _), (rhs_id, _)| lhs_id.cmp(rhs_id));

            for (system_id, system) in systems {
                let label = system_id.get_name();
                let profiler = Profiler::start();
                superluminal_perf::begin_event_with_data("Run System", &label, 0);
                system.run(entity_component_database)?;
                superluminal_perf::end_event();
                let duration = profiler.finish();

                entity_component_database
                    .get_entity_component_mut::<SystemProfilingData>(system_debug_entity)?
                    .set_duration(system_id, duration);
            }
        }

        superluminal_perf::end_event();

        Ok(())
    }
}
