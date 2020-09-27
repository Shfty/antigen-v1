use std::{borrow::Borrow, borrow::BorrowMut, fmt::Debug};

use crate::entity_component_system::{ComponentDebugTrait, ComponentTrait};

#[derive(Debug, Clone)]
pub struct EventQueue<T>(Vec<T>)
where
    T: Debug;

impl<T> Borrow<Vec<T>> for EventQueue<T>
where
    T: Debug,
{
    fn borrow(&self) -> &Vec<T> {
        &self.0
    }
}

impl<T> BorrowMut<Vec<T>> for EventQueue<T>
where
    T: Debug,
{
    fn borrow_mut(&mut self) -> &mut Vec<T> {
        &mut self.0
    }
}

impl<T> Default for EventQueue<T>
where
    T: Debug,
{
    fn default() -> Self {
        EventQueue(Vec::new())
    }
}

impl<T> ComponentTrait for EventQueue<T> where T: Debug + 'static {}

impl<T> ComponentDebugTrait for EventQueue<T>
where
    T: Debug,
{
    fn get_name() -> String {
        format!("Event Queue ({})", std::any::type_name::<T>())
    }

    fn get_description() -> String {
        format!(
            "Event queue for objects of type {}",
            std::any::type_name::<T>()
        )
    }
}
