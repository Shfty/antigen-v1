use crate::primitive_types::UID;
use std::{
    any::Any,
    any::TypeId,
    fmt::{Debug, Display},
    ops::{Add, AddAssign},
};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ComponentID(pub TypeId);

impl Display for ComponentID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ComponentID(component_id) = self;
        write!(f, "{:?}", component_id)
    }
}

pub fn get_component_id<T: ComponentTrait + 'static>() -> ComponentID {
    ComponentID(TypeId::of::<T>())
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ComponentDataID(pub UID);

impl Add<UID> for ComponentDataID {
    type Output = ComponentDataID;

    fn add(self, rhs: i64) -> Self::Output {
        let ComponentDataID(self_id) = self;
        ComponentDataID(self_id + rhs)
    }
}

impl AddAssign<UID> for ComponentDataID {
    fn add_assign(&mut self, rhs: UID) {
        let ComponentDataID(self_id) = self;
        *self_id = *self_id + rhs;
    }
}

pub type ComponentData = Box<dyn ComponentTrait>;

pub trait ComponentTrait: CloneComponentTrait + AnyComponentTrait + Debug {}

pub trait ComponentMetadataTrait {
    fn get_name() -> &'static str;
    fn get_description() -> &'static str;
}

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
}

impl ComponentInterface {
    pub fn new(official_name: &str, description: &str) -> ComponentInterface {
        ComponentInterface {
            official_name: official_name.into(),
            description: description.into(),
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
