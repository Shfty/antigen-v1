use crate::components::{
    pancurses_input_buffer_component::PancursesInputBufferComponent,
    pancurses_mouse_component::PancursesMouseComponent,
    pancurses_window_component::PancursesWindowComponent,
};
use antigen::{
    components::WindowComponent,
    entity_component_system::system_interface::SystemInterface,
    entity_component_system::ComponentStorage,
    entity_component_system::EntityComponentDirectory,
    entity_component_system::SystemDebugTrait,
    entity_component_system::{SystemError, SystemTrait},
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

        self.input_buffer.clear();
        if let Some(entity_id) = window_entity {
            let window_component =
                db.get_entity_component::<PancursesWindowComponent>(entity_id)?;
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

        // Check for special inputs
        for input in &self.input_buffer {
            if let pancurses::Input::Character('\u{1b}') = input {
                return Err(SystemError::Quit);
            }

            if let pancurses::Input::KeyResize = input {
                pancurses::resize_term(0, 0);
            }
        }

        let pancurses_mouse_entities =
            db.entity_component_directory
                .get_entities_by_predicate(|entity_id| {
                    db.entity_component_directory
                        .entity_has_component::<PancursesMouseComponent>(entity_id)
                });
        assert!(pancurses_mouse_entities.len() <= 1);

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
        let entities = db
            .entity_component_directory
            .get_entities_by_predicate(|entity_id| {
                db.entity_component_directory
                    .entity_has_component::<PancursesInputBufferComponent>(entity_id)
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

impl SystemDebugTrait for PancursesInputSystem {
    fn get_name() -> &'static str {
        "Pancurses Input"
    }
}
