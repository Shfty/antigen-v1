use crate::components::{
    pancurses_input_axis_component::PancursesInputAxisComponent,
    pancurses_input_buffer_component::PancursesInputBufferComponent,
};
use antigen::{
    components::IntRangeComponent,
    ecs::{EntityComponentDatabase, SystemError, SystemTrait},
};

#[derive(Debug)]
pub struct PancursesInputAxisSystem;

impl PancursesInputAxisSystem {
    pub fn new() -> Self {
        PancursesInputAxisSystem
    }
}

impl<T> SystemTrait<T> for PancursesInputAxisSystem
where
    T: EntityComponentDatabase,
{
    fn run(&mut self, db: &mut T) -> Result<(), SystemError> {
        let entities = db.get_entities_by_predicate(|entity_id| {
            db.entity_has_component::<PancursesInputAxisComponent>(entity_id)
                && db.entity_has_component::<PancursesInputBufferComponent>(entity_id)
                && db.entity_has_component::<IntRangeComponent>(entity_id)
        });

        for entity_id in entities {
            let pancurses_prev_next_input_component =
                db.get_entity_component::<PancursesInputAxisComponent>(entity_id)?;
            let (prev_input, next_input) = (
                pancurses_prev_next_input_component.get_negative_input(),
                pancurses_prev_next_input_component.get_positive_input(),
            );

            let pancurses_input_buffer_component =
                db.get_entity_component_mut::<PancursesInputBufferComponent>(entity_id)?;

            let mut offset: i64 = 0;

            while let Some(input) = pancurses_input_buffer_component.pop() {
                if input == prev_input {
                    offset -= 1;
                } else if input == next_input {
                    offset += 1;
                } else {
                    return Ok(());
                }
            }

            let ui_tab_input_component =
                db.get_entity_component_mut::<IntRangeComponent>(entity_id)?;

            ui_tab_input_component.set_index(ui_tab_input_component.get_index() + offset);
        }

        Ok(())
    }
}
