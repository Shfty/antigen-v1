use crate::{
    entity_component_system::ComponentStorage,
    entity_component_system::{EntityComponentDirectory, SystemError, SystemTrait},
    profiler::Profiler,
};

use super::{EntityComponentDatabase, SystemRunner};

pub struct SingleThreadedSystemRunner<'a, S, D>
where
    S: ComponentStorage,
    D: EntityComponentDirectory,
{
    system_names: Vec<String>,
    systems: Vec<&'a mut dyn SystemTrait<S, D>>,
}

impl<'a, S, D> SystemRunner<'a, S, D> for SingleThreadedSystemRunner<'a, S, D>
where
    S: ComponentStorage,
    D: EntityComponentDirectory,
{
    fn new() -> SingleThreadedSystemRunner<'a, S, D> {
        SingleThreadedSystemRunner {
            system_names: Vec::new(),
            systems: Vec::new(),
        }
    }

    fn register_system(&mut self, name: &str, system: &'a mut dyn SystemTrait<S, D>) {
        self.system_names.push(name.into());
        self.systems.push(system);
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
