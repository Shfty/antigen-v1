use crate::entity_component_system::EntityComponentSystem;

use super::SystemTrait;

type SystemClosures = Vec<Box<dyn FnOnce(&mut EntityComponentSystem)>>;

pub trait MapSystemAssembler: FnOnce(SystemAssembler) -> SystemAssembler {}

impl<T> MapSystemAssembler for T where T: FnOnce(SystemAssembler) -> SystemAssembler {}

#[derive(Default)]
pub struct SystemAssembler {
    closures: SystemClosures,
}

impl SystemAssembler {
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

    pub fn assemble<F>(self, f: F) -> SystemAssembler
    where
        F: MapSystemAssembler,
    {
        f(self)
    }

    pub fn finish(mut self, ecs: &mut EntityComponentSystem) {
        for closure in self.closures.drain(..) {
            closure(ecs);
        }
    }
}
