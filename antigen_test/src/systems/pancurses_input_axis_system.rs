use crate::components::{
    pancurses_input_axis_component::PancursesInputAxisComponent,
    pancurses_input_buffer_component::PancursesInputBufferComponent,
};
use antigen::{
    components::IntRangeComponent,
    entity_component_system::entity_component_database::ComponentStorage,
    entity_component_system::entity_component_database::EntityComponentDatabase,
    entity_component_system::entity_component_database::EntityComponentDirectory,
    entity_component_system::get_entity_component,
    entity_component_system::get_entity_component_mut,
    entity_component_system::{SystemError, SystemTrait},
};

#[derive(Debug)]
pub struct PancursesInputAxisSystem;

impl PancursesInputAxisSystem {
    pub fn new() -> Self {
        PancursesInputAxisSystem
    }
}

impl<CS, CD> SystemTrait<CS, CD> for PancursesInputAxisSystem
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
{
    fn run(&mut self, db: &mut EntityComponentDatabase<CS, CD>) -> Result<(), SystemError>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
    {
        let entities = db.entity_component_directory.get_entities_by_predicate(|entity_id| {
            db.entity_component_directory.entity_has_component::<PancursesInputAxisComponent>(entity_id)
                && db.entity_component_directory.entity_has_component::<PancursesInputBufferComponent>(entity_id)
                && db.entity_component_directory.entity_has_component::<IntRangeComponent>(entity_id)
        });

        for entity_id in entities {
            let pancurses_prev_next_input_component =
                get_entity_component::<CS, CD, PancursesInputAxisComponent>(
                    &db.component_storage,
                    &db.entity_component_directory,
                    entity_id,
                )?;
            let (prev_input, next_input) = (
                pancurses_prev_next_input_component.get_negative_input(),
                pancurses_prev_next_input_component.get_positive_input(),
            );

            let pancurses_input_buffer_component =
                get_entity_component_mut::<CS, CD, PancursesInputBufferComponent>(
                    &mut db.component_storage,
                    &mut db.entity_component_directory,
                    entity_id,
                )?;

            let mut offset: i64 = 0;

            for input in pancurses_input_buffer_component.get_inputs() {
                let input = input;

                if input == prev_input {
                    offset -= 1;
                } else if input == next_input {
                    offset += 1;
                } else {
                    return Ok(());
                }
            }

            let ui_tab_input_component = get_entity_component_mut::<CS, CD, IntRangeComponent>(
                &mut db.component_storage,
                &mut db.entity_component_directory,
                entity_id,
            )?;

            ui_tab_input_component.set_index(ui_tab_input_component.get_index() + offset);
        }

        Ok(())
    }
}
