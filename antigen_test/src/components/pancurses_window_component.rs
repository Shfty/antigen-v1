use antigen::{primitive_types::UID, ecs::{ComponentMetadataTrait, ComponentTrait}};

pub type WindowID = UID;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct PancursesWindowComponent {
    pub window_id: WindowID,
}

impl PancursesWindowComponent {
    pub fn new(window_id: WindowID) -> Self {
        PancursesWindowComponent { window_id }
    }
}

impl Default for PancursesWindowComponent {
    fn default() -> Self {
        PancursesWindowComponent::new(0)
    }
}

impl ComponentTrait for PancursesWindowComponent {}

impl ComponentMetadataTrait for PancursesWindowComponent {
    fn get_name() -> &'static str {
        "Pancurses Window"
    }

    fn get_description() -> &'static str {
        "Represents a window (or sub-window) inside the Pancurses renderer"
    }
}
