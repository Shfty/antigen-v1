mod traits;

pub mod system_interface;
pub mod system_runner;
pub mod system_storage;

mod assemblage;
mod component_storage;
mod entity_component_directory;

pub use assemblage::{Assemblage, AssemblageID};
pub use component_storage::{ComponentDataID, ComponentStorage, HeapComponentStorage};
pub use entity_component_directory::{EntityComponentDirectory, SingleThreadedDirectory};

pub use system_interface::SystemInterface;
pub use system_runner::SystemRunner;
pub use system_storage::SystemStorage;
pub use traits::{
    ComponentID, ComponentTrait, EntityID, Scene, SystemError, SystemID, SystemTrait,
};

use crate::{
    components::SystemProfilingData, systems::ComponentDataDebug, systems::ComponentDebug,
    systems::EntityDebug, systems::SceneTreeDebug, systems::SystemDebug,
};

pub struct EntityComponentSystem<CS, CD, SS, SR>
where
    CS: ComponentStorage + 'static,
    CD: EntityComponentDirectory + 'static,
    SS: SystemStorage<CS, CD>,
    SR: SystemRunner,
{
    pub component_storage: CS,
    pub entity_component_directory: CD,
    pub system_storage: SS,
    pub system_runner: SR,
}

impl<'a, CS, CD, SS, SR> EntityComponentSystem<CS, CD, SS, SR>
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory + 'static,
    SS: SystemStorage<CS, CD> + 'static,
    SR: SystemRunner + 'static,
{
    pub fn new(
        component_storage: CS,
        entity_component_directory: CD,
        system_storage: SS,
        system_runner: SR,
    ) -> Result<Self, String>
    where
        SR: SystemRunner + 'static,
    {
        let mut ecs = EntityComponentSystem {
            component_storage,
            entity_component_directory,
            system_storage,
            system_runner,
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
        T: SystemTrait<CS, CD> + 'static,
    {
        self.system_storage.insert_system(system);
    }

    pub fn run(&'a mut self) -> Result<(), SystemError> {
        let mut entity_component_database = SystemInterface::new(
            &mut self.component_storage,
            &mut self.entity_component_directory,
        );

        self.system_runner
            .run(&mut self.system_storage, &mut entity_component_database)
    }

    pub fn get_system_interface(&'a mut self) -> SystemInterface<CS, CD> {
        SystemInterface::new(
            &mut self.component_storage,
            &mut self.entity_component_directory,
        )
    }
}

impl<'a, CS, CD, SS, SR> Default for EntityComponentSystem<CS, CD, SS, SR>
where
    CS: ComponentStorage + Default + 'static,
    CD: EntityComponentDirectory + Default + 'static,
    SS: SystemStorage<CS, CD> + Default + 'static,
    SR: SystemRunner + Default + 'static,
{
    fn default() -> Self {
        EntityComponentSystem::new(CS::default(), CD::default(), SS::default(), SR::default())
            .unwrap()
    }
}
