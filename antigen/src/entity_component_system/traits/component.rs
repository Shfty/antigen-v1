use std::{
    any::Any,
    any::TypeId,
    fmt::{Debug, Display},
};

/// Type-based unique component ID
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ComponentID {
    pub type_id: TypeId,
    pub type_name: &'static str,
}

impl ComponentID {
    pub fn get<T: ComponentTrait + 'static>() -> Self {
        ComponentID {
            type_id: TypeId::of::<T>(),
            type_name: if true {
                std::any::type_name::<T>()
                    .split("::")
                    .last()
                    .expect("Failed to split component type name")
            } else {
                std::any::type_name::<T>()
            },
        }
    }
}

impl Display for ComponentID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.type_name)
    }
}

/// Base component trait
pub trait ComponentTrait: Debug + UpcastComponentTrait {}

impl<T> ComponentTrait for T where T: Debug + Any {}

/// Trait for upcasting a component to an Any reference
pub trait UpcastComponentTrait: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_mut_any(&mut self) -> &mut dyn Any;
}

impl<T> UpcastComponentTrait for T
where
    T: ComponentTrait,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

/// Trait for downcasting an Any reference to a concrete component type
pub trait DowncastComponentTrait<T: UpcastComponentTrait> {
    fn as_data(component: &dyn ComponentTrait) -> &T;
    fn as_mut_data(component: &mut dyn ComponentTrait) -> &mut T;
}

impl<T> DowncastComponentTrait<T> for T
where
    T: UpcastComponentTrait,
{
    fn as_data(component: &dyn ComponentTrait) -> &T {
        component.as_any().downcast_ref::<T>().unwrap_or_else(|| {
            panic!(
                "Failed to downcast component to type {}",
                std::any::type_name::<T>()
            )
        })
    }

    fn as_mut_data(component: &mut dyn ComponentTrait) -> &mut T {
        component
            .as_mut_any()
            .downcast_mut::<T>()
            .unwrap_or_else(|| {
                panic!(
                    "Failed to downcast component to type {}",
                    std::any::type_name::<T>()
                )
            })
    }
}
