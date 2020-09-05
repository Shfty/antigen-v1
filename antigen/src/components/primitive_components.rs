use crate::ecs::ComponentTrait;
use std::fmt::Debug;

pub type CharComponent = PrimitiveComponent<char>;
pub type StringSliceComponent<'a> = PrimitiveComponent<&'a str>;
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
