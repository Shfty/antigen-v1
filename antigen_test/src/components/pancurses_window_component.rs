use antigen::ecs::{ComponentDebugTrait, ComponentTrait};
use pancurses::Window;

#[derive(Debug)]
pub struct PancursesWindowComponent {
    window: Option<Window>,
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

    pub fn get_window(&self) -> Option<&Window> {
        self.window.as_ref()
    }

    pub fn set_window(&mut self, window: Option<Window>) -> &mut Self {
        self.window = window;
        self
    }
}

impl<'a> Default for PancursesWindowComponent {
    fn default() -> Self {
        PancursesWindowComponent::new()
    }
}

impl ComponentTrait for PancursesWindowComponent {}

impl ComponentDebugTrait for PancursesWindowComponent {
    fn get_name() -> String {
        "Pancurses Window".into()
    }

    fn get_description() -> String {
        "Represents a window (or sub-window) inside the Pancurses renderer".into()
    }
}
