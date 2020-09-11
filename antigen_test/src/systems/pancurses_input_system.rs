use crate::components::{
    pancurses_input_buffer_component::PancursesInputBufferComponent,
    pancurses_mouse_component::PancursesMouseComponent,
    pancurses_window_component::PancursesWindowComponent,
};
use antigen::{
    components::ParentEntityComponent,
    ecs::EntityComponentDatabaseDebug,
    ecs::{EntityComponentDatabase, SystemEvent, SystemTrait},
};

#[derive(Debug)]
pub struct PancursesInputSystem {
    input_buffer_size: i64,
    input_buffer: Vec<pancurses::Input>,
}

impl PancursesInputSystem {
    pub fn new(input_buffer_size: i64) -> Self {
        PancursesInputSystem {
            input_buffer_size,
            input_buffer: Vec::new(),
        }
    }
}

impl<T> SystemTrait<T> for PancursesInputSystem
where
    T: EntityComponentDatabase + EntityComponentDatabaseDebug,
{
    fn run(&mut self, db: &mut T) -> Result<SystemEvent, String> {
        self.input_buffer.clear();

        let window_entities = db.get_entities_by_predicate(|entity_id| {
            db.entity_has_component::<PancursesWindowComponent>(entity_id)
                && !db.entity_has_component::<ParentEntityComponent>(entity_id)
        });

        assert!(window_entities.len() <= 1);

        if let Some(entity_id) = window_entities.get(0) {
            let window_component =
                db.get_entity_component::<PancursesWindowComponent>(*entity_id)?;
            if let Some(window) = &window_component.window {
                for _ in 0..self.input_buffer_size {
                    if let Some(input) = window.getch() {
                        self.input_buffer.push(input);
                    } else {
                        break;
                    }
                }
            }

            pancurses::flushinp();
        }

        let pancurses_mouse_entities = db.get_entities_by_predicate(|entity_id| {
            db.entity_has_component::<PancursesMouseComponent>(entity_id)
        });
        assert!(pancurses_mouse_entities.len() <= 1);

        for input in &self.input_buffer {
            if let pancurses::Input::Character('\u{1b}') = input {
                return Ok(SystemEvent::Quit);
            }

            if let pancurses::Input::KeyMouse = input {
                if let Ok(mouse_event) = pancurses::getmouse() {
                    let pancurses_mouse_component =
                        if let Some(entity_id) = pancurses_mouse_entities.get(0) {
                            db.get_entity_component_mut::<PancursesMouseComponent>(*entity_id)?
                        } else {
                            return Err("No pancurses mouse entity".into());
                        };

                    pancurses_mouse_component.position.0 = mouse_event.x as i64;
                    pancurses_mouse_component.position.1 = mouse_event.y as i64;
                    pancurses_mouse_component.button_mask = mouse_event.bstate as i64;
                }
            }
        }

        let entities = db.get_entities_by_predicate(|entity_id| {
            db.entity_has_component::<PancursesInputBufferComponent>(entity_id)
        });

        for entity_id in entities {
            let pancurses_input_buffer_component =
                db.get_entity_component_mut::<PancursesInputBufferComponent>(entity_id)?;

            for input in &self.input_buffer {
                pancurses_input_buffer_component.input_buffer.push(*input);
            }
        }

        Ok(SystemEvent::None)
    }
}
