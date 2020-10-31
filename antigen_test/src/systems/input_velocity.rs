use std::cell::{Ref, RefMut};

use antigen::{
    components::EventQueue,
    components::Velocity,
    core::events::AntigenInputEvent,
    entity_component_system::ComponentStore,
    entity_component_system::{EntityID, SystemError, SystemTrait},
    primitive_types::Vector2I,
};
use store::StoreQuery;

#[derive(Debug)]
pub struct InputVelocity;

impl SystemTrait for InputVelocity {
    fn run(&mut self, db: &mut ComponentStore) -> Result<(), SystemError> {
        let (_key, event_queue) =
            StoreQuery::<(EntityID, Ref<EventQueue<AntigenInputEvent>>)>::iter(db.as_ref())
                .next()
                .expect("No antigen input event queue");

        let mut move_input: Vector2I = Vector2I(0, 0);

        for input in event_queue.iter() {
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

        for (_key, mut velocity) in
            StoreQuery::<(EntityID, RefMut<Velocity>)>::iter(db.as_ref())
        {
            **velocity = move_input;
        }

        Ok(())
    }
}
