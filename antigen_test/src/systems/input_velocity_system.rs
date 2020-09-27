use antigen::{
    components::EventQueue,
    components::Velocity,
    core::events::AntigenInputEvent,
    entity_component_system::system_interface::SystemInterface,
    entity_component_system::ComponentStorage,
    entity_component_system::EntityComponentDirectory,
    entity_component_system::SystemDebugTrait,
    entity_component_system::{SystemError, SystemTrait},
    primitive_types::Vector2I,
};

#[derive(Debug)]
pub struct InputVelocitySystem;

impl InputVelocitySystem {
    pub fn new() -> Self {
        InputVelocitySystem
    }
}

impl<CS, CD> SystemTrait<CS, CD> for InputVelocitySystem
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
{
    fn run(&mut self, db: &mut SystemInterface<CS, CD>) -> Result<(), SystemError>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
    {
        let antigen_event_queue_entity =
            db.entity_component_directory
                .get_entity_by_predicate(|entity_id| {
                    db.entity_component_directory
                        .entity_has_component::<EventQueue<AntigenInputEvent>>(entity_id)
                });

        if let Some(antigen_event_queue_entity) = antigen_event_queue_entity {
            let mut move_input: Vector2I = Vector2I(0, 0);

            let event_queue: &Vec<AntigenInputEvent> = db
                .get_entity_component::<EventQueue<AntigenInputEvent>>(antigen_event_queue_entity)?
                .as_ref();

            for input in event_queue {
                match input {
                    AntigenInputEvent::KeyPress {
                        key_code: antigen::core::keyboard::Key::Left,
                    } => move_input.0 -= 1,
                    AntigenInputEvent::KeyPress {
                        key_code: antigen::core::keyboard::Key::Right,
                    } => move_input.0 += 1,
                    AntigenInputEvent::KeyPress {
                        key_code: antigen::core::keyboard::Key::Up,
                    } => move_input.1 -= 1,
                    AntigenInputEvent::KeyPress {
                        key_code: antigen::core::keyboard::Key::Down,
                    } => move_input.1 += 1,
                    _ => (),
                }
            }
            move_input.0 = std::cmp::min(std::cmp::max(move_input.0, -1), 1);
            move_input.1 = std::cmp::min(std::cmp::max(move_input.1, -1), 1);

            let entities = db
                .entity_component_directory
                .get_entities_by_predicate(|entity_id| {
                    db.entity_component_directory
                        .entity_has_component::<Velocity>(entity_id)
                });

            for entity_id in entities {
                *db.get_entity_component_mut::<Velocity>(entity_id)? = move_input.into();
            }
        }

        Ok(())
    }
}

impl SystemDebugTrait for InputVelocitySystem {
    fn get_name() -> &'static str {
        "Input Velocity"
    }
}
