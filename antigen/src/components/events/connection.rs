use std::{fmt::Debug, ops::Deref, ops::DerefMut};

use crate::entity_component_system::{ComponentStore, EntityID};

use super::EventQueue;

/// Holds a list of entities to be used as targets for emitted events
pub struct Connection {
    targets: Vec<EntityID>,
    convert: Box<dyn Fn(&ComponentStore, EntityID)>,
}

impl Debug for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Connection")
            .field("targets", &self.targets)
            .finish()
    }
}

impl Connection {
    pub fn new<O, I>(targets: Vec<EntityID>, convert: fn(O) -> Option<I>) -> Self
    where
        O: Clone + Debug + 'static,
        I: Clone + Debug + 'static,
    {
        let closure_targets = targets.clone();
        let convert = move |db: &ComponentStore, entity_id: EntityID| {
            if let Some(output_queue) = db.get::<EventQueue<O>>(&entity_id) {
                for event in output_queue.iter() {
                    if let Some(event) = convert(event.clone()) {
                        for target in &closure_targets {
                            if let Some(mut input_queue) = db.get_mut::<EventQueue<I>>(target) {
                                input_queue.push(event.clone());
                            }
                        }
                    }
                }
            }
        };

        let convert: Box<dyn Fn(&ComponentStore, EntityID)> = Box::new(convert);

        Connection { targets, convert }
    }

    pub fn run(&self, db: &ComponentStore, entity_id: EntityID) {
        let convert = &self.convert;
        convert(db, entity_id);
    }
}

impl Deref for Connection {
    type Target = Vec<EntityID>;

    fn deref(&self) -> &Self::Target {
        &self.targets
    }
}

impl DerefMut for Connection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.targets
    }
}
