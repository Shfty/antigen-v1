use std::fmt::Debug;

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
