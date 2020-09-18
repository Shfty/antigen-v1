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

        let ecs_debug_system = ECSDebugSystem;
        ecs.system_storage
            .insert_system("ECS Debug", ecs_debug_system);

        let mut db = ecs.get_entity_component_database();

        let entity_debug_entity = db.create_entity(None)?;
        {
            db.insert_entity_component(entity_debug_entity, EntityDebugComponent::default())?
                .register_entity(entity_debug_entity, "Entity Debug".into());

            db.insert_entity_component(entity_debug_entity, DebugExcludeComponent)?;
        }

        let component_debug_entity = db.create_entity("Component Debug".into())?;
        {
            db.insert_entity_component(component_debug_entity, ComponentDebugComponent::default())?;
            db.insert_entity_component(component_debug_entity, DebugExcludeComponent)?;
        }

        Ok(ecs)
    }

    pub fn push_system<T>(&mut self, name: &str, system: T)
    where
        T: SystemTrait<CS, CD> + 'static,
    {
        self.system_storage.insert_system(name, system)
    }

    pub fn run(&'a mut self) -> Result<(), SystemError> {
        let mut entity_component_database = EntityComponentDatabase::new(
            &mut self.component_storage,
            &mut self.entity_component_directory,
        );

        self.system_runner
            .run(&mut self.system_storage, &mut entity_component_database)
    }

    pub fn get_entity_component_database(&'a mut self) -> EntityComponentDatabase<CS, CD> {
        EntityComponentDatabase::new(
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
