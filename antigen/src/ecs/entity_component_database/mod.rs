use super::{
    component::ComponentID, ComponentDataID, ComponentDebugTrait, ComponentTrait, EntityID,
EntityCreateCallback, ComponentCreateCallback};

mod single_threaded_database;

pub use single_threaded_database::SingleThreadedDatabase;

pub trait EntityComponentDatabase {
    fn is_valid_entity(&self, entity_id: &EntityID) -> bool;
    fn is_valid_component<T: ComponentTrait + 'static>(&self) -> bool;

    fn register_component<T: ComponentTrait + ComponentDebugTrait + 'static>(
        &mut self,
    ) -> Result<ComponentID, String>;

    fn register_entity_create_callback(&mut self, callback: EntityCreateCallback<Self>);
    fn register_component_create_callback(&mut self, callback: ComponentCreateCallback<Self>);

    fn create_entity(&mut self, debug_label: Option<&str>) -> Result<EntityID, String>;
    fn destroy_entity(&mut self, entity_id: EntityID) -> Result<(), String>;

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

    fn remove_registered_component_from_entity(
        &mut self,
        entity_id: EntityID,
        component_id: ComponentID,
    ) -> Result<(), String>;

    fn add_registered_component_to_entity(
        &mut self,
        entity_id: EntityID,
        component_id: ComponentID,
        component_data: Box<dyn ComponentTrait>,
    ) -> Result<ComponentDataID, String>;

    fn get_entity_by_predicate(&self, predicate: impl Fn(&EntityID) -> bool) -> Option<EntityID>;
    fn get_entities_by_predicate(&self, predicate: impl Fn(&EntityID) -> bool) -> Vec<EntityID>;

    fn get_components_by_predicate(
        &self,
        predicate: impl Fn(&ComponentID) -> bool,
    ) -> Vec<ComponentID>;

    fn entity_has_component_by_id(&self, entity_id: &EntityID, component_id: &ComponentID) -> bool;
    fn entity_has_component<T: ComponentTrait + 'static>(&self, entity_id: &EntityID) -> bool {
        self.entity_has_component_by_id(entity_id, &ComponentID::get::<T>())
    }

    fn get_entity_component<T: ComponentTrait + 'static>(
        &self,
        entity_id: EntityID,
    ) -> Result<&T, String>;

    fn get_entity_component_data_id(
        &self,
        entity_id: &EntityID,
        component_id: &ComponentID,
    ) -> Result<ComponentDataID, String>;

    fn get_entity_component_mut<T: ComponentTrait + 'static>(
        &mut self,
        entity_id: EntityID,
    ) -> Result<&mut T, String>;

    fn get_component_data(
        &self,
        component_data_id: &ComponentDataID,
    ) -> Result<&dyn ComponentTrait, String>;
}
