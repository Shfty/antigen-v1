use std::{
    cell::{Ref, RefMut},
    ops::Deref,
};

use store::StoreQuery;

use crate::{
    components::EventQueue,
    components::Size,
    core::events::{MouseMove, MousePress, MouseRelease, MouseScroll},
    entity_component_system::{ComponentStore, EntityID, SystemError, SystemTrait},
    primitive_types::Vector2I,
};

use crate::components::LocalMousePositionData;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct LocalMouseMove(MouseMove);

impl Deref for LocalMouseMove {
    type Target = MouseMove;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct LocalMousePress(MousePress);

impl Deref for LocalMousePress {
    type Target = MousePress;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct LocalMouseRelease(MouseRelease);

impl Deref for LocalMouseRelease {
    type Target = MouseRelease;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct LocalMouseScroll(MouseScroll);

impl Deref for LocalMouseScroll {
    type Target = MouseScroll;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LocalMouseEvents;

impl SystemTrait for LocalMouseEvents {
    fn run(&mut self, db: &mut ComponentStore) -> Result<(), SystemError> {
        let (_, mouse_move_event_queue) =
            StoreQuery::<(EntityID, Ref<EventQueue<MouseMove>>)>::iter(db.as_ref())
                .next()
                .expect("No mouse move event queue");

        let (_, mouse_press_event_queue) =
            StoreQuery::<(EntityID, Ref<EventQueue<MousePress>>)>::iter(db.as_ref())
                .next()
                .expect("No mouse press event queue");

        let (_, mouse_release_event_queue) =
            StoreQuery::<(EntityID, Ref<EventQueue<MouseRelease>>)>::iter(db.as_ref())
                .next()
                .expect("No mouse release event queue");

        let (_, mouse_scroll_event_queue) =
            StoreQuery::<(EntityID, Ref<EventQueue<MouseScroll>>)>::iter(db.as_ref())
                .next()
                .expect("No mouse scroll event queue");

        for (
            entity_id,
            local_mouse_position,
            size,
            local_mouse_move_queue,
            local_mouse_press_queue,
            local_mouse_release_queue,
            local_mouse_scroll_queue,
        ) in StoreQuery::<(
            EntityID,
            Ref<LocalMousePositionData>,
            Ref<Size>,
            Option<RefMut<EventQueue<LocalMouseMove>>>,
            Option<RefMut<EventQueue<LocalMousePress>>>,
            Option<RefMut<EventQueue<LocalMouseRelease>>>,
            Option<RefMut<EventQueue<LocalMouseScroll>>>,
        )>::iter(db.as_ref())
        {
            let Vector2I(mouse_x, mouse_y) = **local_mouse_position;
            let Vector2I(width, height) = **size;
            let range_x = 0i64..width;
            let range_y = 0i64..height as i64;

            if range_x.contains(&mouse_x) && range_y.contains(&mouse_y) {
                if let Some(mut local_mouse_move_queue) = local_mouse_move_queue {
                    for mouse_move in mouse_move_event_queue.iter() {
                        local_mouse_move_queue.push(LocalMouseMove(MouseMove {
                            position: **local_mouse_position,
                            delta: mouse_move.delta,
                        }));
                    }
                }

                if let Some(mut local_mouse_press_queue) = local_mouse_press_queue {
                    for mouse_press in mouse_press_event_queue.iter() {
                        local_mouse_press_queue.push(LocalMousePress(*mouse_press));
                    }
                }

                if let Some(mut local_mouse_release_queue) = local_mouse_release_queue {
                    for mouse_release in mouse_release_event_queue.iter() {
                        local_mouse_release_queue.push(LocalMouseRelease(*mouse_release));
                    }
                }

                if let Some(mut local_mouse_scroll_queue) = local_mouse_scroll_queue {
                    for mouse_scroll in mouse_scroll_event_queue.iter() {
                        local_mouse_scroll_queue.push(LocalMouseScroll(*mouse_scroll));
                    }
                }
            }
        }

        Ok(())
    }
}
