mod traits;

pub mod system_interface;
pub mod system_runner;
pub mod system_storage;

mod assemblage;
mod entity_component_directory;

pub use assemblage::*;
pub use entity_component_directory::*;

pub use system_interface::SystemInterface;
pub use system_runner::SystemRunner;
pub use system_storage::SystemStorage;
pub use traits::*;

use crate::{
    components::SystemProfilingData, systems::ComponentDataDebug, systems::ComponentDebug,
    systems::EntityDebug, systems::SceneTreeDebug, systems::SystemDebug,
};

use store::Store;

mod component_data_id;
pub use component_data_id::*;

pub struct EntityComponentSystem<CD, SS, SR>
where
    CD: EntityComponentDirectory + 'static,
    SS: SystemStorage<CD>,
    SR: SystemRunner,
{
    pub entity_component_directory: CD,
    pub system_storage: SS,
    pub system_runner: SR,
    pub component_store: Store<EntityID>,
}

impl<'a, CD, SS, SR> EntityComponentSystem<CD, SS, SR>
where
    CD: EntityComponentDirectory + 'static,
    SS: SystemStorage<CD> + 'static,
    SR: SystemRunner + 'static,
{
    pub fn new(
        entity_component_directory: CD,
        system_runner: SR,

        system_storage: SS,
    ) -> Result<Self, String>
    where
        SR: SystemRunner + 'static,
    {
        let mut ecs = EntityComponentSystem {
            entity_component_directory,
            system_runner,

            system_storage,

            component_store: Store::default(),
        };

        {
            let mut db = ecs.get_system_interface();

            let system_debug_entity = db.create_entity("System Profiling Data".into())?;
            {
                db.insert_entity_component(system_debug_entity, SystemProfilingData::default())?;
            }
        }

        ecs.push_system(EntityDebug);
        ecs.push_system(SceneTreeDebug);
        ecs.push_system(ComponentDebug);
        ecs.push_system(SystemDebug);
        ecs.push_system(ComponentDataDebug);

        Ok(ecs)
    }

    pub fn push_system<T>(&mut self, system: T)
    where
        T: SystemTrait<CD> + 'static,
    {
        self.system_storage.insert_system(system);
    }

    pub fn run(&'a mut self) -> Result<(), SystemError> {
        let mut system_interface = SystemInterface::new(
            &mut self.entity_component_directory,
            &mut self.component_store,
        );

        self.system_runner
            .run(&mut self.system_storage, &mut system_interface)
    }

    pub fn get_system_interface(&'a mut self) -> SystemInterface<CD> {
        SystemInterface::new(
            &mut self.entity_component_directory,
            &mut self.component_store,
        )
    }
}

impl<'a, CD, SS, SR> Default for EntityComponentSystem<CD, SS, SR>
where
    CD: EntityComponentDirectory + Default + 'static,
    SS: SystemStorage<CD> + Default + 'static,
    SR: SystemRunner + Default + 'static,
{
    fn default() -> Self {
        EntityComponentSystem::new(CD::default(), SR::default(), SS::default()).unwrap()
    }
}
