use crate::ecs::{ComponentMetadataTrait, ComponentTrait};
use std::fmt::Debug;

pub type CharComponent = PrimitiveComponent<char>;
pub type StringComponent = PrimitiveComponent<String>;

#[derive(Debug, Copy, Clone)]
pub struct PrimitiveComponent<T>
where
    T: Debug + Clone + 'static,
{
    pub data: T,
}

impl<T> PrimitiveComponent<T>
where
    T: Debug + Clone + 'static,
{
    pub fn new(data: T) -> Self {
        PrimitiveComponent { data }
    }
}

impl<T> Default for PrimitiveComponent<T>
where
    T: Debug + Default + Clone + 'static,
{
    fn default() -> Self {
        PrimitiveComponent::new(T::default())
    }
}

impl<T> ComponentTrait for PrimitiveComponent<T> where T: Debug + Clone + 'static {}

impl<T> ComponentMetadataTrait for PrimitiveComponent<T> where T: Debug + Clone + 'static {
    fn get_name() -> &'static str {
        "Primitive"
    }

    fn get_description() -> &'static str {
        "Primitive Type Component"
    }
}