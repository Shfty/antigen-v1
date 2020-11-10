use std::cell::{Ref, RefMut};

use antigen::{
    components::EventQueue,
    core::events::{MouseMove, MousePress, MouseRelease, MouseScroll},
    entity_component_system::{ComponentStore, EntityID, SystemError, SystemTrait},
    primitive_types::Vector2I,
};
use store::StoreQuery;

use crate::CursesEvent;

const WHEEL_UP: usize = 65536;
const WHEEL_DOWN: usize = 2097152;

type ReadCursesEventQueue<'a> = (EntityID, Ref<'a, EventQueue<CursesEvent>>);

/// Converts pancurses mouse inputs into antigen mouse inputs
#[derive(Debug)]
pub struct CursesMouse {
    position: Vector2I,
    button_mask: usize,
}

impl Default for CursesMouse {
    fn default() -> Self {
        CursesMouse {
            position: Vector2I::default(),
            button_mask: 0,
        }
    }
}

impl SystemTrait for CursesMouse {
    fn run(&mut self, db: &mut ComponentStore) -> Result<(), SystemError> {
        let (_, curses_event_queue) = StoreQuery::<ReadCursesEventQueue>::iter(db.as_ref())
            .next()
            .expect("No curses event queue entity");

        for curses_event in curses_event_queue.iter() {
            let CursesEvent(curses_event) = curses_event;
            if *curses_event == pancurses::Input::KeyMouse {
                // Check for mouse input
                let mouse_event =
                    pancurses::getmouse().expect("Failed to get pancurses mouse event");

                let event_x = mouse_event.x as i64;
                let event_y = mouse_event.y as i64;
                let event_button_mask = mouse_event.bstate as usize;

                let mut delta = Vector2I(0, 0);
                let prev_position = self.position;
                let prev_button_mask: usize = self.button_mask;
                {
                    if event_x > -1 && event_x != self.position.0 {
                        delta.0 = event_x - self.position.0;
                        self.position.0 = event_x as i64;
                    }

                    if event_y > -1 && event_y != self.position.1 {
                        delta.1 = event_y - self.position.1;
                        self.position.1 = event_y as i64;
                    }

                    if self.button_mask != event_button_mask {
                        self.button_mask = event_button_mask
                    }
                }

                if delta != Vector2I(0, 0) {
                    for (_, mut mouse_move_queue) in
                        StoreQuery::<(EntityID, RefMut<EventQueue<MouseMove>>)>::iter(db.as_ref())
                    {
                        mouse_move_queue.push(MouseMove {
                            position: prev_position + delta,
                            delta,
                        })
                    }
                }

                if WHEEL_UP & self.button_mask > 0 {
                    for (_, mut mouse_scroll_queue) in
                        StoreQuery::<(EntityID, RefMut<EventQueue<MouseScroll>>)>::iter(db.as_ref())
                    {
                        mouse_scroll_queue.push(MouseScroll { delta: -1 })
                    }
                }

                if WHEEL_DOWN & self.button_mask > 0 {
                    for (_, mut mouse_scroll_queue) in
                        StoreQuery::<(EntityID, RefMut<EventQueue<MouseScroll>>)>::iter(db.as_ref())
                    {
                        mouse_scroll_queue.push(MouseScroll { delta: 1 })
                    }
                }

                if self.button_mask != prev_button_mask {
                    let mut pressed_mask = 0usize;
                    {
                        if (pancurses::BUTTON1_PRESSED as usize & self.button_mask) > 0 {
                            pressed_mask |= 1;
                        }
                        if (pancurses::BUTTON2_PRESSED as usize & self.button_mask) > 0 {
                            pressed_mask |= 4;
                        }
                        if (pancurses::BUTTON3_PRESSED as usize & self.button_mask) > 0 {
                            pressed_mask |= 2;
                        }
                    }

                    let mut released_mask = 0usize;
                    {
                        if (pancurses::BUTTON1_RELEASED as usize & self.button_mask) > 0 {
                            released_mask |= 1;
                        }
                        if (pancurses::BUTTON2_RELEASED as usize & self.button_mask) > 0 {
                            released_mask |= 4;
                        }
                        if (pancurses::BUTTON3_RELEASED as usize & self.button_mask) > 0 {
                            released_mask |= 2;
                        }
                    }

                    if pressed_mask != 0 {
                        for (_, mut mouse_press_queue) in
                            StoreQuery::<(EntityID, RefMut<EventQueue<MousePress>>)>::iter(
                                db.as_ref(),
                            )
                        {
                            mouse_press_queue.push(MousePress {
                                button_mask: pressed_mask,
                            })
                        }
                    }

                    if released_mask != 0 {
                        for (_, mut mouse_release_queue) in
                            StoreQuery::<(EntityID, RefMut<EventQueue<MouseRelease>>)>::iter(
                                db.as_ref(),
                            )
                        {
                            mouse_release_queue.push(MouseRelease {
                                button_mask: released_mask,
                            })
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
