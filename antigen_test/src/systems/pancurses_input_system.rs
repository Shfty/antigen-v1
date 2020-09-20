use crate::{
    components::{
        pancurses_mouse_component::PancursesMouseComponent,
        pancurses_window_component::PancursesWindowComponent,
    },
    pancurses_keys::PancursesInput,
};
use antigen::{
    components::EventQueueComponent,
    components::WindowComponent,
    entity_component_system::system_interface::SystemInterface,
    entity_component_system::ComponentStorage,
    entity_component_system::EntityComponentDirectory,
    entity_component_system::SystemDebugTrait,
    entity_component_system::{SystemError, SystemTrait},
    events::AntigenEvent,
    primitive_types::IVector2,
};

#[derive(Debug)]
pub struct PancursesInputSystem {
    input_buffer_size: i64,
}

impl PancursesInputSystem {
    pub fn new(input_buffer_size: i64) -> Self {
        PancursesInputSystem { input_buffer_size }
    }
}

use antigen::IntoKey;

impl<CS, CD> SystemTrait<CS, CD> for PancursesInputSystem
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
{
    fn run(&mut self, db: &mut SystemInterface<CS, CD>) -> Result<(), SystemError>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
    {
        let window_entity = db
            .entity_component_directory
            .get_entity_by_predicate(|entity_id| {
                db.entity_component_directory
                    .entity_has_component::<WindowComponent>(entity_id)
                    && db
                        .entity_component_directory
                        .entity_has_component::<PancursesWindowComponent>(entity_id)
            });

        let mut antigen_keys: Vec<antigen::Key> = Vec::new();
        if let Some(entity_id) = window_entity {
            let window_component =
                db.get_entity_component::<PancursesWindowComponent>(entity_id)?;
            if let Some(window) = window_component.get_window() {
                let mut i = self.input_buffer_size;
                while let Some(input) = window.getch() {
                    if let pancurses::Input::KeyResize = input {
                        pancurses::resize_term(0, 0);
                    } else {
                        let pancurses_input: PancursesInput = input.into();
                        let antigen_key = pancurses_input.into_key();
                        if antigen_key != antigen::Key::Unknown {
                            antigen_keys.push(antigen_key);
                        }
                    }

                    i -= 1;
                    if i <= 0 {
                        break;
                    }
                }
            }

            pancurses::flushinp();
        }

        let event_queue_entity =
            db.entity_component_directory
                .get_entity_by_predicate(|entity_id| {
                    db.entity_component_directory
                        .entity_has_component::<EventQueueComponent<AntigenEvent>>(entity_id)
                });

        if let Some(event_queue_entity) = event_queue_entity {
            let event_queue_component = db
                .get_entity_component_mut::<EventQueueComponent<AntigenEvent>>(
                    event_queue_entity,
                )?;

            for antigen_input in antigen_keys {
                event_queue_component.push_event(AntigenEvent::KeyPress {
                    key_code: antigen_input,
                });
                event_queue_component.push_event(AntigenEvent::KeyRelease {
                    key_code: antigen_input,
                });
            }
        }

        let pancurses_mouse_entity =
            db.entity_component_directory
                .get_entity_by_predicate(|entity_id| {
                    db.entity_component_directory
                        .entity_has_component::<PancursesMouseComponent>(entity_id)
                });

        if let Some(pancurses_mouse_entity) = pancurses_mouse_entity {
            let pancurses_mouse_component =
                db.get_entity_component_mut::<PancursesMouseComponent>(pancurses_mouse_entity)?;

            // Check for mouse input
            if let Ok(mouse_event) = pancurses::getmouse() {
                let event_x = mouse_event.x as i64;
                let event_y = mouse_event.y as i64;
                let event_button_mask = mouse_event.bstate as usize;

                let prev_position;
                let mut delta = IVector2(0, 0);
                let mut new_button_mask: Option<usize> = None;
                {
                    prev_position = pancurses_mouse_component.get_position();

                    if event_x > -1 && event_x != prev_position.0 {
                        delta.0 = event_x - prev_position.0;
                        pancurses_mouse_component.set_mouse_x(event_x as i64);
                    }

                    if event_y > -1 && event_y != prev_position.1 {
                        delta.1 = event_y - prev_position.1;
                        pancurses_mouse_component.set_mouse_y(event_y);
                    }

                    let prev_button_mask = pancurses_mouse_component.get_button_mask();
                    if prev_button_mask != event_button_mask {
                        new_button_mask = Some(event_button_mask);
                        pancurses_mouse_component.set_button_mask(event_button_mask);
                    }
                }

                if let Some(event_queue_entity) = event_queue_entity {
                    let event_queue_component = db
                        .get_entity_component_mut::<EventQueueComponent<AntigenEvent>>(
                            event_queue_entity,
                        )?;

                    if delta != IVector2(0, 0) {
                        event_queue_component.push_event(AntigenEvent::MouseMove {
                            position: prev_position + delta,
                            delta,
                        })
                    }

                    if let Some(new_button_mask) = new_button_mask {
                        let mut pressed_mask = 0usize;
                        {
                            if (pancurses::BUTTON1_PRESSED as usize & new_button_mask) > 0 {
                                pressed_mask |= 1;
                            }
                            if (pancurses::BUTTON2_PRESSED as usize & new_button_mask) > 0 {
                                pressed_mask |= 4;
                            }
                            if (pancurses::BUTTON3_PRESSED as usize & new_button_mask) > 0 {
                                pressed_mask |= 2;
                            }
                            if (pancurses::BUTTON4_PRESSED as usize & new_button_mask) > 0 {
                                pressed_mask |= 8;
                            }
                            if (pancurses::BUTTON5_PRESSED as usize & new_button_mask) > 0 {
                                pressed_mask |= 16;
                            }
                        }

                        let mut released_mask = 0usize;
                        {
                            if (pancurses::BUTTON1_RELEASED as usize & new_button_mask) > 0 {
                                released_mask |= 1;
                            }
                            if (pancurses::BUTTON2_RELEASED as usize & new_button_mask) > 0 {
                                released_mask |= 4;
                            }
                            if (pancurses::BUTTON3_RELEASED as usize & new_button_mask) > 0 {
                                released_mask |= 2;
                            }
                            if (pancurses::BUTTON4_RELEASED as usize & new_button_mask) > 0 {
                                released_mask |= 8;
                            }
                            if (pancurses::BUTTON5_RELEASED as usize & new_button_mask) > 0 {
                                released_mask |= 16;
                            }
                        }

                        if pressed_mask != 0 {
                            event_queue_component.push_event(AntigenEvent::MousePress {
                                button_mask: pressed_mask,
                            })
                        }

                        if released_mask != 0 {
                            event_queue_component.push_event(AntigenEvent::MouseRelease {
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

impl SystemDebugTrait for PancursesInputSystem {
    fn get_name() -> &'static str {
        "Pancurses Input"
    }
}
