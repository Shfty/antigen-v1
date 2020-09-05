use super::GUID;
use std::{any::Any, any::TypeId, fmt::Debug};

pub type ComponentID = TypeId;
pub type ComponentDataID = GUID;
pub type ComponentData = Box<dyn ComponentTrait>;

pub trait ComponentTrait: CloneComponentTrait + AnyComponentTrait + Debug {}

pub trait CloneComponentTrait {
    fn clone_component(&self) -> ComponentData;
}

impl<T> CloneComponentTrait for T
where
    T: ComponentTrait + Clone + 'static,
{
    fn clone_component(&self) -> ComponentData {
        Box::new(self.clone())
    }
}

pub trait AnyComponentTrait {
    fn as_any(&self) -> &dyn Any;
    fn as_mut_any(&mut self) -> &mut dyn Any;
}

impl<T> AnyComponentTrait for T
where
    T: ComponentTrait + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Clone for ComponentData {
    fn clone(&self) -> Self {
        self.clone_component()
    }
}

pub struct ComponentInterface {
    pub official_name: String,
    pub description: String,
    pub data_constructor: Box<dyn Fn() -> ComponentData>,
}

impl ComponentInterface {
    pub fn new<F: 'static>(
        official_name: &str,
        description: &str,
        data_constructor: F,
    ) -> ComponentInterface
    where
        F: Fn() -> ComponentData,
    {
        ComponentInterface {
            official_name: official_name.into(),
            description: description.into(),
            data_constructor: Box::new(data_constructor),
        }
    }
}

impl Debug for ComponentInterface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Component")
            .field("official_name", &self.official_name)
            .field("description", &self.description)
            .finish()
    }
}
