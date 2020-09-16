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

pub trait ComponentTrait: AnyComponentTrait + Debug {}

pub trait ComponentDebugTrait {
    fn get_name() -> String;
    fn get_description() -> String;
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
