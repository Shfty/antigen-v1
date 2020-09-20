use std::fmt::Debug;

use crate::entity_component_system::{ComponentDebugTrait, ComponentTrait};

#[derive(Debug, Clone)]
pub struct EventQueueComponent<T>
where
    T: Debug,
{
    queue: Vec<T>,
}

impl<T> EventQueueComponent<T>
where
    T: Debug,
{
    pub fn new() -> Self {
        EventQueueComponent { queue: Vec::new() }
    }

    pub fn get_events(&self) -> &Vec<T> {
        &self.queue
    }

    pub fn clear_events(&mut self) {
        self.queue.clear();
    }

    pub fn push_event(&mut self, event: T) {
        self.queue.push(event);
    }
}

impl<T> Default for EventQueueComponent<T>
where
    T: Debug,
{
    fn default() -> Self {
        EventQueueComponent::new()
    }
}

impl<T> ComponentTrait for EventQueueComponent<T> where T: Debug + 'static {}

impl<T> ComponentDebugTrait for EventQueueComponent<T>
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
