use std::cell::{Ref, RefMut};

use store::{HybridStore, StoreTrait};

use crate::components::Name;

use super::{ComponentTrait, EntityComponentDirectory, EntityID};

/// Ties together component data storage, entity-component lookup, and callback handling
pub struct SystemInterface<'a, CD>
where
    CD: EntityComponentDirectory + 'static,
{
    pub entity_component_directory: &'a mut CD,

    pub component_store: &'a mut HybridStore<EntityID>,
}

impl<'a, CD> SystemInterface<'a, CD>
where
    CD: EntityComponentDirectory,
{
    pub fn new(
        entity_component_directory: &'a mut CD,
        component_store: &'a mut HybridStore<EntityID>,
    ) -> Self {
        SystemInterface {
            entity_component_directory,

            component_store,
        }
    }
}

impl<'a, CD> SystemInterface<'a, CD>
where
    CD: EntityComponentDirectory,
{
    // CREATE
    pub fn create_entity(&mut self, debug_label: Option<&str>) -> Result<EntityID, String> {
        let entity_id = self.entity_component_directory.create_entity()?;

        if let Some(debug_label) = debug_label {
            self.insert_entity_component(entity_id, Name(debug_label.into()))?;
        }

        Ok(entity_id)
    }

    // INSERT
    pub fn insert_entity_component<T>(
        &mut self,
        entity_id: EntityID,
        component_data: T,
    ) -> Result<(), String>
    where
        T: ComponentTrait + 'static,
    {
        self.component_store.insert(entity_id, component_data);

        Ok(())
    }

    // GET
    pub fn is_valid_entity(&self, entity_id: &EntityID) -> bool {
        self.entity_component_directory.is_valid_entity(entity_id)
    }

    pub fn entity_has_component<T: ComponentTrait + 'static>(&self, entity_id: &EntityID) -> bool {
        self.component_store.contains_type_key::<T>(entity_id)
    }

    pub fn get_entity_component<T>(&self, entity_id: EntityID) -> Result<Ref<T>, String>
    where
        CD: EntityComponentDirectory,
        T: ComponentTrait + 'static,
    {
        self.component_store.get::<T>(entity_id).ok_or(format!(
            "Failed to get component store for type {}",
            std::any::type_name::<T>()
        ))
    }

    pub fn get_entity_component_mut<T>(&mut self, entity_id: EntityID) -> Result<RefMut<T>, String>
    where
        CD: EntityComponentDirectory,
        T: ComponentTrait + 'static,
    {
        self.component_store.get_mut::<T>(entity_id).ok_or(format!(
            "Failed to get component store for type {}",
            std::any::type_name::<T>()
        ))
    }

    // DESTROY
    pub fn remove_component_from_entity<T>(&mut self, entity_id: EntityID) -> Result<(), String>
    where
        CD: EntityComponentDirectory,
        T: ComponentTrait + 'static,
    {
        self.component_store.remove::<T>(&entity_id);

        Ok(())
    }

    pub fn destroy_entity(&mut self, entity_id: EntityID) -> Result<(), String>
    where
        CD: EntityComponentDirectory,
    {
        self.component_store.remove_key(&entity_id);
        self.entity_component_directory.destroy_entity(entity_id)
    }
}
