use std::{collections::HashMap, fmt::Debug};

use super::{ComponentDataID, ComponentID, ComponentTrait, ComponentDropCallback};

pub trait ComponentStorage {
    fn register_component_drop_callback(
        &mut self,
        component_id: ComponentID,
        callback: ComponentDropCallback,
    );

    fn store_component_by_id(
        &mut self,
        component_id: ComponentID,
    ) -> Result<ComponentDataID, String>;

    fn insert_component(
        &mut self,
        component_data: Box<dyn ComponentTrait>
    ) -> Result<ComponentDataID, String>;

    fn get_component_data(
        &self,
        component_data_id: &ComponentDataID,
    ) -> Result<&dyn ComponentTrait, String>;

    fn get_component_data_mut(
        &mut self,
        component_data_id: &ComponentDataID,
    ) -> Result<&mut dyn ComponentTrait, String>;

    fn remove_component_data(
        &mut self,
        component_id: &ComponentID,
        component_data_id: &ComponentDataID,
    ) -> Result<(), String>;
}

pub struct HeapComponentStorage<'a> {
    component_constructors: HashMap<ComponentID, &'a dyn Fn() -> Box<dyn ComponentTrait>>,
    component_data: HashMap<ComponentDataID, Box<dyn ComponentTrait>>,
    component_drop_callbacks: HashMap<ComponentID, Vec<ComponentDropCallback>>,
}

impl<'a> Debug for HeapComponentStorage<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let component_ids: Vec<String> = self
            .component_constructors
            .keys()
            .map(|key| key.to_string())
            .collect();

        f.debug_struct("HeapComponentStorage")
            .field("component_constructors", &component_ids)
            .field("component_data", &self.component_data)
            .finish()
    }
}

impl<'a> HeapComponentStorage<'a> {
    pub fn new() -> Self {
        HeapComponentStorage {
            component_constructors: HashMap::new(),
            component_data: HashMap::new(),
            component_drop_callbacks: HashMap::new(),
        }
    }

    pub fn register_component<T>(&mut self)
    where
        T: ComponentTrait + Default + 'static,
    {
        self.component_constructors
            .insert(ComponentID::get::<T>(), &|| Box::new(T::default()));
    }

    pub fn store_component<T>(&mut self, component_data: T) -> ComponentDataID
    where
        T: ComponentTrait + Default + 'static,
    {
        self.register_component::<T>();

        let component_data_id = ComponentDataID::next();
        self.component_data
            .insert(component_data_id, Box::new(component_data));
        component_data_id
    }

    pub fn store_component_by_id(&mut self, component_id: ComponentID) -> ComponentDataID {
        let component_data_id = ComponentDataID::next();
        let component_data = self.component_constructors.get(&component_id).unwrap()();
        self.component_data
            .insert(component_data_id, component_data);
        component_data_id
    }
}

impl<'a> Default for HeapComponentStorage<'a> {
    fn default() -> Self {
        HeapComponentStorage::new()
    }
}

impl<'a> ComponentStorage for HeapComponentStorage<'a> {
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

    fn store_component_by_id(
        &mut self,
        component_id: ComponentID,
    ) -> Result<ComponentDataID, String> {
        let component_data = match self.component_constructors.get(&component_id) {
            Some(constructor) => constructor(),
            None => {
                return Err(format!(
                    "Error storing component {} by ID: No constructor registered",
                    component_id
                ))
            }
        };

        let component_data_id = ComponentDataID::next();
        self.component_data
            .insert(component_data_id, component_data);

        Ok(component_data_id)
    }

    fn insert_component(
        &mut self,
        component_data: Box<dyn ComponentTrait>
    ) -> Result<ComponentDataID, String> {
        let id = ComponentDataID::next();
        self.component_data.insert(id, component_data);
        Ok(id)
    }

    fn get_component_data(
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

    fn get_component_data_mut(
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
