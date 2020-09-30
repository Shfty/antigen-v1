use std::{fmt::Debug, ops::Deref, ops::DerefMut};

#[derive(Debug, Clone)]
pub struct EventQueue<T>(Vec<T>)
where
    T: Debug;

impl<T> Deref for EventQueue<T>
where
    T: Debug,
{
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for EventQueue<T>
where
    T: Debug,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
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
