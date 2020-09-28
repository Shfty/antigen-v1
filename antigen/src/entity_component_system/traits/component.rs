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
            type_name: std::any::type_name::<T>(),
        }
    }
}

impl ComponentID {
    pub fn get_name(&self) -> String {
        Self::strip_namespaces(self.type_name)
    }

    /// __NOTE:__
    /// Depends on <std::any::type_name> returning strings in the form `crate::subcrate::type<crate::subcrate::type>`,
    /// which may change in future versions of Rust
    // (Ideally this should be codified internally, but for now avoiding manual ComponentTrait impls is preferred)
    fn strip_namespaces(string: &str) -> String {
        let before: &str;
        let after: Option<&str>;

        if let Some(open_bracket) = string.find('<') {
            let (split_before, split_after) = string.split_at(open_bracket);
            before = split_before;
            after = Some(split_after);
        } else {
            before = string;
            after = None;
        }

        let before = before.split("::").last().unwrap();
        if let Some(after) = after {
            before.to_string() + "<" + &Self::strip_namespaces(&after[1..after.len() - 1]) + ">"
        } else {
            before.into()
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
