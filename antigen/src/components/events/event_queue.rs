use std::fmt::Debug;

use crate::entity_component_system::ComponentDebugTrait;

#[derive(Debug, Clone)]
pub struct EventQueue<T>(Vec<T>)
where
    T: Debug;

impl<T> AsRef<Vec<T>> for EventQueue<T>
where
    T: Debug,
{
    fn as_ref(&self) -> &Vec<T> {
        &self.0
    }
}

impl<T> AsMut<Vec<T>> for EventQueue<T>
where
    T: Debug,
{
    fn as_mut(&mut self) -> &mut Vec<T> {
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
