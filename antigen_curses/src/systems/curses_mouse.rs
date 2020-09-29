use antigen::{
    components::EventQueue,
    core::events::AntigenInputEvent,
    entity_component_system::{
        system_interface::SystemInterface, ComponentStorage, EntityComponentDirectory,
        SystemError, SystemTrait,
    },
    primitive_types::Vector2I,
};

use crate::components::{CursesEvent, CursesEventQueue};

const WHEEL_UP: usize = 65536;
const WHEEL_DOWN: usize = 2097152;

/// Converts pancurses mouse inputs into antigen mouse inputs
#[derive(Debug)]
pub struct CursesMouse {
    position: Vector2I,
    button_mask: usize,
}

impl CursesMouse {
    pub fn new() -> Self {
        CursesMouse {
            position: Vector2I::default(),
            button_mask: 0,
        }
    }
}

impl Default for CursesMouse {
    fn default() -> Self {
        CursesMouse::new()
    }
}

impl<CS, CD> SystemTrait<CS, CD> for CursesMouse
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

            for pancurses_event in pancurses_event_queue.clone() {
                if pancurses_event == CursesEvent::KeyMouse {
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

                    let event_queue_entity =
                        db.entity_component_directory
                            .get_entity_by_predicate(|entity_id| {
                                db.entity_component_directory
                                    .entity_has_component::<EventQueue<AntigenInputEvent>>(
                                        entity_id,
                                    )
                            });

                    if let Some(event_queue_entity) = event_queue_entity {
                        let antigen_event_queue: &mut Vec<AntigenInputEvent> = db
                            .get_entity_component_mut::<EventQueue<AntigenInputEvent>>(
                                event_queue_entity,
                            )?
                            .as_mut();

                        if delta != Vector2I(0, 0) {
                            antigen_event_queue.push(AntigenInputEvent::MouseMove {
                                position: prev_position + delta,
                                delta,
                            })
                        }

                        if WHEEL_UP & self.button_mask > 0 {
                            antigen_event_queue.push(AntigenInputEvent::MouseScroll { delta: -1 })
                        }

                        if WHEEL_DOWN & self.button_mask > 0 {
                            antigen_event_queue.push(AntigenInputEvent::MouseScroll { delta: 1 })
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
                                if (pancurses::BUTTON4_PRESSED as usize & self.button_mask) > 0 {
                                    pressed_mask |= 8;
                                }
                                if (pancurses::BUTTON5_PRESSED as usize & self.button_mask) > 0 {
                                    pressed_mask |= 16;
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
                                if (pancurses::BUTTON4_RELEASED as usize & self.button_mask) > 0 {
                                    released_mask |= 8;
                                }
                                if (pancurses::BUTTON5_RELEASED as usize & self.button_mask) > 0 {
                                    released_mask |= 16;
                                }
                            }

                            if pressed_mask != 0 {
                                antigen_event_queue.push(AntigenInputEvent::MousePress {
                                    button_mask: pressed_mask,
                                })
                            }

                            if released_mask != 0 {
                                antigen_event_queue.push(AntigenInputEvent::MouseRelease {
                                    button_mask: released_mask,
                                })
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
