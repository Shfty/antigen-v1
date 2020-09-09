use antigen::ecs::{EntityComponentSystem, EntityComponentSystemDebug, SystemEvent, SystemTrait};

use crate::profiler::Profiler;

pub struct SystemRunner<'a, T>
where
    T: EntityComponentSystem + EntityComponentSystemDebug,
{
    ecs: &'a mut T,
    system_names: Vec<String>,
    systems: Vec<&'a mut dyn SystemTrait<T>>,
}

impl<'a, T> SystemRunner<'a, T>
where
    T: EntityComponentSystem + EntityComponentSystemDebug,
{
    pub fn new(ecs: &'a mut T) -> SystemRunner<'a, T> {
        SystemRunner {
            ecs,
            system_names: Vec::new(),
            systems: Vec::new(),
        }
    }

    pub fn register_system(&mut self, name: &str, system: &'a mut dyn SystemTrait<T>) {
        self.system_names.push(name.into());
        self.systems.push(system);
    }

    pub fn run(&mut self) -> Result<SystemEvent, String> {
        for (name, system) in self.system_names.iter().zip(self.systems.iter_mut()) {
            let profiler = Profiler::start(&format!("\tRun {} system", name));
            if let Ok(SystemEvent::Quit) = system.run(self.ecs) {
                return Ok(SystemEvent::Quit);
            };
            profiler.finish();
        }

        Ok(SystemEvent::None)
    }
}
