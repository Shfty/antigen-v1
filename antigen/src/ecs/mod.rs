use std::{
    any::TypeId,
    collections::{HashMap, HashSet},
    fmt::Debug,
};

mod assemblage;
mod component;
mod entity;
mod entity_query;
mod system;

pub use assemblage::AssemblageID;
pub use component::ComponentTrait;
pub use entity::EntityID;
pub use system::SystemTrait;

use assemblage::{Assemblage, AssemblageBuilder};
use component::{ComponentData, ComponentDataID, ComponentID, ComponentInterface};
use entity::EntityDebug;
use entity_query::EntityQueryBuilder;

type GUID = i64;

#[derive(Debug)]
pub struct ECS {
    entity_head: EntityID,
    component_data_head: ComponentDataID,
    assemblage_head: AssemblageID,

    entities: HashSet<EntityID>,
    components: HashMap<ComponentID, ComponentInterface>,
    component_data: HashMap<ComponentDataID, ComponentData>,

    entity_debug: HashMap<EntityID, EntityDebug>,
    entity_components: HashMap<EntityID, HashMap<ComponentID, ComponentDataID>>,

    assemblages: HashMap<AssemblageID, Assemblage>,
    assemblage_components: HashMap<AssemblageID, HashSet<ComponentID>>,
    assemblage_data: HashMap<AssemblageID, HashMap<ComponentID, ComponentData>>,
}

impl Default for ECS {
    fn default() -> Self {
        ECS {
            entity_head: 0,
            component_data_head: 0,
            assemblage_head: 0,

            entities: HashSet::new(),
            components: HashMap::new(),
            component_data: HashMap::new(),

            entity_debug: HashMap::new(),
            entity_components: HashMap::new(),

            assemblages: HashMap::new(),
            assemblage_components: HashMap::new(),
            assemblage_data: HashMap::new(),
        }
    }
}

impl ECS {
    // Public Interface
    pub fn get_component_id<T: ComponentTrait + 'static>() -> ComponentID {
        TypeId::of::<T>()
    }

    pub fn new() -> ECS {
        ECS::default()
    }

    pub fn register_component<T: ComponentTrait + Default + 'static>(
        &mut self,
        official_name: &str,
        description: &str,
    ) -> ComponentID {
        let component_id = TypeId::of::<T>();
        let component: ComponentInterface =
            ComponentInterface::new(official_name, description, || Box::new(T::default()));
        self.components.insert(component_id, component);

        component_id
    }

    pub fn build_assemblage(
        &mut self,
        official_name: &str,
        description: &str,
    ) -> AssemblageBuilder {
        AssemblageBuilder::new(self, official_name, description)
    }

    pub fn create_entity(&mut self, label: &str) -> EntityID {
        let entity_id: EntityID = self.entity_head;
        self.entities.insert(entity_id);
        self.entity_components.insert(entity_id, HashMap::new());
        self.entity_head += 1;

        let entity_debug = EntityDebug::new(label);
        self.entity_debug.insert(entity_id, entity_debug);

        entity_id
    }

    pub fn assemble_entity(
        &mut self,
        assemblage_id: AssemblageID,
        label: &str,
    ) -> Result<EntityID, String> {
        let entity_id = self.create_entity(label);

        let assemblage_components = match self.assemblage_components.get(&assemblage_id) {
            Some(assemblage_components) => assemblage_components,
            None => return Err("No such assemblage".into()),
        };

        let mut assemblage_data = match self.assemblage_data.get(&assemblage_id) {
            Some(assemblage_data) => assemblage_data.clone(),
            None => return Err("No such assemblage".into()),
        };

        let assemblage_components: Vec<ComponentID> =
            assemblage_components.iter().copied().collect();

        let mut pending_component_data: Vec<(ComponentID, ComponentData)> = Vec::new();
        for component_id in assemblage_components {
            match assemblage_data.remove(&component_id) {
                Some(component_data) => pending_component_data.push((component_id, component_data)),
                None => {
                    self.create_component_and_add_to_entity_by_id(entity_id, component_id)?;
                }
            }
        }

        for (component_id, component_data) in pending_component_data {
            self.add_component_to_entity(entity_id, component_id, component_data)?;
        }

        Ok(entity_id)
    }

    pub fn create_component_and_add_to_entity<T: ComponentTrait + 'static>(
        &mut self,
        entity_id: EntityID,
    ) -> Result<&mut T, String> {
        self.create_component_and_add_to_entity_by_id(entity_id, ECS::get_component_id::<T>())?;
        let component = self.get_entity_component::<T>(entity_id)?;
        let component = match component.as_mut_any().downcast_mut::<T>() {
            Some(component) => component,
            None => return Err("Component type mismatch".into()),
        };

        Ok(component)
    }

    pub fn get_entities_with_components(&mut self, components: &[ComponentID]) -> Vec<EntityID> {
        self.entities
            .iter()
            .filter(|entity_id| {
                !components
                    .iter()
                    .any(|component_id| !self.entity_has_component(**entity_id, *component_id))
            })
            .copied()
            .collect()
    }

    pub fn get_entity_component<T: ComponentTrait + 'static>(
        &mut self,
        entity_id: EntityID,
    ) -> Result<&mut T, String> {
        let entity_components = match self.entity_components.get(&entity_id) {
            Some(entity_components) => entity_components,
            None => return Err("No such entity".into()),
        };

        let component_data_id = match entity_components.get(&ECS::get_component_id::<T>()) {
            Some(component_data_id) => *component_data_id,
            None => return Err("No such component".into()),
        };

        let component_data = self.get_component_data_by_id(component_data_id)?;

        let component_data = match component_data.as_mut_any().downcast_mut::<T>() {
            Some(component_data) => component_data,
            None => return Err("Component type mismatch".into()),
        };

        Ok(component_data)
    }

    pub fn build_entity_query(&mut self) -> EntityQueryBuilder {
        EntityQueryBuilder::new(self)
    }

    // Private Methods
    fn get_component_by_id(&self, component_id: ComponentID) -> Option<&ComponentInterface> {
        self.components.get(&component_id)
    }

    fn get_component_data_by_id(
        &mut self,
        component_data_id: ComponentDataID,
    ) -> Result<&mut ComponentData, String> {
        match self.component_data.get_mut(&component_data_id) {
            Some(component_data) => Ok(component_data),
            None => Err("No such component data".into()),
        }
    }

    fn register_assemblage(
        &mut self,
        assemblage: Assemblage,
        component_data: HashMap<ComponentID, ComponentData>,
    ) -> AssemblageID {
        let assemblage_id = self.assemblage_head;

        self.assemblages.insert(assemblage_id, assemblage);

        self.assemblage_components
            .insert(assemblage_id, component_data.keys().copied().collect());

        self.assemblage_data.insert(assemblage_id, component_data);

        self.assemblage_head += 1;

        assemblage_id
    }

    fn add_component_data(&mut self, component_data: ComponentData) -> ComponentDataID {
        let component_data_id = self.component_data_head;
        self.component_data
            .insert(component_data_id, component_data);
        self.component_data_head += 1;
        component_data_id
    }

    fn add_component_to_entity(
        &mut self,
        entity_id: EntityID,
        component_id: ComponentID,
        component_data: ComponentData,
    ) -> Result<ComponentDataID, String> {
        if self.get_component_by_id(component_id).is_none() {
            return Err(format!(
                "Can't add unregistered component {:?} to entity {}",
                component_id, entity_id
            ));
        }

        let component_data_id = self.add_component_data(component_data);

        let entity_component = match self.entity_components.get_mut(&entity_id) {
            Some(entity_component) => entity_component,
            None => {
                return Err(format!(
                    "No such component {:?} on entity {}",
                    component_id, entity_id
                ))
            }
        };

        entity_component.insert(component_id, component_data_id);

        Ok(component_data_id)
    }

    fn create_component_and_add_to_entity_by_id(
        &mut self,
        entity_id: EntityID,
        component_id: ComponentID,
    ) -> Result<ComponentDataID, String> {
        let component = match self.get_component_by_id(component_id) {
            Some(component) => component,
            None => {
                return Err(format!(
                    "Can't add unregistered component {:?} to entity {}",
                    component_id, entity_id
                ))
            }
        };

        let constructor = &component.data_constructor;
        let component_data = constructor();
        let component_data_id =
            self.add_component_to_entity(entity_id, component_id, component_data)?;

        Ok(component_data_id)
    }

    fn entity_has_component(&self, entity_id: EntityID, component_id: ComponentID) -> bool {
        match self.entity_components.get(&entity_id) {
            Some(components) => components.get(&component_id).is_some(),
            None => false,
        }
    }
}
