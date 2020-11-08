use std::{cell::Ref, cell::RefMut};

use crate::{
    components::{EventQueue, LocalMousePositionData, Position, Size},
    core::events::AntigenInputEvent,
    entity_component_system::{ComponentStore, EntityID, SystemError, SystemTrait},
    primitive_types::Vector2I,
};

use store::StoreQuery;

type ReadAntigenEventQueueEntity<'a> = (EntityID, Ref<'a, EventQueue<AntigenInputEvent>>);

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ButtonEvent {
    Pressed,
    Released,
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Button;

impl SystemTrait for Button {
    fn run(&mut self, db: &mut ComponentStore) -> Result<(), SystemError> {
        // Fetch entity references
        for (_, _, size, local_mouse_position, mut button_event_queue) in
            StoreQuery::<(
                EntityID,
                Ref<Position>,
                Ref<Size>,
                Ref<LocalMousePositionData>,
                RefMut<EventQueue<ButtonEvent>>,
            )>::iter(db.as_ref())
        {
            let Vector2I(width, height) = **size;

            // Determine whether the mouse is inside this control
            let Vector2I(mouse_x, mouse_y) = **local_mouse_position;
            let range_x = 0i64..width;
            let range_y = 0i64..height as i64;

            if range_x.contains(&mouse_x) && range_y.contains(&mouse_y) {
                if let Some((_, event_queue)) =
                    StoreQuery::<ReadAntigenEventQueueEntity>::iter(db.as_ref()).next()
                {
                    for event in event_queue.iter() {
                        match event {
                            AntigenInputEvent::MousePress { button_mask: 1 } => {
                                // Push press event into queue
                                button_event_queue.push(ButtonEvent::Pressed);
                            }
                            AntigenInputEvent::MouseRelease { button_mask: 1 } => {
                                // Push press event into queue
                                button_event_queue.push(ButtonEvent::Released);
                            }
                            _ => (),
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
