use std::cell::{Ref, RefMut};

use crate::components::InputAxisData;
use antigen::{
    components::EventQueue,
    components::IntRange,
    core::events::KeyPress,
    entity_component_system::ComponentStore,
    entity_component_system::EntityID,
    entity_component_system::{SystemError, SystemTrait},
};
use store::StoreQuery;

type WriteInputAxis<'a> = (EntityID, Ref<'a, InputAxisData>, RefMut<'a, IntRange>);

#[derive(Debug)]
pub struct InputAxis;

impl SystemTrait for InputAxis {
    fn run(&mut self, db: &mut ComponentStore) -> Result<(), SystemError> {
        let (_key, event_queue) =
            StoreQuery::<(EntityID, Ref<EventQueue<KeyPress>>)>::iter(db.as_ref())
                .next()
                .expect("No antigen input event queue");

        for (_key, input_axis_data, mut int_range) in
            StoreQuery::<WriteInputAxis>::iter(db.as_ref())
        {
            let prev_input = input_axis_data.get_negative_input();
            let next_input = input_axis_data.get_positive_input();

            let mut offset: i64 = 0;

            for event in event_queue.iter() {
                let key_code = event.key_code;
                if key_code == prev_input {
                    offset -= 1;
                } else if key_code == next_input {
                    offset += 1;
                }
            }

            let index = int_range.get_index();
            int_range.set_index(index + offset);
        }

        Ok(())
    }
}
