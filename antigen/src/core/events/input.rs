use crate::primitive_types::Vector2I;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct MouseMove {
    pub position: Vector2I,
    pub delta: Vector2I,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct MousePress {
    pub button_mask: usize,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct MouseRelease {
    pub button_mask: usize,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct MouseScroll {
    pub delta: i8,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct KeyPress {
    pub key_code: crate::core::keyboard::Key,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct KeyRelease {
    pub key_code: crate::core::keyboard::Key,
}
