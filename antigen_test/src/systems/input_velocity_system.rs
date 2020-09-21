use antigen::{
    components::EventQueueComponent,
    components::VelocityComponent,
    entity_component_system::system_interface::SystemInterface,
    entity_component_system::ComponentStorage,
    entity_component_system::EntityComponentDirectory,
    entity_component_system::SystemDebugTrait,
    entity_component_system::{SystemError, SystemTrait},
    events::AntigenEvent,
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
                        .entity_has_component::<EventQueueComponent<AntigenEvent>>(entity_id)
                });

        if let Some(antigen_event_queue_entity) = antigen_event_queue_entity {
            let mut move_input: Vector2I = Vector2I(0, 0);
            for input in db
                .get_entity_component::<EventQueueComponent<AntigenEvent>>(
                    antigen_event_queue_entity,
                )?
                .get_events()
            {
                match input {
                    AntigenEvent::KeyPress {
                        key_code: antigen::keyboard::Key::Left,
                    } => move_input.0 -= 1,
                    AntigenEvent::KeyPress {
                        key_code: antigen::keyboard::Key::Right,
                    } => move_input.0 += 1,
                    AntigenEvent::KeyPress {
                        key_code: antigen::keyboard::Key::Up,
                    } => move_input.1 -= 1,
                    AntigenEvent::KeyPress {
                        key_code: antigen::keyboard::Key::Down,
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
                        .entity_has_component::<VelocityComponent>(entity_id)
                });

            for entity_id in entities {
                db.get_entity_component_mut::<VelocityComponent>(entity_id)?
                    .set_velocity(move_input);
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
