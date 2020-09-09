use crate::{
    ecs::{EntityComponentDatabase, EntityComponentDatabaseDebug, SystemEvent, SystemTrait},
    profiler::Profiler,
};

use super::SystemRunner;

pub struct SingleThreadedSystemRunner<'a, T>
where
    T: EntityComponentDatabase + EntityComponentDatabaseDebug,
{
    db: &'a mut T,
    system_names: Vec<String>,
    systems: Vec<&'a mut dyn SystemTrait<T>>,
}

impl<'a, T> SystemRunner<'a, T> for SingleThreadedSystemRunner<'a, T>
where
    T: EntityComponentDatabase + EntityComponentDatabaseDebug,
{
    fn new(db: &'a mut T) -> SingleThreadedSystemRunner<'a, T> {
        SingleThreadedSystemRunner {
            db,
            system_names: Vec::new(),
            systems: Vec::new(),
        }
    }

    fn register_system(&mut self, name: &str, system: &'a mut dyn SystemTrait<T>) {
        self.system_names.push(name.into());
        self.systems.push(system);
    }

    fn run(&mut self) -> Result<SystemEvent, String> {
        for (name, system) in self.system_names.iter().zip(self.systems.iter_mut()) {
            let profiler = Profiler::start(&format!("\tRun {} system", name));
            if let Ok(SystemEvent::Quit) = system.run(self.db) {
                return Ok(SystemEvent::Quit);
            };
            profiler.finish();
        }

        Ok(SystemEvent::None)
    }
}
