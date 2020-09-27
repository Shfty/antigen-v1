use antigen::{
    components::{Size, Window},
    entity_component_system::{
        system_interface::SystemInterface, ComponentStorage, ComponentTrait,
        EntityComponentDirectory, EntityID, SystemDebugTrait, SystemError, SystemTrait,
    },
    primitive_types::Vector2I,
};

use crate::{CursesEvent, CursesEventQueue, CursesWindow};

// TODO: Properly delete windows when their component is removed

#[derive(Debug)]
pub struct CursesWindowSystem;

impl CursesWindowSystem {
    pub fn new<CS>(component_storage: &mut CS) -> Self
    where
        CS: ComponentStorage,
    {
        fn drop_callback(_: &mut dyn ComponentTrait) {
            pancurses::endwin();
        }

        component_storage.register_component_drop_callback::<CursesWindow>(drop_callback);

        CursesWindowSystem
    }

    fn try_create_window<CS, CD>(
        &mut self,
        db: &mut SystemInterface<CS, CD>,
        entity_id: EntityID,
    ) -> Result<(), String>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
    {
        let curses_window = db.get_entity_component::<CursesWindow>(entity_id)?;
        let curses_window: &Option<pancurses::Window> = curses_window.as_ref();
        if curses_window.is_some() {
            return Ok(());
        }

        let Vector2I(width, height) = (*db.get_entity_component::<Size>(entity_id)?).into();

        let title = match db.get_entity_component::<String>(entity_id) {
            Ok(string_component) => string_component,
            Err(_) => "Antigen",
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

        let curses_window: &mut Option<pancurses::Window> = db
            .get_entity_component_mut::<CursesWindow>(entity_id)?
            .as_mut();

        *curses_window = Some(window);

        Ok(())
    }
}

impl<CS, CD> SystemTrait<CS, CD> for CursesWindowSystem
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
            let pancurses_event_queue: &Vec<CursesEvent> = db
                .get_entity_component::<CursesEventQueue>(pancurses_event_queue_entity)?
                .as_ref();

            for event in pancurses_event_queue {
                if let CursesEvent::KeyResize = event {
                    pancurses::resize_term(0, 0);
                }
            }
        }

        // Get window entities, update internal window state
        let window_entity = db
            .entity_component_directory
            .get_entity_by_predicate(|entity_id| {
                db.entity_component_directory
                    .entity_has_component::<Window>(entity_id)
                    && db
                        .entity_component_directory
                        .entity_has_component::<CursesWindow>(entity_id)
                    && db
                        .entity_component_directory
                        .entity_has_component::<Size>(entity_id)
            });

        if let Some(window_entity) = window_entity {
            // Make sure the window exists
            self.try_create_window(db, window_entity)?;

            // Process any pending resize inputs
            let pancurses_event_queue_entity = db
                .entity_component_directory
                .get_entity_by_predicate(|entity_id| {
                    db.entity_component_directory
                        .entity_has_component::<CursesEventQueue>(entity_id)
                });

            if let Some(pancurses_event_queue_entity) = pancurses_event_queue_entity {
                let pancurses_event_queue: &Vec<CursesEvent> = db
                    .get_entity_component::<CursesEventQueue>(pancurses_event_queue_entity)?
                    .as_ref();

                if pancurses_event_queue
                    .iter()
                    .any(|input| *input == CursesEvent::KeyResize)
                {
                    pancurses::resize_term(0, 0);
                }
            }

            // Update window component size
            let curses_window: &Option<pancurses::Window> = db
                .get_entity_component::<CursesWindow>(window_entity)?
                .as_ref();

            if let Some(window) = curses_window {
                let (window_height, window_width) = window.get_max_yx();

                *db.get_entity_component_mut::<Size>(window_entity)? =
                    Vector2I(window_width as i64, window_height as i64).into();
            }
        }

        Ok(())
    }
}

impl SystemDebugTrait for CursesWindowSystem {
    fn get_name() -> &'static str {
        "Curses Window"
    }
}
