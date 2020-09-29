use std::fmt::Debug;

use crate::{
    components::EventQueue,
    entity_component_system::{
        ComponentStorage, EntityComponentDirectory, EntityID, SystemDebugTrait, SystemError,
        SystemTrait,
    },
};
use crate::{components::EventTargets, entity_component_system::system_interface::SystemInterface};

#[derive(Debug)]
pub struct EventProcessorSystem<O, I>
where
    O: Debug,
    I: Debug,
{
    convert: fn(O) -> Option<I>,
}

impl<O, I> EventProcessorSystem<O, I>
where
    O: Debug,
    I: Debug,
{
    pub fn new(convert: fn(O) -> Option<I>) -> Self {
        EventProcessorSystem { convert }
    }
}

impl<CS, CD, O, I> SystemTrait<CS, CD> for EventProcessorSystem<O, I>
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
    O: Debug + Copy + 'static,
    I: Debug + Copy + 'static,
{
    fn run(&mut self, db: &mut SystemInterface<CS, CD>) -> Result<(), SystemError>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
    {
        let output_entities =
            db.entity_component_directory
                .get_entities_by_predicate(|entity_id| {
                    db.entity_component_directory
                        .entity_has_component::<EventQueue<O>>(entity_id)
                        && db
                            .entity_component_directory
                            .entity_has_component::<EventTargets>(entity_id)
                });

        for output_entity in output_entities {
            let events: Vec<O>;
            {
                let event_queue: &mut Vec<O> = db
                    .get_entity_component_mut::<EventQueue<O>>(output_entity)?
                    .as_mut();

                events = event_queue.clone();
            }

            let events: Vec<I> = events.into_iter().flat_map(self.convert).collect();

            let event_targets: &Vec<EntityID> = db
                .get_entity_component::<EventTargets>(output_entity)?
                .as_ref();

            let event_targets: Vec<EntityID> = event_targets
                .iter()
                .filter(|entity_id| {
                    db.entity_component_directory
                        .entity_has_component::<EventQueue<I>>(entity_id)
                })
                .copied()
                .collect();

            for event_target in event_targets {
                let event_queue = db.get_entity_component_mut::<EventQueue<I>>(event_target)?;
                let event_queue: &mut Vec<I> = event_queue.as_mut();
                event_queue.append(&mut events.clone());
            }
        }

        Ok(())
    }
}

impl<O, I> SystemDebugTrait for EventProcessorSystem<O, I>
where
    O: Debug,
    I: Debug,
{
    fn get_name() -> &'static str {
        "Event Processor"
    }
}
