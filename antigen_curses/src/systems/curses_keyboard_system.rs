use antigen::{
    components::EventQueueComponent,
    entity_component_system::{
        system_interface::SystemInterface, ComponentStorage, EntityComponentDirectory,
        SystemDebugTrait, SystemError, SystemTrait,
    },
    events::AntigenEvent,
    keyboard::IntoKey,
};

use crate::{CursesEventQueueComponent, CursesInput};

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
                        .entity_has_component::<CursesEventQueueComponent>(entity_id)
                });

        if let Some(pancurses_event_queue_entity) = pancurses_event_queue_entity {
            let mut antigen_keys: Vec<antigen::keyboard::Key> = Vec::new();
            for event in db
                .get_entity_component::<CursesEventQueueComponent>(
                    pancurses_event_queue_entity,
                )?
                .get_events()
            {
                let event = *event;

                if let pancurses::Input::KeyResize = event {
                    pancurses::resize_term(0, 0);
                } else {
                    let pancurses_input: CursesInput = event.into();
                    let antigen_key = pancurses_input.into_key();
                    if antigen_key != antigen::keyboard::Key::Unknown {
                        antigen_keys.push(antigen_key);
                    }
                }
            }

            let antigen_event_queue_entity =
                db.entity_component_directory
                    .get_entity_by_predicate(|entity_id| {
                        db.entity_component_directory
                            .entity_has_component::<EventQueueComponent<AntigenEvent>>(entity_id)
                    });

            if let Some(event_queue_entity) = antigen_event_queue_entity {
                let antigen_event_queue_component = db
                    .get_entity_component_mut::<EventQueueComponent<AntigenEvent>>(
                        event_queue_entity,
                    )?;

                for antigen_input in antigen_keys {
                    antigen_event_queue_component.push_event(AntigenEvent::KeyPress {
                        key_code: antigen_input,
                    });
                    antigen_event_queue_component.push_event(AntigenEvent::KeyRelease {
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
        "Pancurses Keyboard"
    }
}
