use std::collections::{HashMap, HashSet};

use super::ComponentID;

use super::{ComponentDataID, ComponentTrait, EntityComponentDirectory, EntityID};

pub struct SingleThreadedDirectory {
    entities: HashSet<EntityID>,
    components: HashSet<ComponentID>,
    entity_components: HashMap<EntityID, HashMap<ComponentID, ComponentDataID>>,
}

impl SingleThreadedDirectory {
    pub fn new() -> Self {
        SingleThreadedDirectory {
            entities: HashSet::new(),
            components: HashSet::new(),
            entity_components: HashMap::new(),
        }
    }

    pub fn get_entity_component_data_id_by_type<T>(
        &self,
        entity_id: EntityID,
    ) -> Result<ComponentDataID, String>
    where
        T: ComponentTrait + 'static,
    {
        self.get_entity_component_data_id(&entity_id, &ComponentID::get::<T>())
    }
}

impl Default for SingleThreadedDirectory {
    fn default() -> Self {
        SingleThreadedDirectory::new()
    }
}

impl EntityComponentDirectory for SingleThreadedDirectory {
    // CREATE
    fn create_entity(&mut self) -> Result<EntityID, String> {
        let entity_id: EntityID = EntityID::next();
        self.entities.insert(entity_id);
        self.entity_components.insert(entity_id, HashMap::new());

        Ok(entity_id)
    }

    // DESTROY
    fn destroy_entity(&mut self, entity_id: EntityID) -> Result<(), String> {
        self.entity_components.remove(&entity_id);
        self.entities.remove(&entity_id);

        Ok(())
    }

    // EXIST
    fn is_valid_entity(&self, entity_id: &EntityID) -> bool {
        self.entities.contains(entity_id)
    }

    fn entity_has_component_by_id(&self, entity_id: &EntityID, component_id: &ComponentID) -> bool {
        match self.entity_components.get(entity_id) {
            Some(components) => components.get(component_id).is_some(),
            None => false,
        }
    }

    // GET
    fn get_entity_by_predicate(&self, predicate: impl Fn(&EntityID) -> bool) -> Option<EntityID> {
        self.entities.iter().copied().find(predicate)
    }

    fn get_entities_by_predicate(&self, predicate: impl Fn(&EntityID) -> bool) -> Vec<EntityID> {
        self.entities.iter().copied().filter(predicate).collect()
    }

    fn get_components_by_predicate(
        &self,
        predicate: impl Fn(&ComponentID) -> bool,
    ) -> Vec<ComponentID> {
        self.components.iter().copied().filter(predicate).collect()
    }

    fn get_entity_component_data_id(
        &self,
        entity_id: &EntityID,
        component_id: &ComponentID,
    ) -> Result<ComponentDataID, String> {
        let entity_components = match self.entity_components.get(&entity_id) {
            Some(entity_components) => entity_components,
            None => panic!(
                "Error getting entity component data ID: No such entity {}",
                entity_id
            ),
        };

        match entity_components.get(&component_id) {
            Some(component_data_id) => Ok(*component_data_id),
            None => Err(format!(
                "Error getting entity {} component data ID: No such component {}",
                entity_id,
                component_id
            )),
        }
    }
}
