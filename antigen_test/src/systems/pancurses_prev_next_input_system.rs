use crate::components::{
    pancurses_input_buffer_component::PancursesInputBufferComponent,
    pancurses_prev_next_input_component::PancursesPrevNextInputComponent,
};
use antigen::{
    components::IntRangeComponent,
    ecs::{SystemTrait, EntityComponentSystem, SystemEvent},
ecs::EntityComponentSystemDebug};

#[derive(Debug)]
pub struct PancursesPrevNextInputSystem;

impl PancursesPrevNextInputSystem {
    pub fn new() -> Self {
        PancursesPrevNextInputSystem
    }
}

impl<T> SystemTrait<T> for PancursesPrevNextInputSystem where T: EntityComponentSystem + EntityComponentSystemDebug {
    fn run(&mut self, ecs: &mut T) -> Result<SystemEvent, String> {
        let entities = ecs.get_entities_by_predicate(|entity_id| {
            ecs.entity_has_component::<PancursesPrevNextInputComponent>(entity_id)
                && ecs.entity_has_component::<PancursesInputBufferComponent>(entity_id)
                && ecs.entity_has_component::<IntRangeComponent>(entity_id)
        });

        for entity_id in entities {
            let pancurses_prev_next_input_component =
                ecs.get_entity_component::<PancursesPrevNextInputComponent>(entity_id)?;
            let (prev_input, next_input) = (
                pancurses_prev_next_input_component.prev_input,
                pancurses_prev_next_input_component.next_input,
            );

            let pancurses_input_buffer_component =
                ecs.get_entity_component::<PancursesInputBufferComponent>(entity_id)?;

            let mut offset: i64 = 0;

            while let Some(input) = pancurses_input_buffer_component.input_buffer.pop() {
                if input == prev_input {
                    offset -= 1;
                } else if input == next_input {
                    offset += 1;
                } else {
                    return Ok(SystemEvent::None);
                }
            }

            let ui_tab_input_component =
                ecs.get_entity_component::<IntRangeComponent>(entity_id)?;

            let new_index = (ui_tab_input_component.index as i64) + offset;

            ui_tab_input_component.index = std::cmp::min(
                std::cmp::max(new_index, ui_tab_input_component.range.start),
                ui_tab_input_component.range.end - 1,
            );
        }

        Ok(SystemEvent::None)
    }
}
