use crate::components::InputAxisComponent;
use antigen::{
    components::EventQueueComponent,
    components::IntRangeComponent,
    core::events::AntigenEvent,
    entity_component_system::system_interface::SystemInterface,
    entity_component_system::ComponentStorage,
    entity_component_system::EntityComponentDirectory,
    entity_component_system::SystemDebugTrait,
    entity_component_system::{SystemError, SystemTrait},
};

#[derive(Debug)]
pub struct InputAxisSystem;

impl InputAxisSystem {
    pub fn new() -> Self {
        InputAxisSystem
    }
}

impl<CS, CD> SystemTrait<CS, CD> for InputAxisSystem
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
{
    fn run(&mut self, db: &mut SystemInterface<CS, CD>) -> Result<(), SystemError>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
    {
        let event_queue_entity =
            db.entity_component_directory
                .get_entity_by_predicate(|entity_id| {
                    db.entity_component_directory
                        .entity_has_component::<EventQueueComponent<AntigenEvent>>(entity_id)
                });

        if let Some(event_queue_entity) = event_queue_entity {
            let entities = db
                .entity_component_directory
                .get_entities_by_predicate(|entity_id| {
                    db.entity_component_directory
                        .entity_has_component::<InputAxisComponent>(entity_id)
                        && db
                            .entity_component_directory
                            .entity_has_component::<IntRangeComponent>(entity_id)
                });

            for entity_id in entities {
                let input_axis_component =
                    db.get_entity_component::<InputAxisComponent>(entity_id)?;
                let (prev_input, next_input) = (
                    input_axis_component.get_negative_input(),
                    input_axis_component.get_positive_input(),
                );

                let mut offset: i64 = 0;

                for event in db
                    .get_entity_component::<EventQueueComponent<AntigenEvent>>(event_queue_entity)?
                    .get_events()
                {
                    if let AntigenEvent::KeyPress { key_code } = event {
                        if *key_code == prev_input {
                            offset -= 1;
                        } else if *key_code == next_input {
                            offset += 1;
                        }
                    }
                }

                let ui_tab_input_component =
                    db.get_entity_component_mut::<IntRangeComponent>(entity_id)?;

                ui_tab_input_component.set_index(ui_tab_input_component.get_index() + offset);
            }
        }

        Ok(())
    }
}

impl SystemDebugTrait for InputAxisSystem {
    fn get_name() -> &'static str {
        "Input Axis"
    }
}
