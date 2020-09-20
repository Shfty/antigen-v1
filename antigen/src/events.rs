use crate::primitive_types::IVector2;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum AntigenEvent {
    MouseMove { position: IVector2, delta: IVector2 },
    MousePress { button_mask: usize },
    MouseRelease { button_mask: usize },
    KeyPress { key_code: crate::Key },
    KeyRelease { key_code: crate::Key },
}
