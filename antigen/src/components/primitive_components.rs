use crate::{
    entity_component_system::{ComponentDebugTrait, ComponentTrait},
    primitive_types::ColorRGBF,
};
use std::fmt::Debug;

pub type CharComponent = PrimitiveComponent<char>;
pub type StringComponent = PrimitiveComponent<String>;
pub type StringListComponent = PrimitiveComponent<Vec<String>>;
pub type ColorComponent = PrimitiveComponent<ColorRGBF>;

#[derive(Debug, Copy, Clone)]
pub struct PrimitiveComponent<T>
where
    T: Debug + Clone + 'static,
{
    data: T,
}

impl<T> PrimitiveComponent<T>
where
    T: Debug + Clone + 'static,
{
    pub fn new(data: T) -> Self {
        PrimitiveComponent { data }
    }

    pub fn get_data(&self) -> &T {
        &self.data
    }

    pub fn set_data(&mut self, data: T) -> &mut Self {
        self.data = data;
        self
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

impl<T> ComponentDebugTrait for PrimitiveComponent<T>
where
    T: Debug + Clone + 'static,
{
    fn get_name() -> String {
        std::any::type_name::<T>().into()
    }

    fn get_description() -> String {
        format!(
            "Primitive component containing a {}",
            std::any::type_name::<T>()
        )
    }
}
