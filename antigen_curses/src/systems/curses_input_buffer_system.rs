use std::borrow::{Borrow, BorrowMut};

use antigen::{
    components::Window,
    entity_component_system::{
        system_interface::SystemInterface, ComponentStorage, EntityComponentDirectory,
        SystemDebugTrait, SystemError, SystemTrait,
    },
};

use crate::{CursesEvent, CursesEventQueue, CursesWindow};

/// Reads input from a pancurses window and pushes it into an event queue
#[derive(Debug)]
pub struct CursesInputBufferSystem;

impl<CS, CD> SystemTrait<CS, CD> for CursesInputBufferSystem
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
                        .entity_has_component::<CursesEventQueue>(entity_id)
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
                                .entity_has_component::<CursesWindow>(entity_id)
                    });

            if let Some(entity_id) = window_entity {
                if let Some(window) = db.get_entity_component::<CursesWindow>(entity_id)?.borrow() {
                    if let Some(input) = window.getch() {
                        // Fetch the entity queue component and push inputs into it
                        let event_queue: &mut Vec<CursesEvent> = db
                            .get_entity_component_mut::<CursesEventQueue>(event_queue_entity)?
                            .borrow_mut();

                        event_queue.push(input);
                    }
                }
            }
        }

        Ok(())
    }
}

impl SystemDebugTrait for CursesInputBufferSystem {
    fn get_name() -> &'static str {
        "Curses Input Buffer"
    }
}
