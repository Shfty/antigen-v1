use std::{any::Any, fmt::Debug};

use store::TypeKey;

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
pub trait DowncastComponentTrait {
    type Target: UpcastComponentTrait;

    fn as_data(component: &dyn ComponentTrait) -> &Self::Target;
    fn as_mut_data(component: &mut dyn ComponentTrait) -> &mut Self::Target;
}

impl<T> DowncastComponentTrait for T
where
    T: UpcastComponentTrait,
{
    type Target = T;

    fn as_data(component: &dyn ComponentTrait) -> &T {
        component.as_any().downcast_ref::<T>().unwrap_or_else(|| {
            panic!(
                "Failed to downcast component to type {}",
                TypeKey::of::<T>().get_name()
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
                    TypeKey::of::<T>().get_name()
                )
            })
    }
}
