use crate::components::{
    pancurses_input_buffer_component::PancursesInputBufferComponent,
    pancurses_window_component::PancursesWindowComponent,
};
use antigen::{
    components::ParentEntityComponent,
    ecs::{EntityComponentDatabase, SystemEvent, SystemTrait},
ecs::EntityComponentDatabaseDebug};

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

impl<T> SystemTrait<T> for PancursesInputSystem where T: EntityComponentDatabase + EntityComponentDatabaseDebug
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

        for input in &self.input_buffer {
            if let pancurses::Input::Character('\u{1b}') = input {
                return Ok(SystemEvent::Quit);
            }

            // TODO: Remove after window teardown is implemented
            if let pancurses::Input::Character(' ') = input {
                let window_entities = db.get_entities_by_predicate(|entity_id| {
                    db.entity_has_component::<PancursesWindowComponent>(entity_id)
                });

                for entity_id in window_entities {
                    let pancurses_window_component =
                        db.get_entity_component::<PancursesWindowComponent>(entity_id)?;
                    if pancurses_window_component.window_id == 0 {
                        db.remove_component_from_entity::<PancursesWindowComponent>(entity_id)?;
                    }
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
