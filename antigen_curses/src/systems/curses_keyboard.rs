use std::cell::{Ref, RefMut};

use antigen::{
    components::EventQueue,
    core::{events::KeyPress, events::KeyRelease},
    entity_component_system::{ComponentStore, EntityID, SystemError, SystemTrait},
};
use store::StoreQuery;

use crate::CursesEvent;

type ReadCursesEventQueue<'a> = (EntityID, Ref<'a, EventQueue<CursesEvent>>);

/// Converts pancurses keyboard inputs into antigen keyboard inputs
#[derive(Debug)]
pub struct CursesKeyboard;

impl SystemTrait for CursesKeyboard {
    fn run(&mut self, db: &mut ComponentStore) -> Result<(), SystemError> {
        let (_, curses_event_queue) = StoreQuery::<ReadCursesEventQueue>::iter(db.as_ref())
            .next()
            .expect("No curses event queue entity");

        let (_, mut mouse_press_queue) =
            StoreQuery::<(EntityID, RefMut<EventQueue<KeyPress>>)>::iter(db.as_ref())
                .next()
                .expect("No antigen event queue entity");

        let (_, mut mouse_release_queue) =
            StoreQuery::<(EntityID, RefMut<EventQueue<KeyRelease>>)>::iter(db.as_ref())
                .next()
                .expect("No antigen event queue entity");

        let antigen_keys = curses_event_queue.iter().flat_map(|event| {
            let CursesEvent(event) = event;
            if pancurses::Input::KeyResize == *event {
                pancurses::resize_term(0, 0);
                None
            } else {
                let pancurses_input: CursesEvent = (*event).into();
                Some(pancurses_input.into())
            }
        });

        for antigen_input in antigen_keys {
            mouse_press_queue.push(KeyPress {
                key_code: antigen_input,
            });
            mouse_release_queue.push(KeyRelease {
                key_code: antigen_input,
            });
        }

        Ok(())
    }
}
