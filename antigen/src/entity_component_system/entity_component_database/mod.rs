use super::{ComponentDebugTrait, ComponentID, ComponentTrait, EntityID};

mod assemblage;
mod callback_manager;
mod component_storage;
mod entity_component_directory;

pub use assemblage::{Assemblage, AssemblageID};
pub use callback_manager::CallbackManager;
use component_storage::ComponentDropCallback;
pub use component_storage::{ComponentDataID, ComponentStorage, HeapComponentStorage};
pub use entity_component_directory::{EntityComponentDirectory, SingleThreadedDirectory};

/// Ties together component data storage, entity-component lookup, and callback handling
pub struct EntityComponentDatabase<CS: ComponentStorage, CD: EntityComponentDirectory> {
    pub component_storage: CS,
    pub entity_component_directory: CD,
    pub callback_manager: CallbackManager<CS, CD>,
}

impl<S, D> EntityComponentDatabase<S, D>
where
    S: ComponentStorage,
    D: EntityComponentDirectory,
{
    pub fn new(component_storage: S, entity_component_directory: D) -> Self {
        EntityComponentDatabase {
            component_storage,
            entity_component_directory,
            callback_manager: CallbackManager::new(),
        }
    }
}

impl<'a, S, D> EntityComponentDatabase<S, D>
where
    S: ComponentStorage,
    D: EntityComponentDirectory,
{
    // DESTROY
    pub fn destroy_entity(&mut self, entity_id: EntityID) -> Result<(), String> {
        // TODO: Destroy component data

        self.entity_component_directory.destroy_entity(entity_id)
    }

    pub fn destroy_component<T>(&mut self) -> Result<(), String>
    where
        T: ComponentTrait + ComponentDebugTrait + 'static,
    {
        // TODO: Destroy component data

        self.entity_component_directory.destroy_component::<T>()
    }
}
