use antigen::{
    components::EventQueue,
    core::events::AntigenInputEvent,
    core::keyboard::IntoKey,
    entity_component_system::{
        system_interface::SystemInterface, ComponentStorage, EntityComponentDirectory,
        SystemDebugTrait, SystemError, SystemTrait,
    },
};

use crate::{CursesEvent, CursesEventQueue, CursesInput};

/// Converts pancurses keyboard inputs into antigen keyboard inputs
#[derive(Debug)]
pub struct CursesKeyboardSystem;

impl<CS, CD> SystemTrait<CS, CD> for CursesKeyboardSystem
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
{
    fn run(&mut self, db: &mut SystemInterface<CS, CD>) -> Result<(), SystemError>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
    {
        let pancurses_event_queue_entity =
            db.entity_component_directory
                .get_entity_by_predicate(|entity_id| {
                    db.entity_component_directory
                        .entity_has_component::<CursesEventQueue>(entity_id)
                });

        if let Some(pancurses_event_queue_entity) = pancurses_event_queue_entity {
            let mut antigen_keys: Vec<antigen::core::keyboard::Key> = Vec::new();

            let event_queue: &Vec<CursesEvent> = db
                .get_entity_component::<CursesEventQueue>(pancurses_event_queue_entity)?
                .as_ref();

            for event in event_queue {
                let event = *event;

                if let CursesEvent::KeyResize = event {
                    pancurses::resize_term(0, 0);
                } else {
                    let pancurses_input: CursesInput = event.into();
                    let antigen_key = pancurses_input.into_key();
                    if antigen_key != antigen::core::keyboard::Key::Unknown {
                        antigen_keys.push(antigen_key);
                    }
                }
            }

            let antigen_event_queue_entity =
                db.entity_component_directory
                    .get_entity_by_predicate(|entity_id| {
                        db.entity_component_directory
                            .entity_has_component::<EventQueue<AntigenInputEvent>>(entity_id)
                    });

            if let Some(event_queue_entity) = antigen_event_queue_entity {
                let antigen_event_queue: &mut Vec<AntigenInputEvent> = db
                    .get_entity_component_mut::<EventQueue<AntigenInputEvent>>(event_queue_entity)?
                    .as_mut();

                for antigen_input in antigen_keys {
                    antigen_event_queue.push(AntigenInputEvent::KeyPress {
                        key_code: antigen_input,
                    });
                    antigen_event_queue.push(AntigenInputEvent::KeyRelease {
                        key_code: antigen_input,
                    });
                }
            }
        }

        Ok(())
    }
}

impl SystemDebugTrait for CursesKeyboardSystem {
    fn get_name() -> &'static str {
        "Curses Keyboard"
    }
}
