mod assemblage;
mod component;
mod entity;
mod system;

pub mod component_storage;
pub mod entity_component_database;
pub mod system_runner;

use std::collections::HashMap;

pub use assemblage::{Assemblage, AssemblageID};
pub use component::{ComponentDataID, ComponentDebugTrait, ComponentID, ComponentTrait};
pub use component_storage::{ComponentStorage, HeapComponentStorage};
pub use entity::EntityID;
pub use entity_component_database::EntityComponentDatabase;
use entity_component_database::SingleThreadedDatabase;
pub use system::{SystemError, SystemTrait};
use system_runner::SingleThreadedSystemRunner;
pub use system_runner::SystemRunner;

type EntityCreateCallback<T> = fn(&mut T, EntityID, Option<&str>);
type ComponentCreateCallback<T> = fn(&mut T, ComponentID, &str, &str);

pub struct EntityComponentSystem<'a> {
    component_storage: HeapComponentStorage<'a>,
    entity_component_directory: SingleThreadedDatabase<'a>,
    system_runner: SingleThreadedSystemRunner<'a, SingleThreadedDatabase<'a>>,
}

impl<'a> EntityComponentSystem<'a> {
    /*
    fn new() -> Self {
        EntityComponentSystem {
            component_storage: HeapComponentStorage::new(),
            entity_component_directory: SingleThreadedDatabase::new(),
            system_runner: SystemRunner::new(),
        }
    }
    */

    // Methods to keep
    fn is_valid_entity(&self, entity_id: &EntityID) -> bool {
        self.entity_component_directory.is_valid_entity(entity_id)
    }

    fn is_valid_component<T: ComponentTrait + 'static>(&self) -> bool {
        self.entity_component_directory.is_valid_component::<T>()
    }

    fn create_entity(&mut self, debug_label: Option<&str>) -> Result<EntityID, String> {
        self.entity_component_directory.create_entity(debug_label)
    }

    fn destroy_entity(&mut self, entity_id: EntityID) -> Result<(), String> {
        self.entity_component_directory.destroy_entity(entity_id)
    }

    fn run(&mut self) -> Result<(), SystemError> {
        self.system_runner.run(&mut self.entity_component_directory)
    }

    // Methods to audit
    fn register_component<T: ComponentTrait + ComponentDebugTrait + Default + 'static>(
        &mut self,
    ) -> Result<ComponentID, String> {
        self.component_storage.register_component::<T>();
        self.entity_component_directory.register_component::<T>()
    }

    fn register_entity_create_callback(
        &mut self,
        callback: for<'r, 's> fn(
            &'r mut SingleThreadedDatabase<'a>,
            EntityID,
            std::option::Option<&'s str>,
        ),
    ) {
        self.entity_component_directory
            .register_entity_create_callback(callback)
    }

    fn register_component_create_callback(
        &mut self,
        callback: for<'r, 's, 't0> fn(
            &'r mut SingleThreadedDatabase<'a>,
            ComponentID,
            &'s str,
            &'t0 str,
        ),
    ) {
        self.entity_component_directory
            .register_component_create_callback(callback)
    }

    fn add_registered_component_to_entity(
        &mut self,
        entity_id: EntityID,
        component_id: ComponentID,
        component_data: Box<dyn ComponentTrait>,
    ) -> Result<ComponentDataID, String> {
        let component_data_id = self.component_storage.insert_component(component_data)?;
        self.entity_component_directory.insert_entity_component(
            &entity_id,
            component_id,
            component_data_id,
        )
    }

    fn remove_registered_component_from_entity(
        &mut self,
        entity_id: EntityID,
        component_id: ComponentID,
    ) -> Result<(), String> {
        self.entity_component_directory
            .remove_registered_component_from_entity(entity_id, component_id)
    }

    fn get_entity_by_predicate(&self, predicate: impl Fn(&EntityID) -> bool) -> Option<EntityID> {
        self.entity_component_directory
            .get_entity_by_predicate(predicate)
    }

    fn get_entities_by_predicate(&self, predicate: impl Fn(&EntityID) -> bool) -> Vec<EntityID> {
        self.entity_component_directory
            .get_entities_by_predicate(predicate)
    }

    fn get_components_by_predicate(
        &self,
        predicate: impl Fn(&ComponentID) -> bool,
    ) -> Vec<ComponentID> {
        self.entity_component_directory
            .get_components_by_predicate(predicate)
    }

    fn entity_has_component<T: ComponentTrait + 'static>(&self, entity_id: &EntityID) -> bool {
        self.entity_component_directory
            .entity_has_component::<T>(entity_id)
    }

    fn entity_has_component_by_id(&self, entity_id: &EntityID, component_id: &ComponentID) -> bool {
        self.entity_component_directory
            .entity_has_component_by_id(entity_id, component_id)
    }

    fn get_entity_component<T: ComponentTrait + 'static>(
        &self,
        entity_id: EntityID,
    ) -> Result<&T, String> {
        self.entity_component_directory
            .get_entity_component(entity_id)
    }

    fn get_entity_component_mut<T: ComponentTrait + 'static>(
        &mut self,
        entity_id: EntityID,
    ) -> Result<&mut T, String> {
        self.entity_component_directory
            .get_entity_component_mut(entity_id)
    }

    fn get_entity_component_data_id(
        &self,
        entity_id: &EntityID,
        component_id: &ComponentID,
    ) -> Result<ComponentDataID, String> {
        self.entity_component_directory
            .get_entity_component_data_id(entity_id, component_id)
    }

    fn add_component_to_entity<T: ComponentTrait + ComponentDebugTrait + 'static>(
        &mut self,
        entity_id: EntityID,
        component_data: T,
    ) -> Result<&mut T, String> {
        self.entity_component_directory
            .add_component_to_entity(entity_id, component_data)
    }

    fn remove_component_from_entity<T: ComponentTrait + ComponentDebugTrait + 'static>(
        &mut self,
        entity_id: EntityID,
    ) -> Result<(), String> {
        self.entity_component_directory
            .remove_component_from_entity::<T>(entity_id)
    }

    fn get_component_data(
        &self,
        component_data_id: &ComponentDataID,
    ) -> Result<&dyn ComponentTrait, String> {
        self.component_storage.get_component_data(component_data_id)
    }

    fn register_system(
        &mut self,
        name: &str,
        system: &'a mut dyn SystemTrait<SingleThreadedDatabase<'a>>,
    ) {
        self.system_runner.register_system(name, system)
    }
}
