use std::cell::{Ref, RefMut};

use antigen::{
    components::{EventQueue, Size, Window},
    entity_component_system::{ComponentStore, EntityID, SystemError, SystemTrait},
    primitive_types::Vector2I,
};
use store::StoreQuery;

use crate::{components::CursesWindowData, CursesEvent};

type ReadCursesEventQueue<'a> = (EntityID, Ref<'a, EventQueue<CursesEvent>>);
type WriteCursesWindow<'a> = (
    EntityID,
    Ref<'a, Window>,
    Option<Ref<'a, String>>,
    RefMut<'a, Size>,
    RefMut<'a, CursesWindowData>,
);

#[derive(Debug)]
pub struct CursesWindow;

impl CursesWindow {
    fn try_create_window(
        &mut self,
        curses_window: &mut RefMut<CursesWindowData>,
        size: &RefMut<Size>,
        string: Option<Ref<String>>,
    ) -> Result<(), String> {
        if curses_window.is_some() {
            return Ok(());
        }

        let Vector2I(width, height) = ***size;

        let title = if let Some(string) = string {
            (*string).clone()
        } else {
            "Antigen".into()
        };

        let window = pancurses::initscr();

        pancurses::resize_term(height as i32, width as i32);
        pancurses::set_title(&title);
        pancurses::mousemask(
            pancurses::ALL_MOUSE_EVENTS | pancurses::REPORT_MOUSE_POSITION,
            std::ptr::null_mut(),
        );
        pancurses::mouseinterval(0);
        pancurses::curs_set(0);
        pancurses::noecho();
        pancurses::start_color();

        window.keypad(true);
        window.timeout(0);

        ***curses_window = Some(window);

        Ok(())
    }
}

impl SystemTrait for CursesWindow {
    fn run(&mut self, db: &mut ComponentStore) -> Result<(), SystemError> {
        let (_, curses_event_queue) = StoreQuery::<ReadCursesEventQueue>::iter(db.as_ref())
            .next()
            .expect("No curses event queue entity");

        // Get window entity, update internal window state
        let (_, _window, string, mut size, mut curses_window) =
            StoreQuery::<WriteCursesWindow>::iter(db.as_ref())
                .next()
                .expect("No curses window entity");

        // Make sure the window exists
        self.try_create_window(&mut curses_window, &size, string)?;

        // Process any pending resize inputs
        let wants_resize = curses_event_queue.iter().any(|input| {
            let CursesEvent(input) = input;
            *input == pancurses::Input::KeyResize
        });

        if wants_resize {
            pancurses::resize_term(0, 0);
        }

        // Update window component size
        let curses_window = (**curses_window).as_ref();
        if let Some(curses_window) = curses_window {
            let (height, width) = curses_window.get_max_yx();
            **size = Vector2I(width as i64, height as i64);
        }

        Ok(())
    }
}
