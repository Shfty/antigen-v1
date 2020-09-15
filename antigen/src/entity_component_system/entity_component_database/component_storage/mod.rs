mod heap_component_storage;
pub use heap_component_storage::HeapComponentStorage;

use super::{ComponentDataID, ComponentDropCallback, ComponentID, ComponentTrait};

pub trait ComponentStorage {
    fn register_component<T>(&mut self)
    where
        T: ComponentTrait + Default + 'static;

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
        component_data: Box<dyn ComponentTrait>,
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