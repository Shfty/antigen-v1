use crate::{
    ecs::{EntityComponentDatabase, SystemError, SystemTrait},
    profiler::Profiler,
};

use super::SystemRunner;

pub struct SingleThreadedSystemRunner<'a, T>
where
    T: EntityComponentDatabase,
{
    system_names: Vec<String>,
    systems: Vec<&'a mut dyn SystemTrait<T>>,
}

impl<'a, T> SystemRunner<'a, T> for SingleThreadedSystemRunner<'a, T>
where
    T: EntityComponentDatabase,
{
    fn new() -> SingleThreadedSystemRunner<'a, T> {
        SingleThreadedSystemRunner {
            system_names: Vec::new(),
            systems: Vec::new(),
        }
    }

    fn register_system(&mut self, name: &str, system: &'a mut dyn SystemTrait<T>) {
        self.system_names.push(name.into());
        self.systems.push(system);
    }

    fn run(&mut self, db: &mut T) -> Result<(), SystemError> {
        for (name, system) in self.system_names.iter().zip(self.systems.iter_mut()) {
            let profiler = Profiler::start(&format!("\tRun {} system", name));
            system.run(db)?;
            profiler.finish();
        }

        Ok(())
    }
}
