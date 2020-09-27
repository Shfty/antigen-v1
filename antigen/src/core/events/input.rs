use crate::primitive_types::Vector2I;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum AntigenInputEvent {
    MouseMove {
        position: Vector2I,
        delta: Vector2I,
    },
    MousePress {
        button_mask: usize,
    },
    MouseRelease {
        button_mask: usize,
    },
    MouseScroll {
        delta: i8,
    },
    KeyPress {
        key_code: crate::core::keyboard::Key,
    },
    KeyRelease {
        key_code: crate::core::keyboard::Key,
    },
}
