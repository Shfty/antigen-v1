use crate::{
    entity_component_system::entity_component_database::ComponentStorage,
    entity_component_system::entity_component_database::EntityComponentDirectory,
    entity_component_system::SystemTrait,
};

use super::SystemStorage;

pub struct HeapSystemStorage<S, D>
where
    S: ComponentStorage,
    D: EntityComponentDirectory,
{
    systems: Vec<(String, Box<dyn SystemTrait<S, D>>)>,
}

impl<S, D> HeapSystemStorage<S, D>
where
    S: ComponentStorage,
    D: EntityComponentDirectory,
{
    pub fn new() -> HeapSystemStorage<S, D> {
        HeapSystemStorage {
            systems: Vec::new(),
        }
    }
}

impl<S, D> Default for HeapSystemStorage<S, D>
where
    S: ComponentStorage,
    D: EntityComponentDirectory,
{
    fn default() -> Self {
        HeapSystemStorage::new()
    }
}

impl<CS, CD> SystemStorage<CS, CD> for HeapSystemStorage<CS, CD>
where
    CS: ComponentStorage + 'static,
    CD: EntityComponentDirectory + 'static,
{
    fn insert_system<T>(&mut self, name: &str, system: T)
    where
        T: SystemTrait<CS, CD> + 'static,
    {
        self.systems.push((name.into(), Box::new(system)));
    }

    fn get_system_names(&self) -> Vec<String> {
        self.systems.iter().map(|(name, _)| name).cloned().collect()
    }

    fn get_system(&mut self, name: &str) -> Result<&mut dyn SystemTrait<CS, CD>, String> {
        let (_, system) = self
            .systems
            .iter_mut()
            .find(|(candidate_name, _)| candidate_name == name)
            .ok_or(format!("Error getting system {}: No such system", name))?;

        Ok(system.as_mut())
    }
}
