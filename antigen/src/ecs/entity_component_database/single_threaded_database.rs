use std::{
    any::TypeId,
    collections::{HashMap, HashSet},
};

use crate::{
    components::ComponentDebugComponent, components::DebugExcludeComponent,
    components::EntityDebugComponent, ecs::ComponentID, ecs::ComponentStorage,
    ecs::HeapComponentStorage,
};

use super::{
    ComponentCreateCallback, ComponentDataID, ComponentDebugTrait, ComponentTrait,
    EntityComponentDatabase, EntityCreateCallback, EntityID,
};

pub struct SingleThreadedDatabase<'a> {
    entities: HashSet<EntityID>,
    components: HashSet<ComponentID>,
    entity_components: HashMap<EntityID, HashMap<ComponentID, ComponentDataID>>,

    entity_create_callbacks: Vec<EntityCreateCallback<Self>>,
    component_create_callbacks: Vec<ComponentCreateCallback<Self>>,

    component_storage: &'a mut dyn ComponentStorage,
}

impl<'a> SingleThreadedDatabase<'a> {
    pub fn new(component_storage: &'a mut dyn ComponentStorage) -> Result<Self, String> {
        let mut db = SingleThreadedDatabase {
            entities: HashSet::new(),
            components: HashSet::new(),
            entity_components: HashMap::new(),
            component_storage,

            entity_create_callbacks: Vec::new(),
            component_create_callbacks: Vec::new(),
        };

        {
            let entity_debug_entity = db.create_entity(None).unwrap();

            db.add_component_to_entity(entity_debug_entity, EntityDebugComponent::default())?
                .register_entity(entity_debug_entity, "Entity Debug".into());

            db.add_component_to_entity(entity_debug_entity, DebugExcludeComponent)?;
        }

        {
            let component_debug_entity = db.create_entity("Component Debug".into()).unwrap();

            db.add_component_to_entity(component_debug_entity, ComponentDebugComponent::default())?;
            db.add_component_to_entity(component_debug_entity, DebugExcludeComponent)?;
        }

        Ok(db)
    }

    pub fn get_entity_component_data_id_by_type<T>(
        &self,
        entity_id: EntityID,
    ) -> Result<ComponentDataID, String>
    where
        T: ComponentTrait + 'static,
    {
        self.get_entity_component_data_id_by_id(entity_id, ComponentID::get::<T>())
    }

    pub fn get_entity_component_data_id_by_id(
        &self,
        entity_id: EntityID,
        component_id: ComponentID,
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
                "Error getting entity component data ID: No such component {}",
                component_id
            )),
        }
    }

    pub fn insert_entity_component(
        &mut self,
        entity_id: &EntityID,
        component_id: ComponentID,
        component_data_id: ComponentDataID,
    ) -> Result<ComponentDataID, String> {
        let entity_components = self
            .entity_components
            .get_mut(entity_id)
            .expect(&format!("No such entity {}", entity_id));
        entity_components.insert(component_id, component_data_id);
        Ok(component_data_id)
    }

    pub fn remove_entity_component(
        &mut self,
        entity_id: &EntityID,
        component_id: &ComponentID,
    ) -> Result<(), String> {
        let entity_components = self
            .entity_components
            .get_mut(&entity_id)
            .expect(&format!("No such entity {}", entity_id));
        match entity_components.remove(component_id) {
            Some(_) => Ok(()),
            None => Err("No such component".into()),
        }
    }
}

impl EntityComponentDatabase for SingleThreadedDatabase<'_> {
    fn is_valid_entity(&self, entity_id: &EntityID) -> bool {
        self.entities.contains(entity_id)
    }

    fn is_valid_component<T: ComponentTrait + 'static>(&self) -> bool {
        self.components.get(&ComponentID::get::<T>()).is_some()
    }

    fn register_component<T: ComponentTrait + ComponentDebugTrait + 'static>(
        &mut self,
    ) -> Result<ComponentID, String> {
        let component_id = ComponentID(TypeId::of::<T>());
        self.components.insert(component_id);

        for callback in &self.component_create_callbacks.clone() {
            callback(self, component_id, &T::get_name(), &T::get_description());
        }

        Ok(component_id)
    }

    fn register_entity_create_callback(&mut self, callback: EntityCreateCallback<Self>) {
        self.entity_create_callbacks.push(callback);
    }

    fn register_component_create_callback(&mut self, callback: ComponentCreateCallback<Self>) {
        self.component_create_callbacks.push(callback);
    }

    fn create_entity(&mut self, debug_label: Option<&str>) -> Result<EntityID, String> {
        let entity_id: EntityID = EntityID::next();
        self.entities.insert(entity_id);
        self.entity_components.insert(entity_id, HashMap::new());

        for callback in &self.entity_create_callbacks.clone() {
            callback(self, entity_id, debug_label);
        }

        Ok(entity_id)
    }

    fn destroy_entity(&mut self, entity_id: EntityID) -> Result<(), String> {
        if let Some(components) = self.entity_components.get(&entity_id) {
            for (component_id, _) in components.clone() {
                self.remove_registered_component_from_entity(entity_id, component_id)?;
            }
        }

        self.entity_components.remove(&entity_id);
        self.entities.remove(&entity_id);

        Ok(())
    }

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

    fn entity_has_component<T: ComponentTrait + 'static>(&self, entity_id: &EntityID) -> bool {
        self.entity_has_component_by_id(entity_id, &ComponentID::get::<T>())
    }

    fn entity_has_component_by_id(&self, entity_id: &EntityID, component_id: &ComponentID) -> bool {
        match self.entity_components.get(entity_id) {
            Some(components) => components.get(component_id).is_some(),
            None => false,
        }
    }

    fn get_entity_component_data_id(
        &self,
        entity_id: &EntityID,
        component_id: &ComponentID,
    ) -> Result<ComponentDataID, String> {
        match self.entity_components.get(entity_id) {
            Some(entity_components) => match entity_components.get(component_id) {
                Some(component_data_id) => Ok(*component_data_id),
                None => Err(format!(
                    "Failed to get entity component data ID: No such component {} on entity {}",
                    component_id, entity_id
                )),
            },
            None => panic!(
                "Failed to get entity component data ID: No such entity {}",
                entity_id
            ),
        }
    }

    fn add_component_to_entity<T: ComponentTrait + ComponentDebugTrait + 'static>(
        &mut self,
        entity_id: EntityID,
        component_data: T,
    ) -> Result<&mut T, String> {
        if !self.is_valid_component::<T>() {
            self.register_component::<T>()?;
        }

        self.add_registered_component_to_entity(
            entity_id,
            ComponentID::get::<T>(),
            Box::new(component_data),
        )?;

        let component = self.get_entity_component_mut::<T>(entity_id)?;
        let component = match component.as_mut_any().downcast_mut::<T>() {
            Some(component) => component,
            None => return Err("Component type mismatch".into()),
        };

        Ok(component)
    }

    fn remove_component_from_entity<T: ComponentTrait + ComponentDebugTrait + 'static>(
        &mut self,
        entity_id: EntityID,
    ) -> Result<(), String> {
        self.remove_registered_component_from_entity(entity_id, ComponentID::get::<T>())
    }

    // TODO: Remove
    fn get_entity_component<T: ComponentTrait + 'static>(
        &self,
        entity_id: EntityID,
    ) -> Result<&T, String> {
        let component_data_id = self.get_entity_component_data_id_by_type::<T>(entity_id)?;

        let component_data = match self
            .component_storage
            .get_component_data(&component_data_id)
        {
            Ok(component_data) => component_data,
            Err(err) => {
                return Err(format!(
                    "Error getting component for entity {}: {}",
                    entity_id, err
                ))
            }
        };

        let component_data = match component_data.as_any().downcast_ref::<T>() {
            Some(component_data) => component_data,
            None => return Err("Error getting entity component: Component type mismatch".into()),
        };

        Ok(component_data)
    }

    fn get_entity_component_mut<T: ComponentTrait + 'static>(
        &mut self,
        entity_id: EntityID,
    ) -> Result<&mut T, String> {
        let component_data_id = self.get_entity_component_data_id_by_type::<T>(entity_id)?;

        let component_data = match self
            .component_storage
            .get_component_data_mut(&component_data_id)
        {
            Ok(component_data) => component_data,
            Err(err) => {
                return Err(format!(
                    "Error getting mutable component for entity {}: {}",
                    entity_id, err
                ))
            }
        };

        let component_data = match component_data.as_mut_any().downcast_mut::<T>() {
            Some(component_data) => component_data,
            None => {
                return Err(
                    "Error getting mutable entity component: Component type mismatch".into(),
                )
            }
        };

        Ok(component_data)
    }

    fn add_registered_component_to_entity(
        &mut self,
        entity_id: EntityID,
        component_id: ComponentID,
        component_data: Box<dyn ComponentTrait>,
    ) -> Result<ComponentDataID, String> {
        self.components
            .get(&component_id)
            .expect("Can't add unregistered component to entity");

        let component_data_id = self.component_storage.insert_component(component_data)?;

        self.insert_entity_component(&entity_id, component_id, component_data_id)
    }

    fn remove_registered_component_from_entity(
        &mut self,
        entity_id: EntityID,
        component_id: ComponentID,
    ) -> Result<(), String> {
        self.components
            .get(&component_id)
            .expect("Can't remove unregistered component from entity");

        let entity_components = match self.entity_components.get_mut(&entity_id) {
            Some(entity_components) => entity_components,
            None => {
                return Err(
                    "Error removing registered component from entity: No such entity component"
                        .into(),
                )
            }
        };

        let component_data_id = match entity_components.get(&component_id) {
            Some(component_data_id) => component_data_id,
            None => {
                return Err(
                    "Error removing registered component from entity: No such component data"
                        .into(),
                )
            }
        };

        let component_data_id = self.get_entity_component_data_id_by_id(entity_id, component_id)?;

        self.component_storage
            .remove_component_data(&component_id, &component_data_id)?;

        self.remove_entity_component(&entity_id, &component_id)?;

        Ok(())
    }

    fn get_component_data(
        &self,
        component_data_id: &ComponentDataID,
    ) -> Result<&dyn ComponentTrait, String> {
        self.component_storage.get_component_data(component_data_id)
    }
}
