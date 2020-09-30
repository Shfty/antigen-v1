use crate::entity_component_system::{traits::DowncastComponentTrait, ComponentID, ComponentTrait};

use super::{ComponentDataID, ComponentDropCallback, ComponentStorage};
use std::collections::HashMap;

pub struct HeapComponentStorage {
    component_data: HashMap<ComponentDataID, Box<dyn ComponentTrait>>,
    component_drop_callbacks: HashMap<ComponentID, Vec<ComponentDropCallback>>,
}

impl HeapComponentStorage {
    pub fn new() -> Self {
        HeapComponentStorage {
            component_data: HashMap::new(),
            component_drop_callbacks: HashMap::new(),
        }
    }
}

impl Default for HeapComponentStorage {
    fn default() -> Self {
        HeapComponentStorage::new()
    }
}

impl ComponentStorage for HeapComponentStorage {
    fn register_component_drop_callback<T>(&mut self, callback: ComponentDropCallback)
    where
        T: ComponentTrait + 'static,
    {
        let component_id = ComponentID::get::<T>();
        if let Some(component_drop_callbacks) = self.component_drop_callbacks.get_mut(&component_id)
        {
            component_drop_callbacks.push(callback);
        } else {
            self.component_drop_callbacks
                .insert(component_id, vec![callback]);
        }
    }

    fn insert_component<T>(&mut self, component_data: T) -> Result<ComponentDataID, String>
    where
        T: ComponentTrait + 'static,
    {
        let id = ComponentDataID::next();
        self.component_data.insert(id, Box::new(component_data));
        Ok(id)
    }

    fn get_component_data<T>(&self, component_data_id: &ComponentDataID) -> Result<&T, String>
    where
        T: ComponentTrait + 'static,
    {
        let component_data = self.component_data.get(component_data_id).ok_or(format!(
            "Error getting component data: No such data {}",
            component_data_id
        ))?;

        let component_data = component_data.as_ref();
        let component_data = T::as_data(component_data);

        Ok(component_data)
    }

    fn get_component_data_mut<T>(
        &mut self,
        component_data_id: &ComponentDataID,
    ) -> Result<&mut T, String>
    where
        T: ComponentTrait + 'static,
    {
        let component_data = self
            .component_data
            .get_mut(component_data_id)
            .ok_or(format!(
                "Error getting component data: No such data {}",
                component_data_id
            ))?;

        let component_data = component_data.as_mut();
        let component_data = T::as_mut_data(component_data);

        Ok(component_data)
    }

    fn get_component_data_string(
        &self,
        component_data_id: &ComponentDataID,
    ) -> Result<String, String> {
        let component_data = self.component_data.get(component_data_id).ok_or(format!(
            "Error getting component data: No such data {}",
            component_data_id
        ))?;
        Ok(format!("{:#?}", component_data))
    }

    fn remove_component_data(
        &mut self,
        component_id: &ComponentID,
        component_data_id: &ComponentDataID,
    ) -> Result<(), String> {
        let mut component_data = self
            .component_data
            .remove(component_data_id)
            .ok_or(format!(
                "Error removing component data: No such data {}",
                component_data_id
            ))?;

        if let Some(component_drop_callbacks) = self.component_drop_callbacks.get(component_id) {
            for callback in component_drop_callbacks {
                callback(component_data.as_mut());
            }
        }

        Ok(())
    }
}
