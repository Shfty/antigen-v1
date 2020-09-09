use std::{
    any::TypeId,
    collections::{HashMap, HashSet},
};

use crate::ecs::{AssemblageID, component::ComponentInterface, component::ComponentID, component::ComponentData, entity::EntityDebug, component::get_component_id};

use super::{
    ComponentDataID, ComponentMetadataTrait, ComponentTrait, EntityComponentDatabase, EntityID,
EntityComponentDatabaseDebug};

#[derive(Debug)]
pub struct SingleThreadedDatabase {
    entity_head: EntityID,
    component_data_head: ComponentDataID,
    assemblage_head: AssemblageID,

    entities: HashSet<EntityID>,
    components: HashMap<ComponentID, ComponentInterface>,
    component_data: HashMap<ComponentDataID, ComponentData>,

    entity_debug: HashMap<EntityID, EntityDebug>,
    entity_components: HashMap<EntityID, HashMap<ComponentID, ComponentDataID>>,
}

impl<'a> SingleThreadedDatabase {
    pub fn new() -> Self {
        SingleThreadedDatabase::default()
    }
}

impl<'a> Default for SingleThreadedDatabase {
    fn default() -> Self {
        SingleThreadedDatabase {
            entity_head: EntityID(0),
            component_data_head: ComponentDataID(0),
            assemblage_head: AssemblageID(0),

            entities: HashSet::new(),
            components: HashMap::new(),
            component_data: HashMap::new(),

            entity_debug: HashMap::new(),
            entity_components: HashMap::new(),
        }
    }
}

impl EntityComponentDatabase for SingleThreadedDatabase {
    fn is_component_registered<T: ComponentTrait + 'static>(&self) -> bool {
        self.is_component_registered_by_id(&get_component_id::<T>())
    }

    fn register_component<T: ComponentTrait + ComponentMetadataTrait + 'static>(
        &mut self,
    ) -> ComponentID {
        let component_id = ComponentID(TypeId::of::<T>());
        let component_name = T::get_name();
        let component_description = T::get_description();

        self.register_component_by_id(component_id, component_name, component_description);

        component_id
    }

    fn create_entity(&mut self, label: &str) -> EntityID {
        let entity_id: EntityID = self.entity_head;
        self.entities.insert(entity_id);
        self.entity_components.insert(entity_id, HashMap::new());

        self.entity_head += 1;

        let entity_debug = EntityDebug::new(label);
        self.entity_debug.insert(entity_id, entity_debug);

        entity_id
    }

    fn destroy_entity(&mut self, entity_id: EntityID) -> Result<(), String> {
        self.entity_debug.remove(&entity_id);
        self.entity_components.remove(&entity_id);
        self.entities.remove(&entity_id);

        Ok(())
    }

    fn add_registered_component_to_entity(
        &mut self,
        entity_id: EntityID,
        component_id: ComponentID,
        component_data: ComponentData,
    ) -> Result<ComponentDataID, String> {
        self.components
            .get(&component_id)
            .expect("Can't add unregistered component to entity");

        let component_data_id: ComponentDataID;
        if let Some(entity_components) = self.entity_components.get(&entity_id) {
            if let Some(existing_id) = entity_components.get(&component_id) {
                component_data_id = *existing_id;
            } else {
                component_data_id = self.component_data_head;
                self.component_data_head += 1;
            }
        } else {
            component_data_id = self.component_data_head;
            self.component_data_head += 1;
        }

        self.component_data
            .insert(component_data_id, component_data);

        let entity_components = match self.entity_components.get_mut(&entity_id) {
            Some(entity_components) => entity_components,
            None => return Err(format!("No such entity {}", entity_id)),
        };

        entity_components.insert(component_id, component_data_id);

        Ok(component_data_id)
    }

    fn remove_registered_component_from_entity(
        &mut self,
        entity_id: EntityID,
        component_id: ComponentID,
    ) -> Result<(), String> {
        let entity_components = match self.entity_components.get_mut(&entity_id) {
            Some(entity_components) => entity_components,
            None => return Err("No such entity component".into()),
        };

        let component_data_id = match entity_components.get(&component_id) {
            Some(component_data_id) => component_data_id,
            None => return Err("No such component data".into()),
        };

        if self.component_data.remove(component_data_id).is_none() {
            return Err(format!(
                "Failed to remove component from entity {}",
                &entity_id
            ));
        }

        if entity_components.remove(&component_id).is_none() {
            return Err(format!(
                "Failed to unlink component from entity {}",
                &entity_id
            ));
        }

        Ok(())
    }

    fn get_entities_by_predicate(&self, predicate: impl Fn(&EntityID) -> bool) -> Vec<EntityID> {
        self.entities.iter().copied().filter(predicate).collect()
    }

    fn entity_has_component<T: ComponentTrait + 'static>(&self, entity_id: &EntityID) -> bool {
        match self.entity_components.get(entity_id) {
            Some(components) => components.get(&get_component_id::<T>()).is_some(),
            None => false,
        }
    }

    fn get_entity_component<T: ComponentTrait + 'static>(
        &mut self,
        entity_id: EntityID,
    ) -> Result<&mut T, String> {
        let entity_components = match self.entity_components.get(&entity_id) {
            Some(entity_components) => entity_components,
            None => return Err("No such entity".into()),
        };

        let component_data_id = match entity_components.get(&get_component_id::<T>()) {
            Some(component_data_id) => *component_data_id,
            None => return Err("No such component".into()),
        };

        let component_data = match self.component_data.get_mut(&component_data_id) {
            Some(component_data) => component_data,
            None => return Err("No such component data".into()),
        };

        let component_data = match component_data.as_mut_any().downcast_mut::<T>() {
            Some(component_data) => component_data,
            None => return Err("Component type mismatch".into()),
        };

        Ok(component_data)
    }
}

impl EntityComponentDatabaseDebug for SingleThreadedDatabase {
    fn get_entity_label(&self, entity_id: EntityID) -> &str {
        match self.entity_debug.get(&entity_id) {
            Some(entity_debug) => &entity_debug.label,
            None => "Invalid Entity",
        }
    }

    fn get_entities(&self) -> Vec<&EntityID> {
        self.entities.iter().collect()
    }

    fn get_components(&self) -> Vec<(&ComponentID, &ComponentInterface)> {
        self.components.iter().collect()
    }

    fn get_component_data(&self) -> Vec<(&ComponentDataID, &ComponentData)> {
        self.component_data.iter().collect()
    }

    fn get_entity_components(&self) -> Vec<(&EntityID, Vec<(&ComponentID, &ComponentDataID)>)> {
        self.entity_components
            .iter()
            .map(|(entity_id, entity_components)| (entity_id, entity_components.iter().collect()))
            .collect()
    }
}

// Private Interface
impl SingleThreadedDatabase {
    fn is_component_registered_by_id(&self, component_id: &ComponentID) -> bool {
        self.components.get(component_id).is_some()
    }

    fn register_component_by_id(
        &mut self,
        component_id: ComponentID,
        name: &str,
        description: &str,
    ) {
        let component = ComponentInterface::new(name, description);
        self.components.insert(component_id, component);
    }
}
