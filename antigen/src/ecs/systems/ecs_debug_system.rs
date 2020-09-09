use crate::components::{IntRangeComponent, StringComponent};
use crate::ecs::{
    components::{DebugData, ECSDebugComponent},
    EntityComponentSystemDebug, SystemEvent, {EntityComponentSystem, SystemTrait},
};

#[derive(Debug)]
pub struct ECSDebugSystem;

impl Default for ECSDebugSystem {
    fn default() -> Self {
        ECSDebugSystem
    }
}

impl ECSDebugSystem {
    pub fn new() -> Self {
        ECSDebugSystem::default()
    }
}

impl<T> SystemTrait<T> for ECSDebugSystem where T: EntityComponentSystem + EntityComponentSystemDebug {
    fn run(&mut self, ecs: &mut T) -> Result<SystemEvent, String> where T: EntityComponentSystem + EntityComponentSystemDebug {
        let entities = ecs.get_entities_by_predicate(|entity_id| {
            ecs.entity_has_component::<ECSDebugComponent>(entity_id)
                && ecs.entity_has_component::<StringComponent>(entity_id)
                && ecs.entity_has_component::<IntRangeComponent>(entity_id)
        });

        for entity_id in entities {
            let ecs_debug_component = ecs.get_entity_component::<ECSDebugComponent>(entity_id)?;
            let debug_data = ecs_debug_component.debug_data;

            let ecs_string = match debug_data {
                DebugData::Entities => ecs
                    .get_entities()
                    .iter()
                    .copied()
                    .map(|entity_id| ecs.get_entity_label(*entity_id))
                    .fold("Entities:\n".to_string(), |acc, next| acc + next + "\n"),
                DebugData::Components => ecs
                    .get_components()
                    .iter()
                    .map(|(_, component_interface)| &component_interface.official_name)
                    .fold("Components:\n".to_string(), |acc, next| acc + next + "\n"),
                DebugData::ComponentData => {
                    format!("Component Data: {:#?}", ecs.get_component_data())
                }
                DebugData::EntityComponents => {
                    format!("Entity Components: {:#?}", ecs.get_entity_components())
                }
            };

            let string_component = ecs.get_entity_component::<StringComponent>(entity_id)?;

            string_component.data = ecs_string.clone();
        }

        Ok(SystemEvent::None)
    }
}
