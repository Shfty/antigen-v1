use crate::entity_component_system::{EntityID, ComponentID, ComponentTrait, ComponentDebugTrait};

use super::{ComponentStorage, EntityComponentDirectory};

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

impl<CS, CD> Default for CallbackManager<CS, CD>
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
{
    fn default() -> Self {
        CallbackManager::new()
    }
}
