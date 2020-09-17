use super::{ComponentDebugTrait, ComponentID, ComponentTrait, EntityID};

mod assemblage;
mod component_storage;
mod entity_component_directory;

pub use assemblage::{Assemblage, AssemblageID};
use component_storage::ComponentDropCallback;
pub use component_storage::{ComponentDataID, ComponentStorage, HeapComponentStorage};
pub use entity_component_directory::{EntityComponentDirectory, SingleThreadedDirectory};

pub type EntityCreateCallback<S, D> =
    fn(&mut EntityComponentDatabase<S, D>, EntityID, Option<&str>);

pub type ComponentCreateCallback<S, D> =
    fn(&mut EntityComponentDatabase<S, D>, ComponentID, &str, &str);

/// Ties together component data storage, entity-component lookup, and callback handling
pub struct EntityComponentDatabase<S: ComponentStorage, D: EntityComponentDirectory> {
    component_storage: S,
    entity_component_directory: D,

    entity_create_callbacks: Vec<EntityCreateCallback<S, D>>,
    component_create_callbacks: Vec<ComponentCreateCallback<S, D>>,
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
            entity_create_callbacks: Vec::new(),
            component_create_callbacks: Vec::new(),
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

    // CREATE
    pub fn create_entity(&mut self, debug_label: Option<&str>) -> Result<EntityID, String> {
        let entity_id = self.entity_component_directory.create_entity()?;

        for callback in &self.entity_create_callbacks.clone() {
            callback(self, entity_id, debug_label);
        }

        Ok(entity_id)
    }

    // INSERT
    pub fn insert_component<T: ComponentTrait + ComponentDebugTrait + 'static>(
        &mut self,
    ) -> Result<ComponentID, String> {
        let component_id = self.entity_component_directory.insert_component::<T>()?;

        for callback in &self.component_create_callbacks.clone() {
            callback(self, component_id, &T::get_name(), &T::get_description());
        }

        Ok(component_id)
    }

    pub fn insert_entity_component<T: ComponentTrait + ComponentDebugTrait + 'static>(
        &mut self,
        entity_id: EntityID,
        component_data: T,
    ) -> Result<&mut T, String> {
        if !self.is_valid_component::<T>() {
            self.insert_component::<T>()?;
        }

        let component_data_id = self.component_storage.insert_component(component_data)?;

        self.entity_component_directory
            .insert_entity_component::<T>(&entity_id, component_data_id)?;

        self.component_storage
            .get_component_data_mut::<T>(&component_data_id)
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

    pub fn get_entity_component<T: ComponentTrait + 'static>(
        &self,
        entity_id: EntityID,
    ) -> Result<&T, String> {
        let component_data_id = self
            .entity_component_directory
            .get_entity_component_data_id(&entity_id, &ComponentID::get::<T>())?;

        self.component_storage
            .get_component_data(&component_data_id)
    }

    pub fn get_entity_component_mut<T: ComponentTrait + 'static>(
        &mut self,
        entity_id: EntityID,
    ) -> Result<&mut T, String> {
        let component_data_id = self
            .entity_component_directory
            .get_entity_component_data_id(&entity_id, &ComponentID::get::<T>())?;

        self.component_storage
            .get_component_data_mut::<T>(&component_data_id)
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
    pub fn register_entity_create_callback(&mut self, callback: EntityCreateCallback<S, D>) {
        self.entity_create_callbacks.push(callback);
    }

    pub fn register_component_create_callback(&mut self, callback: ComponentCreateCallback<S, D>) {
        self.component_create_callbacks.push(callback);
    }

    pub fn register_component_drop_callback<T>(&mut self, callback: ComponentDropCallback)
    where
        T: ComponentTrait + 'static,
    {
        self.component_storage
            .register_component_drop_callback::<T>(callback)
    }
}
