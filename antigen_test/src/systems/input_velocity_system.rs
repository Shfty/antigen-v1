use crate::components::pancurses_input_buffer_component::PancursesInputBufferComponent;
use antigen::{
    components::VelocityComponent,
    ecs::{SystemTrait, EntityComponentDatabase, SystemEvent},
    primitive_types::IVector2,
ecs::EntityComponentDatabaseDebug};

#[derive(Debug)]
pub struct InputVelocitySystem;

impl InputVelocitySystem {
    pub fn new() -> Self {
        InputVelocitySystem
    }
}

impl<T> SystemTrait<T> for InputVelocitySystem where T: EntityComponentDatabase + EntityComponentDatabaseDebug {
    fn run(&mut self, db: &mut T) -> Result<SystemEvent, String> {
        let entities = db.get_entities_by_predicate(|entity_id| {
            db.entity_has_component::<PancursesInputBufferComponent>(entity_id)
                && db.entity_has_component::<VelocityComponent>(entity_id)
        });

        for entity_id in entities {
            let pancurses_input_buffer_component =
                db.get_entity_component::<PancursesInputBufferComponent>(entity_id)?;

            let mut move_input: (i64, i64) = (0, 0);
            while let Some(input) = pancurses_input_buffer_component.input_buffer.pop() {
                match input {
                    pancurses::Input::KeyLeft => move_input.0 -= 1,
                    pancurses::Input::KeyRight => move_input.0 += 1,
                    pancurses::Input::KeyUp => move_input.1 -= 1,
                    pancurses::Input::KeyDown => move_input.1 += 1,
                    _ => (),
                }
            }

            move_input.0 = std::cmp::min(std::cmp::max(move_input.0, -1), 1);
            move_input.1 = std::cmp::min(std::cmp::max(move_input.1, -1), 1);

            let velocity_component = db.get_entity_component::<VelocityComponent>(entity_id)?;
            let IVector2(x_vel, y_vel) = &mut velocity_component.data;
            *x_vel = move_input.0;
            *y_vel = move_input.1;
        }

        Ok(SystemEvent::None)
    }
}
