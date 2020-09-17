use crate::components::pancurses_input_buffer_component::PancursesInputBufferComponent;
use antigen::{
    components::VelocityComponent,
    entity_component_system::entity_component_database::ComponentStorage,
    entity_component_system::entity_component_database::EntityComponentDatabase,
    entity_component_system::entity_component_database::EntityComponentDirectory,
    entity_component_system::{SystemError, SystemTrait},
    primitive_types::IVector2,
};

#[derive(Debug)]
pub struct InputVelocitySystem;

impl InputVelocitySystem {
    pub fn new() -> Self {
        InputVelocitySystem
    }
}

impl<S, D> SystemTrait<S, D> for InputVelocitySystem
where
    S: ComponentStorage,
    D: EntityComponentDirectory,
{
    fn run(&mut self, db: &mut EntityComponentDatabase<S, D>) -> Result<(), SystemError>
    where
        S: ComponentStorage,
        D: EntityComponentDirectory,
    {
        let entities = db.get_entities_by_predicate(|entity_id| {
            db.entity_has_component::<PancursesInputBufferComponent>(entity_id)
                && db.entity_has_component::<VelocityComponent>(entity_id)
        });

        for entity_id in entities {
            let pancurses_input_buffer_component =
                db.get_entity_component::<PancursesInputBufferComponent>(entity_id)?;

            let mut move_input: IVector2 = IVector2(0, 0);
            for input in pancurses_input_buffer_component.get_inputs() {
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

            db.get_entity_component_mut::<VelocityComponent>(entity_id)?
                .set_velocity(move_input);
        }

        Ok(())
    }
}
