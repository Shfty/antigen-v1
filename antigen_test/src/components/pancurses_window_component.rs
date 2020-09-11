use antigen::ecs::{ComponentMetadataTrait, ComponentTrait};
use pancurses::Window;

#[derive(Debug)]
pub struct PancursesWindowComponent {
    pub window: Option<Window>,
}

impl Clone for PancursesWindowComponent {
    fn clone(&self) -> Self {
        PancursesWindowComponent { window: None }
    }
}

impl<'a> PancursesWindowComponent {
    pub fn new() -> Self {
        PancursesWindowComponent { window: None }
    }
}

impl<'a> Default for PancursesWindowComponent {
    fn default() -> Self {
        PancursesWindowComponent::new()
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
