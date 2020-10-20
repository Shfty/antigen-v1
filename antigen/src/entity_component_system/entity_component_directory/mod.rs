mod single_threaded_directory;

pub use single_threaded_directory::SingleThreadedDirectory;

use crate::entity_component_system::{ComponentDataID, ComponentID, ComponentTrait, EntityID};

pub trait EntityComponentDirectory {
    // CREATE
    fn create_entity(&mut self) -> Result<EntityID, String>;

    // DESTROY
    fn destroy_entity(&mut self, entity_id: EntityID) -> Result<(), String>;

    // EXIST
    fn is_valid_entity(&self, entity_id: &EntityID) -> bool;

    // GET
    fn get_entity_by_predicate(&self, predicate: impl Fn(&EntityID) -> bool) -> Option<EntityID>;
    fn get_entities_by_predicate(&self, predicate: impl Fn(&EntityID) -> bool) -> Vec<EntityID>;

    fn get_components_by_predicate(
        &self,
        predicate: impl Fn(&ComponentID) -> bool,
    ) -> Vec<ComponentID>;

    fn entity_has_component_by_id(&self, entity_id: &EntityID, component_id: &ComponentID) -> bool;

    fn get_entity_component_data_id(
        &self,
        entity_id: &EntityID,
        component_id: &ComponentID,
    ) -> Result<ComponentDataID, String>;
}
