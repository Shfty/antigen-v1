mod traits;

pub mod entity_component_database;
pub mod system_runner;
pub mod system_storage;

pub use entity_component_database::{
    Assemblage, AssemblageID, ComponentDataID, EntityComponentDatabase,
};
use entity_component_database::{CallbackManager, ComponentStorage, EntityComponentDirectory};
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

// CREATE
pub fn create_entity<CS, CD>(
    component_storage: &mut CS,
    entity_component_directory: &mut CD,
    callback_manager: &mut CallbackManager<CS, CD>,
    debug_label: Option<&str>,
) -> Result<EntityID, String>
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
{
    let entity_id = entity_component_directory.create_entity()?;
    callback_manager.call_entity_create_callbacks(
        component_storage,
        entity_component_directory,
        entity_id,
        debug_label,
    );
    Ok(entity_id)
}

// INSERT
pub fn insert_component<CS, CD, T>(
    component_storage: &mut CS,
    entity_component_directory: &mut CD,
    callback_manager: &mut CallbackManager<CS, CD>,
) -> Result<ComponentID, String>
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
    T: ComponentTrait + ComponentDebugTrait + 'static,
{
    let component_id = entity_component_directory.insert_component::<T>()?;
    callback_manager.call_component_create_callbacks::<T>(
        component_storage,
        entity_component_directory,
        component_id,
    );
    Ok(component_id)
}

pub fn insert_entity_component<'a, S, D, T>(
    component_storage: &'a mut S,
    entity_component_directory: &mut D,
    entity_id: EntityID,
    component_data: T,
) -> Result<&'a mut T, String>
where
    S: ComponentStorage,
    D: EntityComponentDirectory,
    T: ComponentTrait + ComponentDebugTrait + 'static,
{
    if !entity_component_directory.is_valid_component::<T>() {
        entity_component_directory.insert_component::<T>()?;
    }

    let component_data_id = component_storage.insert_component(component_data)?;
    entity_component_directory.insert_entity_component::<T>(&entity_id, component_data_id)?;

    component_storage.get_component_data_mut::<T>(&component_data_id)
}

// GET
pub fn get_entity_component<'a, CS, CD, T>(
    component_storage: &'a CS,
    entity_component_directory: &CD,
    entity_id: EntityID,
) -> Result<&'a T, String>
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
    T: ComponentTrait + 'static,
{
    let component_data_id = entity_component_directory
        .get_entity_component_data_id(&entity_id, &ComponentID::get::<T>())?;

    component_storage.get_component_data(&component_data_id)
}

pub fn get_entity_component_mut<'a, CS, CD, T>(
    component_storage: &'a mut CS,
    entity_component_directory: &mut CD,
    entity_id: EntityID,
) -> Result<&'a mut T, String>
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
    T: ComponentTrait + 'static,
{
    let component_data_id = entity_component_directory
        .get_entity_component_data_id(&entity_id, &ComponentID::get::<T>())?;

    component_storage.get_component_data_mut::<T>(&component_data_id)
}

// DESTROY
pub fn remove_component_from_entity<'a, CS, CD, T>(
    component_storage: &'a mut CS,
    entity_component_directory: &mut CD,
    entity_id: EntityID,
) -> Result<(), String>
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
    T: ComponentTrait + ComponentDebugTrait + 'static,
{
    let component_id = ComponentID::get::<T>();

    let component_data_id =
        entity_component_directory.get_entity_component_data_id(&entity_id, &component_id)?;

    component_storage.remove_component_data(&component_id, &component_data_id)?;

    entity_component_directory.destroy_entity_component(&entity_id, &component_id)?;

    Ok(())
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
        system_storage: SS,
        system_runner: SR,
    ) -> Result<Self, String>
    where
        SR: SystemRunner + 'static,
    {
        let entity_component_database =
            EntityComponentDatabase::new(component_storage, entity_component_directory);

        let mut ecs = EntityComponentSystem {
            entity_component_database,
            system_runner,
            system_storage,
        };

        ecs.system_storage.insert_system(
            "ECS Debug",
            ECSDebugSystem::new(&mut ecs.entity_component_database),
        );

        let entity_debug_entity = create_entity(
            &mut ecs.entity_component_database.component_storage,
            &mut ecs.entity_component_database.entity_component_directory,
            &mut ecs.entity_component_database.callback_manager,
            None,
        )?;
        {
            insert_entity_component(
                &mut ecs.entity_component_database.component_storage,
                &mut ecs.entity_component_database.entity_component_directory,
                entity_debug_entity,
                EntityDebugComponent::default(),
            )?
            .register_entity(entity_debug_entity, "Entity Debug".into());

            insert_entity_component(
                &mut ecs.entity_component_database.component_storage,
                &mut ecs.entity_component_database.entity_component_directory,
                entity_debug_entity,
                DebugExcludeComponent,
            )?;
        }

        let component_debug_entity = create_entity(
            &mut ecs.entity_component_database.component_storage,
            &mut ecs.entity_component_database.entity_component_directory,
            &mut ecs.entity_component_database.callback_manager,
            "Component Debug".into(),
        )?;
        {
            insert_entity_component(
                &mut ecs.entity_component_database.component_storage,
                &mut ecs.entity_component_database.entity_component_directory,
                component_debug_entity,
                ComponentDebugComponent::default(),
            )?;

            insert_entity_component(
                &mut ecs.entity_component_database.component_storage,
                &mut ecs.entity_component_database.entity_component_directory,
                component_debug_entity,
                DebugExcludeComponent,
            )?;
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
