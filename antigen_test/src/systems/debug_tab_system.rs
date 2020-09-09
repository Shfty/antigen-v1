use antigen::{
    components::IntRangeComponent,
    ecs::{
        components::{DebugData, ECSDebugComponent},
        SystemTrait, EntityComponentSystem, SystemEvent,
    },
ecs::EntityComponentSystemDebug};

#[derive(Debug)]
pub struct DebugTabSystem;

impl DebugTabSystem {
    pub fn new() -> Self {
        DebugTabSystem
    }
}

impl<T> SystemTrait<T> for DebugTabSystem where T: EntityComponentSystem + EntityComponentSystemDebug {
    fn run(&mut self, ecs: &mut T) -> Result<SystemEvent, String> {
        let entities = ecs.get_entities_by_predicate(|entity_id| {
            ecs.entity_has_component::<IntRangeComponent>(entity_id)
                && ecs.entity_has_component::<ECSDebugComponent>(entity_id)
        });

        for entity_id in entities {
            let ui_tab_component = ecs.get_entity_component::<IntRangeComponent>(entity_id)?;
            let index = ui_tab_component.index;

            let ecs_debug_component = ecs.get_entity_component::<ECSDebugComponent>(entity_id)?;
            ecs_debug_component.debug_data = match index {
                0 => DebugData::Entities,
                1 => DebugData::Components,
                2 => DebugData::ComponentData,
                3 => DebugData::EntityComponents,
                _ => DebugData::Entities,
            };
        }

        Ok(SystemEvent::None)
    }
}
