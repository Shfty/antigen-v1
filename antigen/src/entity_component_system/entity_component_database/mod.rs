use super::{ComponentDebugTrait, ComponentID, ComponentTrait, EntityID};

mod assemblage;
mod component_storage;
mod entity_component_directory;

pub use assemblage::{Assemblage, AssemblageID};
use component_storage::ComponentDropCallback;
pub use component_storage::{ComponentDataID, ComponentStorage, HeapComponentStorage};
pub use entity_component_directory::{EntityComponentDirectory, SingleThreadedDirectory};

pub type EntityCreateCallback<CS, CD> = fn(&mut CS, &mut CD, EntityID, Option<&str>);

pub type ComponentCreateCallback<CS, CD> = fn(&mut CS, &mut CD, ComponentID, &str, &str);

pub struct CallbackManager<CS, CD>
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
{
    entity_create_callbacks: Vec<EntityCreateCallback<CS, CD>>,
    component_create_callbacks: Vec<ComponentCreateCallback<CS, CD>>,
}

impl<CS, CD> CallbackManager<CS, CD>
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
{
    pub fn new() -> Self {
        CallbackManager {
            entity_create_callbacks: Vec::new(),
            component_create_callbacks: Vec::new(),
        }
    }

    pub fn register_entity_create_callback(&mut self, callback: EntityCreateCallback<CS, CD>) {
        self.entity_create_callbacks.push(callback);
    }

    pub fn register_component_create_callback(
        &mut self,
        callback: ComponentCreateCallback<CS, CD>,
    ) {
        self.component_create_callbacks.push(callback);
    }

    pub fn call_entity_create_callbacks(
        &mut self,
        component_storage: &mut CS,
        entity_component_directory: &mut CD,
        entity_id: EntityID,
        debug_label: Option<&str>,
    ) {
        for callback in &self.entity_create_callbacks.clone() {
            callback(
                component_storage,
                entity_component_directory,
                entity_id,
                debug_label,
            );
        }
    }

    pub fn call_component_create_callbacks<T>(
        &self,
        component_storage: &mut CS,
        entity_component_directory: &mut CD,
        component_id: ComponentID,
    ) where
        T: ComponentTrait + ComponentDebugTrait,
    {
        for callback in &self.component_create_callbacks.clone() {
            callback(
                component_storage,
                entity_component_directory,
                component_id,
                &T::get_name(),
                &T::get_description(),
            );
        }
    }
}

/// Ties together component data storage, entity-component lookup, and callback handling
pub struct EntityComponentDatabase<CS: ComponentStorage, CD: EntityComponentDirectory> {
    pub component_storage: CS,
    pub entity_component_directory: CD,
    pub callback_manager: CallbackManager<CS, CD>,
}

impl<S, D> EntityComponentDatabase<S, D>
where
    S: ComponentStorage,
    D: EntityComponentDirectory,
{
    pub fn new(component_storage: S, entity_component_directory: D) -> Self {
        EntityComponentDatabase {
            component_storage,
            entity_component_directory,
            callback_manager: CallbackManager::new(),
        }
    }
}

impl<'a, S, D> EntityComponentDatabase<S, D>
where
    S: ComponentStorage,
    D: EntityComponentDirectory,
{
    // EXIST
    pub fn is_valid_entity(&self, entity_id: &EntityID) -> bool {
        self.entity_component_directory.is_valid_entity(entity_id)
    }

    pub fn is_valid_component<T: ComponentTrait + 'static>(&self) -> bool {
        self.entity_component_directory.is_valid_component::<T>()
    }

    pub fn entity_has_component<T: ComponentTrait + 'static>(&self, entity_id: &EntityID) -> bool {
        self.entity_component_directory
            .entity_has_component::<T>(entity_id)
    }

    pub fn entity_has_component_by_id(
        &self,
        entity_id: &EntityID,
        component_id: &ComponentID,
    ) -> bool {
        self.entity_component_directory
            .entity_has_component_by_id(entity_id, component_id)
    }

    // DESTROY
    pub fn destroy_entity(&mut self, entity_id: EntityID) -> Result<(), String> {
        self.entity_component_directory.destroy_entity(entity_id)
    }

    pub fn destroy_component<T>(&mut self) -> Result<(), String>
    where
        T: ComponentTrait + ComponentDebugTrait + 'static,
    {
        self.entity_component_directory.destroy_component::<T>()
    }

    pub fn remove_component_from_entity<T: ComponentTrait + ComponentDebugTrait + 'static>(
        &mut self,
        entity_id: EntityID,
    ) -> Result<(), String> {
        let component_id = ComponentID::get::<T>();

        let component_data_id = self
            .entity_component_directory
            .get_entity_component_data_id(&entity_id, &component_id)?;

        self.component_storage
            .remove_component_data(&component_id, &component_data_id)?;

        self.entity_component_directory
            .destroy_entity_component(&entity_id, &component_id)?;

        Ok(())
    }

    // GET
    pub fn get_entity_by_predicate(
        &self,
        predicate: impl Fn(&EntityID) -> bool,
    ) -> Option<EntityID> {
        self.entity_component_directory
            .get_entity_by_predicate(predicate)
    }

    pub fn get_entities_by_predicate(
        &self,
        predicate: impl Fn(&EntityID) -> bool,
    ) -> Vec<EntityID> {
        self.entity_component_directory
            .get_entities_by_predicate(predicate)
    }

    pub fn get_components_by_predicate(
        &self,
        predicate: impl Fn(&ComponentID) -> bool,
    ) -> Vec<ComponentID> {
        self.entity_component_directory
            .get_components_by_predicate(predicate)
    }

    pub fn get_entity_component_data_id(
        &self,
        entity_id: &EntityID,
        component_id: &ComponentID,
    ) -> Result<ComponentDataID, String> {
        self.entity_component_directory
            .get_entity_component_data_id(entity_id, component_id)
    }

    pub fn get_component_data<T>(&self, component_data_id: &ComponentDataID) -> Result<&T, String>
    where
        T: ComponentTrait + 'static,
    {
        self.component_storage
            .get_component_data::<T>(component_data_id)
    }

    pub fn get_component_data_string(
        &self,
        component_data_id: &ComponentDataID,
    ) -> Result<String, String> {
        self.component_storage
            .get_component_data_string(component_data_id)
    }

    // Callback Registration
    pub fn register_component_drop_callback<T>(&mut self, callback: ComponentDropCallback)
    where
        T: ComponentTrait + 'static,
    {
        self.component_storage
            .register_component_drop_callback::<T>(callback)
    }
}
