use std::cell::{Ref, RefMut};

use store::Store;

use crate::components::Name;

use super::{
    traits::ComponentData, ComponentID, ComponentTrait, EntityComponentDirectory, EntityID,
};

/// Ties together component data storage, entity-component lookup, and callback handling
pub struct SystemInterface<'a, CD>
where
    CD: EntityComponentDirectory + 'static,
{
    pub entity_component_directory: &'a mut CD,

    pub component_store: &'a mut Store,
}

impl<'a, CD> SystemInterface<'a, CD>
where
    CD: EntityComponentDirectory,
{
    pub fn new(entity_component_directory: &'a mut CD, component_store: &'a mut Store) -> Self {
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
        if !self.component_store.has_storage_for::<ComponentData<T>>() {
            self.component_store.add_storage_for::<ComponentData<T>>();
        }

        self.component_store
            .get_storage::<ComponentData<T>>()
            .borrow_mut()
            .insert(entity_id, ComponentData(component_data));

        Ok(())
    }

    // GET
    pub fn is_valid_entity(&self, entity_id: &EntityID) -> bool {
        self.entity_component_directory.is_valid_entity(entity_id)
    }

    pub fn entity_has_component<T: ComponentTrait + 'static>(&self, entity_id: &EntityID) -> bool {
        self.component_store
            .get_storage::<ComponentData<T>>()
            .borrow()
            .contains_key(entity_id)
    }

    pub fn get_entity_component<T>(&self, entity_id: EntityID) -> Result<Ref<T>, String>
    where
        CD: EntityComponentDirectory,
        T: ComponentTrait + 'static,
    {
        let storage_ref = self
            .component_store
            .get_storage::<ComponentData<T>>()
            .borrow();

        if !storage_ref.contains_key(&entity_id) {
            return Err(format!(
                "No such component {} for entity {}",
                std::any::type_name::<T>(),
                entity_id
            ));
        }

        Ok(Ref::map(storage_ref, |storage| {
            storage.get(&entity_id).unwrap().as_ref()
        }))
    }

    pub fn get_entity_component_mut<T>(&mut self, entity_id: EntityID) -> Result<RefMut<T>, String>
    where
        CD: EntityComponentDirectory,
        T: ComponentTrait + 'static,
    {
        let storage_ref = self
            .component_store
            .get_storage::<ComponentData<T>>()
            .borrow_mut();

        if !storage_ref.contains_key(&entity_id) {
            return Err(format!(
                "No such component {} for entity {}",
                std::any::type_name::<T>(),
                entity_id
            ));
        }

        Ok(RefMut::map(storage_ref, |storage| {
            storage.get_mut(&entity_id).unwrap().as_mut()
        }))
    }

    // DESTROY
    pub fn remove_component_from_entity<T>(&mut self, entity_id: EntityID) -> Result<(), String>
    where
        CD: EntityComponentDirectory,
        T: ComponentTrait + 'static,
    {
        self.component_store
            .get_storage::<ComponentData<T>>()
            .borrow_mut()
            .remove(&entity_id);

        Ok(())
    }

    pub fn destroy_entity(&mut self, entity_id: EntityID) -> Result<(), String>
    where
        CD: EntityComponentDirectory,
    {
        let component_data_ids = self
            .entity_component_directory
            .get_entity_component_data(&entity_id)?;

        // FIXME: Type-independent removal functionality for store
        /*
        for (component_id, component_data_id) in component_data_ids {
            self.component_storage
                .remove_component_data(&component_id, &component_data_id)?;
        }
        */

        self.entity_component_directory.destroy_entity(entity_id)
    }

    pub fn destroy_component<T>(&mut self) -> Result<(), String>
    where
        CD: EntityComponentDirectory,
        T: ComponentTrait + 'static,
    {
        let component_id = ComponentID::get::<T>();
        let entities: Vec<EntityID> = self
            .entity_component_directory
            .get_entities_by_predicate(|entity_id| self.entity_has_component::<T>(entity_id));

        for entity_id in entities {
            let component_data_id = self
                .entity_component_directory
                .get_entity_component_data_id(&entity_id, &component_id)?;

            // FIXME: Type-independent removal functionality for store
            /*
            self.component_storage
                .remove_component_data(&component_id, &component_data_id)?;
            */
        }

        self.entity_component_directory.destroy_component::<T>()
    }
}
