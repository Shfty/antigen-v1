use crate::entity_component_system::{ComponentID, ComponentTrait};

use super::{ComponentDataID, ComponentDropCallback, ComponentStorage};
use std::collections::HashMap;

pub struct HeapComponentStorage {
    component_data: HashMap<ComponentDataID, Box<dyn ComponentTrait>>,
    component_drop_callbacks: HashMap<ComponentID, Vec<ComponentDropCallback>>,
}

impl<'a> HeapComponentStorage {
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
    fn register_component_drop_callback(
        &mut self,
        component_id: ComponentID,
        callback: ComponentDropCallback,
    ) {
        match self.component_drop_callbacks.get_mut(&component_id) {
            Some(component_drop_callbacks) => {
                component_drop_callbacks.push(callback);
            }
            None => {
                self.component_drop_callbacks
                    .insert(component_id, vec![callback]);
            }
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
        match self.component_data.get(component_data_id) {
            Some(component_data) => match component_data.as_any().downcast_ref::<T>() {
                Some(component_data) => Ok(component_data),
                None => Err(format!(
                    "Error getting component data: Failed to downcast to {}",
                    std::any::type_name::<T>()
                )),
            },
            None => Err(format!(
                "Error getting component data: No such data {}",
                component_data_id
            )),
        }
    }

    fn get_component_data_mut<T>(
        &mut self,
        component_data_id: &ComponentDataID,
    ) -> Result<&mut T, String>
    where
        T: ComponentTrait + 'static,
    {
        match self.component_data.get_mut(component_data_id) {
            Some(component_data) => match component_data.as_mut_any().downcast_mut::<T>() {
                Some(component_data) => Ok(component_data),
                None => Err(format!(
                    "Error getting mutable component data: Failed to downcast to {}",
                    std::any::type_name::<T>()
                )),
            },
            None => Err(format!(
                "Error getting component data: No such data {}",
                component_data_id
            )),
        }
    }

    fn get_component_data_dyn(
        &self,
        component_data_id: &ComponentDataID,
    ) -> Result<&dyn ComponentTrait, String> {
        match self.component_data.get(component_data_id) {
            Some(component_data) => Ok(component_data.as_ref()),
            None => Err(format!(
                "Error getting component data: No such data {}",
                component_data_id
            )),
        }
    }

    fn get_component_data_dyn_mut(
        &mut self,
        component_data_id: &ComponentDataID,
    ) -> Result<&mut dyn ComponentTrait, String> {
        match self.component_data.get_mut(component_data_id) {
            Some(component_data) => Ok(component_data.as_mut()),
            None => Err(format!(
                "Error getting component data: No such data {}",
                component_data_id
            )),
        }
    }

    fn remove_component_data(
        &mut self,
        component_id: &ComponentID,
        component_data_id: &ComponentDataID,
    ) -> Result<(), String> {
        match self.component_data.remove(component_data_id) {
            Some(mut component_data) => {
                if let Some(component_drop_callbacks) =
                    self.component_drop_callbacks.get(component_id)
                {
                    for callback in component_drop_callbacks {
                        callback(component_data.as_mut());
                    }
                }
                Ok(())
            }
            None => Err(format!(
                "Error removing component data: No such data {}",
                component_data_id
            )),
        }
    }
}
