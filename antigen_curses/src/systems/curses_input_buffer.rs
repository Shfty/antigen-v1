use std::cell::{Ref, RefMut};

use antigen::{
    components::EventQueue,
    components::Window,
    entity_component_system::{
        system_interface::SystemInterface, ComponentData, EntityComponentDirectory, EntityID,
        SystemError, SystemTrait,
    },
};
use store::StoreQuery;

use crate::components::{CursesEvent, CursesWindowData};

/// Reads input from a pancurses window and pushes it into an event queue
#[derive(Debug)]
pub struct CursesInputBuffer;

impl<CD> SystemTrait<CD> for CursesInputBuffer
where
    CD: EntityComponentDirectory,
{
    fn run(&mut self, db: &mut SystemInterface<CD>) -> Result<(), SystemError>
    where
        CD: EntityComponentDirectory,
    {
        let (_, (mut event_queue,)) = StoreQuery::<
            EntityID,
            (RefMut<ComponentData<EventQueue<CursesEvent>>>,),
        >::iter(db.component_store)
        .next()
        .expect("No curses event queue");

        let (_, (_window, curses_window)) = StoreQuery::<
            EntityID,
            (
                Ref<ComponentData<Window>>,
                Ref<ComponentData<CursesWindowData>>,
            ),
        >::iter(db.component_store)
        .next()
        .expect("No curses window");

        let window: Option<&pancurses::Window> = curses_window.as_ref().as_ref();
        let input: Option<Option<pancurses::Input>> = window.map(|window| window.getch());

        if let Some(Some(input)) = input {
            event_queue.push(input);
        }

        Ok(())
    }
}
