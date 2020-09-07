use std::{
    any::TypeId,
    collections::{HashMap, HashSet},
    fmt::Debug,
};

mod assemblage;
mod component;
mod entity;
mod system;

pub mod components;
pub mod systems;

pub use assemblage::AssemblageID;
pub use component::{ComponentMetadataTrait, ComponentTrait};
pub use entity::EntityID;
pub use system::SystemTrait;

use assemblage::{Assemblage, AssemblageBuilder};
use component::{
    get_component_id, ComponentData, ComponentDataID, ComponentID, ComponentInterface,
};
use entity::EntityDebug;

pub trait ECS: Sized {
    fn is_component_registered<T: ComponentTrait + 'static>(&self) -> bool;

    fn register_component<T: ComponentTrait + ComponentMetadataTrait + 'static>(
        &mut self,
    ) -> ComponentID;

    fn register_assemblage(
        &mut self,
        assemblage: Assemblage,
        component_data: HashMap<ComponentID, ComponentData>,
    ) -> AssemblageID;

    fn build_assemblage(
        &mut self,
        official_name: &str,
        description: &str,
    ) -> AssemblageBuilder<Self>;

    fn create_entity(&mut self, label: &str) -> EntityID;

    fn assemble_entity(
        &mut self,
        assemblage_id: AssemblageID,
        label: &str,
    ) -> Result<EntityID, String>;

    fn add_component_to_entity<T: ComponentTrait + ComponentMetadataTrait + 'static>(
        &mut self,
        entity_id: EntityID,
        component_data: T,
    ) -> Result<&mut T, String>;

    fn get_entities_by_predicate(&self, predicate: impl Fn(&EntityID) -> bool) -> Vec<EntityID>;

    fn entity_has_component<T: ComponentTrait + 'static>(&self, entity_id: &EntityID) -> bool;

    fn get_entity_component<T: ComponentTrait + 'static>(
        &mut self,
        entity_id: EntityID,
    ) -> Result<&mut T, String>;
}

#[derive(Debug)]
pub struct SingleThreadedECS {
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

impl SingleThreadedECS {
    pub fn new() -> Self {
        SingleThreadedECS::default()
    }
}

impl Default for SingleThreadedECS {
    fn default() -> Self {
        SingleThreadedECS {
            entity_head: EntityID(0),
            component_data_head: ComponentDataID(0),
            assemblage_head: AssemblageID(0),

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

impl ECS for SingleThreadedECS {
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

    fn build_assemblage(
        &mut self,
        official_name: &str,
        description: &str,
    ) -> AssemblageBuilder<Self> {
        AssemblageBuilder::new(self, official_name, description)
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

    fn assemble_entity(
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
                    return Err(format!(
                        "No assemblage data for component {:?}",
                        component_id
                    ))
                }
            }
        }

        for (component_id, component_data) in pending_component_data {
            self.add_registered_component_to_entity(entity_id, component_id, component_data)?;
        }

        Ok(entity_id)
    }

    fn add_component_to_entity<T: ComponentTrait + ComponentMetadataTrait + 'static>(
        &mut self,
        entity_id: EntityID,
        component_data: T,
    ) -> Result<&mut T, String> {
        if !self.is_component_registered::<T>() {
            self.register_component::<T>();
        }

        self.add_registered_component_to_entity(
            entity_id,
            get_component_id::<T>(),
            Box::new(component_data),
        )?;

        let component = self.get_entity_component::<T>(entity_id)?;
        let component = match component.as_mut_any().downcast_mut::<T>() {
            Some(component) => component,
            None => return Err("Component type mismatch".into()),
        };

        Ok(component)
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

// Private Interface
impl SingleThreadedECS {
    fn get_entity_label(&self, entity_id: EntityID) -> &str {
        match self.entity_debug.get(&entity_id) {
            Some(entity_debug) => &entity_debug.label,
            None => "Invalid Entity",
        }
    }

    fn get_entities(&self) -> &HashSet<EntityID> {
        &self.entities
    }

    fn get_components(&self) -> &HashMap<ComponentID, ComponentInterface> {
        &self.components
    }

    fn get_component_data(&self) -> &HashMap<ComponentDataID, ComponentData> {
        &self.component_data
    }

    fn get_entity_components(&self) -> &HashMap<EntityID, HashMap<ComponentID, ComponentDataID>> {
        &self.entity_components
    }

    fn get_assemblages(&self) -> &HashMap<AssemblageID, Assemblage> {
        &self.assemblages
    }

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

    fn add_registered_component_to_entity(
        &mut self,
        entity_id: EntityID,
        component_id: ComponentID,
        component_data: ComponentData,
    ) -> Result<ComponentDataID, String> {
        self.components
            .get(&component_id)
            .expect("Can't add unregistered component to entity");

        let component_data_id = self.component_data_head;
        self.component_data
            .insert(component_data_id, component_data);

        self.component_data_head += 1;

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
}
