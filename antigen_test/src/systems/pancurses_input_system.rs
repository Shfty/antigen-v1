use crate::components::{
    pancurses_input_buffer_component::PancursesInputBufferComponent,
    pancurses_mouse_component::PancursesMouseComponent,
    pancurses_window_component::PancursesWindowComponent,
};
use antigen::{
    components::ParentEntityComponent,
    entity_component_system::{EntityComponentDirectory, SystemError, SystemTrait},
entity_component_system::ComponentStorage, entity_component_system::entity_component_database::EntityComponentDatabase};

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

impl<S, D> SystemTrait<S, D> for PancursesInputSystem
where
    S: ComponentStorage,
    D: EntityComponentDirectory,
{
    fn run(&mut self, db: &mut EntityComponentDatabase<S, D>) -> Result<(), SystemError>
    where
        S: ComponentStorage,
        D: EntityComponentDirectory
    {
        self.input_buffer.clear();

        let window_entities = db.get_entities_by_predicate(|entity_id| {
            db.entity_has_component::<PancursesWindowComponent>(entity_id)
                && !db.entity_has_component::<ParentEntityComponent>(entity_id)
        });

        assert!(window_entities.len() <= 1);

        if let Some(entity_id) = window_entities.get(0) {
            let window_component =
                db.get_entity_component::<PancursesWindowComponent>(*entity_id)?;
            if let Some(window) = window_component.get_window() {
                let mut i = self.input_buffer_size;
                while let Some(input) = window.getch() {
                    self.input_buffer.push(input);
                    i -= 1;
                    if i <= 0 {
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

        // Check for special inputs
        for input in &self.input_buffer {
            if let pancurses::Input::Character('\u{1b}') = input {
                return Err(SystemError::Quit);
            }

            if let pancurses::Input::KeyResize = input {
                pancurses::resize_term(0, 0);
            }
        }

        // Check for mouse input
        if let Ok(mouse_event) = pancurses::getmouse() {
            let pancurses_mouse_component = if let Some(entity_id) = pancurses_mouse_entities.get(0)
            {
                db.get_entity_component_mut::<PancursesMouseComponent>(*entity_id)?
            } else {
                return Err("No pancurses mouse entity".into());
            };

            if mouse_event.x > -1 {
                pancurses_mouse_component.set_mouse_x(mouse_event.x as i64);
            }
            if mouse_event.y > -1 {
                pancurses_mouse_component.set_mouse_y(mouse_event.y as i64);
            }
            pancurses_mouse_component.set_button_mask(mouse_event.bstate as i64);
        }

        // Update entity input buffers
        let entities = db.get_entities_by_predicate(|entity_id| {
            db.entity_has_component::<PancursesInputBufferComponent>(entity_id)
        });

        for entity_id in entities {
            let pancurses_input_buffer_component =
                db.get_entity_component_mut::<PancursesInputBufferComponent>(entity_id)?;

            pancurses_input_buffer_component.clear();

            for input in &self.input_buffer {
                pancurses_input_buffer_component.push(*input);
            }
        }

        Ok(())
    }
}
