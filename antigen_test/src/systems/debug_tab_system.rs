use antigen::{
    components::{DebugData, ECSDebugComponent, IntRangeComponent},
    ecs::EntityComponentDatabaseDebug,
    ecs::{EntityComponentDatabase, SystemEvent, SystemTrait},
};

#[derive(Debug)]
pub struct DebugTabSystem;

impl DebugTabSystem {
    pub fn new() -> Self {
        DebugTabSystem
    }
}

impl<T> SystemTrait<T> for DebugTabSystem
where
    T: EntityComponentDatabase + EntityComponentDatabaseDebug,
{
    fn run(&mut self, db: &mut T) -> Result<SystemEvent, String> {
        let entities = db.get_entities_by_predicate(|entity_id| {
            db.entity_has_component::<IntRangeComponent>(entity_id)
                && db.entity_has_component::<ECSDebugComponent>(entity_id)
        });

        for entity_id in entities {
            let ui_tab_component = db.get_entity_component::<IntRangeComponent>(entity_id)?;
            let index = ui_tab_component.index;

            let ecs_debug_component = db.get_entity_component::<ECSDebugComponent>(entity_id)?;
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
