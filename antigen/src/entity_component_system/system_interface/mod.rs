use crate::components::{ComponentDebugComponent, EntityDebugComponent};

use super::{ComponentDebugTrait, ComponentID, ComponentTrait, EntityID, ComponentStorage, EntityComponentDirectory};

/// Ties together component data storage, entity-component lookup, and callback handling
pub struct SystemInterface<'a, CS, CD>
where
    CS: ComponentStorage + 'static,
    CD: EntityComponentDirectory + 'static,
{
    pub component_storage: &'a mut CS,
    pub entity_component_directory: &'a mut CD,
}

impl<'a, CS, CD> SystemInterface<'a, CS, CD>
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
{
    pub fn new(component_storage: &'a mut CS, entity_component_directory: &'a mut CD) -> Self {
        SystemInterface {
            component_storage,
            entity_component_directory,
        }
    }
}

impl<'a, CS, CD> SystemInterface<'a, CS, CD>
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
{
    // CREATE
    pub fn create_entity(&mut self, debug_label: Option<&str>) -> Result<EntityID, String> {
        let entity_id = self.entity_component_directory.create_entity()?;

        if let Some(debug_label) = debug_label {
            if let Some(entity_debug_entity) = self
                .entity_component_directory
                .get_entity_by_predicate(|entity_id| {
                    self.entity_component_directory
                        .entity_has_component::<EntityDebugComponent>(entity_id)
                })
            {
                if let Ok(entity_debug_component) =
                    self.get_entity_component_mut::<EntityDebugComponent>(entity_debug_entity)
                {
                    entity_debug_component.register_entity(entity_id, debug_label.into());
                }
            }
        }

        Ok(entity_id)
    }

    // INSERT
    pub fn insert_component<T>(&mut self) -> Result<ComponentID, String>
    where
        T: ComponentTrait + ComponentDebugTrait + 'static,
    {
        let component_id = self.entity_component_directory.insert_component::<T>()?;

        if let Some(component_debug_entity) = self
            .entity_component_directory
            .get_entity_by_predicate(|entity_id| {
                self.entity_component_directory
                    .entity_has_component::<ComponentDebugComponent>(entity_id)
            })
        {
            if let Ok(component_debug_component) =
                self.get_entity_component_mut::<ComponentDebugComponent>(component_debug_entity)
            {
                component_debug_component.register_component(
                    component_id,
                    T::get_name(),
                    T::get_description(),
                );
            }
        }

        Ok(component_id)
    }

    pub fn insert_entity_component<T>(
        &mut self,
        entity_id: EntityID,
        component_data: T,
    ) -> Result<&mut T, String>
    where
        T: ComponentTrait + ComponentDebugTrait + 'static,
    {
        if !self.entity_component_directory.is_valid_component::<T>() {
            self.insert_component::<T>()?;
        }

        let component_data_id = self.component_storage.insert_component(component_data)?;
        self.entity_component_directory
            .insert_entity_component::<T>(&entity_id, component_data_id)?;

        self.component_storage
            .get_component_data_mut::<T>(&component_data_id)
    }

    // GET
    pub fn get_entity_component<T>(&self, entity_id: EntityID) -> Result<&T, String>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
        T: ComponentTrait + 'static,
    {
        let component_data_id = self
            .entity_component_directory
            .get_entity_component_data_id(&entity_id, &ComponentID::get::<T>())?;

        self.component_storage
            .get_component_data(&component_data_id)
    }

    pub fn get_entity_component_mut<T>(&mut self, entity_id: EntityID) -> Result<&mut T, String>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
        T: ComponentTrait + 'static,
    {
        let component_data_id = self
            .entity_component_directory
            .get_entity_component_data_id(&entity_id, &ComponentID::get::<T>())?;

        self.component_storage
            .get_component_data_mut::<T>(&component_data_id)
    }

    // DESTROY
    pub fn remove_component_from_entity<T>(&mut self, entity_id: EntityID) -> Result<(), String>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
        T: ComponentTrait + ComponentDebugTrait + 'static,
    {
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

    pub fn destroy_entity(&mut self, entity_id: EntityID) -> Result<(), String>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
    {
        let component_data_ids = self
            .entity_component_directory
            .get_entity_component_data(&entity_id)?;

        for (component_id, component_data_id) in component_data_ids {
            self.component_storage
                .remove_component_data(&component_id, &component_data_id)?;
        }

        self.entity_component_directory.destroy_entity(entity_id)
    }

    pub fn destroy_component<T>(&mut self) -> Result<(), String>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
        T: ComponentTrait + ComponentDebugTrait + 'static,
    {
        let component_id = ComponentID::get::<T>();
        let entities: Vec<EntityID> =
            self.entity_component_directory
                .get_entities_by_predicate(|entity_id| {
                    self.entity_component_directory
                        .entity_has_component::<T>(entity_id)
                });

        for entity_id in entities {
            let component_data_id = self
                .entity_component_directory
                .get_entity_component_data_id(&entity_id, &component_id)?;

            self.component_storage
                .remove_component_data(&component_id, &component_data_id)?;
        }

        self.entity_component_directory.destroy_component::<T>()
    }
}
