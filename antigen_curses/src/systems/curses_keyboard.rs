use std::cell::{Ref, RefMut};

use antigen::{
    components::EventQueue,
    core::events::AntigenInputEvent,
    core::keyboard::IntoKey,
    entity_component_system::{
        system_interface::SystemInterface, EntityComponentDirectory, EntityID, SystemError,
        SystemTrait,
    },
};
use store::StoreQuery;

use crate::{components::CursesEvent, CursesInput};

/// Converts pancurses keyboard inputs into antigen keyboard inputs
#[derive(Debug)]
pub struct CursesKeyboard;

impl<CD> SystemTrait<CD> for CursesKeyboard
where
    CD: EntityComponentDirectory,
{
    fn run(&mut self, db: &mut SystemInterface<CD>) -> Result<(), SystemError>
    where
        CD: EntityComponentDirectory,
    {
        let (_, curses_event_queue) =
            StoreQuery::<(EntityID, Ref<EventQueue<CursesEvent>>)>::iter(db.component_store)
                .next()
                .expect("No curses event queue entity");

        let (_, mut antigen_event_queue) = StoreQuery::<(
            EntityID,
            RefMut<EventQueue<AntigenInputEvent>>,
        )>::iter(db.component_store)
        .next()
        .expect("No antigen event queue entity");

        let mut antigen_keys: Vec<antigen::core::keyboard::Key> = Vec::new();

        for event in curses_event_queue.iter() {
            if let CursesEvent::KeyResize = event {
                pancurses::resize_term(0, 0);
            } else {
                let pancurses_input: CursesInput = (*event).into();
                let antigen_key = pancurses_input.into_key();
                if antigen_key != antigen::core::keyboard::Key::Unknown {
                    antigen_keys.push(antigen_key);
                }
            }
        }

        for antigen_input in antigen_keys {
            antigen_event_queue.push(AntigenInputEvent::KeyPress {
                key_code: antigen_input,
            });
            antigen_event_queue.push(AntigenInputEvent::KeyRelease {
                key_code: antigen_input,
            });
        }

        Ok(())
    }
}
