mod component_data_id;
mod heap_component_storage;
pub use component_data_id::ComponentDataID;
pub use heap_component_storage::HeapComponentStorage;

use crate::entity_component_system::{ComponentDropCallback, ComponentID, ComponentTrait};

pub trait ComponentStorage {
    fn register_component_drop_callback(
        &mut self,
        component_id: ComponentID,
        callback: ComponentDropCallback,
    );

    fn insert_component<T>(&mut self, component_data: T) -> Result<ComponentDataID, String>
    where
        T: ComponentTrait + 'static;

    fn get_component_data<T>(&self, component_data_id: &ComponentDataID) -> Result<&T, String>
    where
        T: ComponentTrait + 'static;

    fn get_component_data_mut<T>(
        &mut self,
        component_data_id: &ComponentDataID,
    ) -> Result<&mut T, String>
    where
        T: ComponentTrait + 'static;

    fn get_component_data_dyn(
        &self,
        component_data_id: &ComponentDataID,
    ) -> Result<&dyn ComponentTrait, String>;

    fn get_component_data_dyn_mut(
        &mut self,
        component_data_id: &ComponentDataID,
    ) -> Result<&mut dyn ComponentTrait, String>;

    fn remove_component_data(
        &mut self,
        component_id: &ComponentID,
        component_data_id: &ComponentDataID,
    ) -> Result<(), String>;
}
