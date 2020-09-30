use antigen::{
    components::EventQueue,
    components::Window,
    entity_component_system::{
        system_interface::SystemInterface, ComponentStorage, EntityComponentDirectory, SystemError,
        SystemTrait,
    },
};

use crate::components::{CursesEvent, CursesWindowData};

/// Reads input from a pancurses window and pushes it into an event queue
#[derive(Debug)]
pub struct CursesInputBuffer;

impl<CS, CD> SystemTrait<CS, CD> for CursesInputBuffer
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
{
    fn run(&mut self, db: &mut SystemInterface<CS, CD>) -> Result<(), SystemError>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
    {
        // Fetch event queue entity
        let event_queue_entity =
            db.entity_component_directory
                .get_entity_by_predicate(|entity_id| {
                    db.entity_component_directory
                        .entity_has_component::<EventQueue<CursesEvent>>(entity_id)
                });

        if let Some(event_queue_entity) = event_queue_entity {
            // If event queue exists, fetch the window entry we'll be reading input from
            let window_entity =
                db.entity_component_directory
                    .get_entity_by_predicate(|entity_id| {
                        db.entity_component_directory
                            .entity_has_component::<Window>(entity_id)
                            && db
                                .entity_component_directory
                                .entity_has_component::<CursesWindowData>(entity_id)
                    });

            if let Some(entity_id) = window_entity {
                let window: &Option<pancurses::Window> = db.get_entity_component::<CursesWindowData>(entity_id)?;
                if let Some(window) = window {
                    if let Some(input) = window.getch() {
                        // Fetch the entity queue component and push inputs into it
                        let event_queue: &mut Vec<CursesEvent> = db
                            .get_entity_component_mut::<EventQueue<CursesEvent>>(
                                event_queue_entity,
                            )?
                            .as_mut();

                        event_queue.push(input);
                    }
                }
            }
        }

        Ok(())
    }
}
