use crate::entity_component_system::EntityComponentSystem;

use super::SystemTrait;

type SystemClosures = Vec<Box<dyn FnOnce(&mut EntityComponentSystem)>>;

pub trait MapSystemBuilder: FnOnce(SystemBuilder) -> SystemBuilder {}

impl<T> MapSystemBuilder for T where T: FnOnce(SystemBuilder) -> SystemBuilder {}

#[derive(Default)]
pub struct SystemBuilder {
    closures: SystemClosures,
}

impl SystemBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn system<S>(mut self, system: S) -> Self
    where
        S: SystemTrait + 'static,
    {
        self.closures
            .push(Box::new(move |ecs: &mut EntityComponentSystem| {
                ecs.push_system(system)
            }));
        self
    }

    pub fn map<F>(self, f: F) -> SystemBuilder
    where
        F: MapSystemBuilder,
    {
        f(self)
    }

    pub fn finish(mut self, ecs: &mut EntityComponentSystem) {
        for closure in self.closures.drain(..) {
            closure(ecs);
        }
    }
}
