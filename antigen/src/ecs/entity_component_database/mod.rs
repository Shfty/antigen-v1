use super::{
    component::{get_component_id, ComponentData, ComponentID, ComponentInterface},
    ComponentMetadataTrait, ComponentTrait, EntityID,
ComponentDataID};

mod single_threaded_database;

pub use single_threaded_database::SingleThreadedDatabase;

pub trait EntityComponentDatabase {
    fn is_component_registered<T: ComponentTrait + 'static>(&self) -> bool;

    fn register_component<T: ComponentTrait + ComponentMetadataTrait + 'static>(
        &mut self,
    ) -> ComponentID;

    fn create_entity(&mut self, label: &str) -> EntityID;
    fn destroy_entity(&mut self, entity_id: EntityID) -> Result<(), String>;

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

    fn remove_component_from_entity<T: ComponentTrait + ComponentMetadataTrait + 'static>(
        &mut self,
        entity_id: EntityID,
    ) -> Result<(), String> {
        self.remove_registered_component_from_entity(entity_id, get_component_id::<T>())
    }

    fn remove_registered_component_from_entity(
        &mut self,
        entity_id: EntityID,
        component_id: ComponentID,
    ) -> Result<(), String>;

    fn add_registered_component_to_entity(
        &mut self,
        entity_id: EntityID,
        component_id: ComponentID,
        component_data: ComponentData,
    ) -> Result<ComponentDataID, String>;

    fn get_entities_by_predicate(&self, predicate: impl Fn(&EntityID) -> bool) -> Vec<EntityID>;

    fn entity_has_component<T: ComponentTrait + 'static>(&self, entity_id: &EntityID) -> bool;

    fn get_entity_component<T: ComponentTrait + 'static>(
        &mut self,
        entity_id: EntityID,
    ) -> Result<&mut T, String>;
}

pub trait EntityComponentDatabaseDebug {
    fn get_entity_label(&self, entity_id: EntityID) -> &str;
    fn get_entities(&self) -> Vec<&EntityID>;
    fn get_components(&self) -> Vec<(&ComponentID, &ComponentInterface)>;
    fn get_component_data(&self) -> Vec<(&ComponentDataID, &ComponentData)>;
    fn get_entity_components(&self) -> Vec<(&EntityID, Vec<(&ComponentID, &ComponentDataID)>)>;
}
