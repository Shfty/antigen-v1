use crate::{
    entity_component_system::entity_component_database::ComponentStorage,
    entity_component_system::entity_component_database::EntityComponentDirectory,
    entity_component_system::{SystemError, SystemTrait},
    profiler::Profiler,
};

use super::{EntityComponentDatabase, SystemRunner};

pub struct SingleThreadedSystemRunner<S, D>
where
    S: ComponentStorage,
    D: EntityComponentDirectory,
{
    system_names: Vec<String>,
    systems: Vec<Box<dyn SystemTrait<S, D>>>,
}

impl<S, D> SystemRunner<S, D> for SingleThreadedSystemRunner<S, D>
where
    S: ComponentStorage,
    D: EntityComponentDirectory,
{
    fn new() -> SingleThreadedSystemRunner<S, D> {
        SingleThreadedSystemRunner {
            system_names: Vec::new(),
            systems: Vec::new(),
        }
    }

    fn register_system<T>(&mut self, name: &str, system: T)
    where
        T: SystemTrait<S, D> + 'static,
    {
        self.system_names.push(name.into());
        self.systems.push(Box::new(system));
    }

    fn run(&mut self, ecs: &mut EntityComponentDatabase<S, D>) -> Result<(), SystemError> {
        for (name, system) in self.system_names.iter().zip(self.systems.iter_mut()) {
            let profiler = Profiler::start(&format!("\tRun {} system", name));
            system.run(ecs)?;
            profiler.finish();
        }

        Ok(())
    }
}
