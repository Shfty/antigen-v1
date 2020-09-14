use crate::uid::UID;
use std::{
    any::Any,
    any::TypeId,
    fmt::{Debug, Display},
};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ComponentID(pub TypeId);

impl ComponentID {
    pub fn get<T: ComponentTrait + 'static>() -> Self {
        ComponentID(TypeId::of::<T>())
    }
}

impl Display for ComponentID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ComponentID(component_id) = self;
        write!(f, "{:?}", component_id)
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ComponentDataID(pub UID);

impl ComponentDataID {
    pub fn next() -> Self {
        ComponentDataID(crate::uid::new())
    }
}

impl Display for ComponentDataID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ComponentDataID(component_data_id) = self;
        write!(f, "{}", component_data_id)
    }
}

pub trait ComponentTrait: CloneComponentTrait + AnyComponentTrait + Debug {}

pub trait ComponentDebugTrait {
    fn get_name() -> String;
    fn get_description() -> String;
}

pub trait CloneComponentTrait {
    fn clone_component(&self) -> Box<dyn ComponentTrait>;
}

impl<T> CloneComponentTrait for T
where
    T: ComponentTrait + Clone + 'static,
{
    fn clone_component(&self) -> Box<dyn ComponentTrait> {
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

impl Clone for Box<dyn ComponentTrait> {
    fn clone(&self) -> Self {
        self.clone_component()
    }
}
