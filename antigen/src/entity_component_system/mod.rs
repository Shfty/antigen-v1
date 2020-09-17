mod traits;

pub mod entity_component_database;
pub mod system_runner;
pub mod system_storage;

pub use entity_component_database::{
    Assemblage, AssemblageID, ComponentDataID, EntityComponentDatabase,
};
use entity_component_database::{ComponentStorage, EntityComponentDirectory};
pub use system_runner::SystemRunner;
use system_storage::SystemStorage;
pub use traits::{
    ComponentDebugTrait, ComponentID, ComponentTrait, EntityID, Scene, SystemError, SystemTrait,
};

use crate::{
    components::ComponentDebugComponent, components::DebugExcludeComponent,
    components::EntityDebugComponent, systems::ECSDebugSystem,
};

pub struct EntityComponentSystem<CS, CD, SS, SR>
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
    SS: SystemStorage<CS, CD>,
    SR: SystemRunner,
{
    pub entity_component_database: EntityComponentDatabase<CS, CD>,
    pub system_storage: SS,
    pub system_runner: SR,
}

impl<CS, CD, SS, SR> EntityComponentSystem<CS, CD, SS, SR>
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory + 'static,
    SS: SystemStorage<CS, CD> + 'static,
    SR: SystemRunner + 'static,
{
    pub fn new(
        component_storage: CS,
        entity_component_directory: CD,
        mut system_storage: SS,
        system_runner: SR,
    ) -> Result<Self, String>
    where
        SR: SystemRunner + 'static,
    {
        let mut entity_component_database =
            EntityComponentDatabase::new(component_storage, entity_component_directory);

        system_storage.insert_system(
            "ECS Debug",
            ECSDebugSystem::new(&mut entity_component_database),
        );

        let mut ecs = EntityComponentSystem {
            entity_component_database,
            system_runner,
            system_storage,
        };

        let entity_debug_entity = ecs.entity_component_database.create_entity(None)?;
        {
            ecs.entity_component_database
                .insert_entity_component(entity_debug_entity, EntityDebugComponent::default())?
                .register_entity(entity_debug_entity, "Entity Debug".into());

            ecs.entity_component_database
                .insert_entity_component(entity_debug_entity, DebugExcludeComponent)?;
        }

        let component_debug_entity = ecs
            .entity_component_database
            .create_entity("Component Debug".into())?;
        {
            ecs.entity_component_database.insert_entity_component(
                component_debug_entity,
                ComponentDebugComponent::default(),
            )?;
            ecs.entity_component_database
                .insert_entity_component(component_debug_entity, DebugExcludeComponent)?;
        }

        Ok(ecs)
    }

    pub fn push_system<T>(&mut self, name: &str, system: T)
    where
        T: SystemTrait<CS, CD> + 'static,
    {
        self.system_storage.insert_system(name, system)
    }

    pub fn run(&mut self) -> Result<(), SystemError> {
        self.system_runner.run(
            &mut self.system_storage,
            &mut self.entity_component_database,
        )
    }
}

impl<CS, CD, SS, SR> Default for EntityComponentSystem<CS, CD, SS, SR>
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
