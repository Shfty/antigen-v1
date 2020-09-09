use antigen::{
    ecs::{ComponentMetadataTrait, ComponentTrait},
    primitive_types::UID,
};
use pancurses::Window;

pub type WindowID = UID;

#[derive(Debug)]
pub struct PancursesWindowComponent {
    pub window_id: WindowID,
    pub window: Option<Window>,
}

impl Clone for PancursesWindowComponent {
    fn clone(&self) -> Self {
        PancursesWindowComponent {
            window_id: self.window_id,
            window: None
        }
    }
}

impl<'a> PancursesWindowComponent {
    pub fn new(window_id: WindowID) -> Self {
        PancursesWindowComponent {
            window_id,
            window: None,
        }
    }
}

impl<'a> Default for PancursesWindowComponent {
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
