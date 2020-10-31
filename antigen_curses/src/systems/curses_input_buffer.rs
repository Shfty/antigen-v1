use std::cell::{Ref, RefMut};

use antigen::{
    components::EventQueue,
    components::Window,
    entity_component_system::{
        ComponentStore, EntityID, SystemError,
        SystemTrait,
    },
};
use store::StoreQuery;

use crate::components::{CursesEvent, CursesWindowData};

/// Reads input from a pancurses window and pushes it into an event queue
#[derive(Debug)]
pub struct CursesInputBuffer;

impl SystemTrait for CursesInputBuffer {
    fn run(&mut self, db: &mut ComponentStore) -> Result<(), SystemError> {
        let (_, mut event_queue) =
            StoreQuery::<(EntityID, RefMut<EventQueue<CursesEvent>>)>::iter(db.as_ref())
                .next()
                .expect("No curses event queue");

        let (_, _window, curses_window) =
            StoreQuery::<(EntityID, Ref<Window>, Ref<CursesWindowData>)>::iter(db.as_ref())
                .next()
                .expect("No curses window");

        let window: Option<&pancurses::Window> = curses_window.as_ref();
        let input: Option<Option<pancurses::Input>> = window.map(|window| window.getch());

        if let Some(Some(input)) = input {
            event_queue.push(input);
        }

        Ok(())
    }
}
