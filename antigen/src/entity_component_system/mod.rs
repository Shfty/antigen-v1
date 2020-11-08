mod storage;
mod traits;

pub use storage::*;
pub use traits::*;

use crate::{
    assemblage::EntityBuilder, components::Name, components::SystemProfilingData,
    core::profiler::Profiler, systems::ComponentDataDebug, systems::ComponentDebug,
    systems::EntityDebug, systems::SceneTreeDebug, systems::SystemDebug,
};

use store::StoreQuery;

use std::cell::RefMut;

type SystemProfilingEntity<'a> = (EntityID, RefMut<'a, SystemProfilingData>);

pub struct EntityComponentSystem {
    system_store: SystemStore,
    component_store: ComponentStore,
}

impl<'a> EntityComponentSystem {
    pub fn new() -> Result<Self, String> {
        let mut ecs = EntityComponentSystem {
            system_store: SystemStore::default(),
            component_store: ComponentStore::default(),
        };

        EntityBuilder::new()
            .key_fields(
                EntityID::next(),
                (
                    Name("System Profiling Data".into()),
                    SystemProfilingData::default(),
                ),
            )
            .finish(&mut ecs.component_store);

        ecs.push_system(SystemDebug);
        ecs.push_system(EntityDebug);
        ecs.push_system(SceneTreeDebug);
        ecs.push_system(ComponentDebug);
        ecs.push_system(ComponentDataDebug);

        Ok(ecs)
    }

    pub fn push_system<T>(&mut self, system: T)
    where
        T: SystemTrait + 'static,
    {
        self.system_store.insert_system(system);
    }

    pub fn run(&'a mut self) -> Result<(), SystemError> {
        superluminal_perf::begin_event("System Runner");

        for (system_id, system) in self.system_store.iter() {
            let label = system_id.get_name();
            let profiler = Profiler::start();
            superluminal_perf::begin_event_with_data("Run System", &label, 0);
            system.run(&mut self.component_store)?;
            superluminal_perf::end_event();
            let duration = profiler.finish();

            if let Some((_, mut system_debug)) =
                StoreQuery::<SystemProfilingEntity>::iter(self.component_store.as_ref()).next()
            {
                system_debug.set_duration(system_id, duration)
            }
        }

        superluminal_perf::end_event();

        Ok(())
    }

    pub fn get_component_store(&'a mut self) -> &mut ComponentStore {
        &mut self.component_store
    }
}

impl<'a> Default for EntityComponentSystem {
    fn default() -> Self {
        EntityComponentSystem::new().unwrap()
    }
}
