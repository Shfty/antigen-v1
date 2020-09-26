use antigen::entity_component_system::{ComponentDebugTrait, ComponentTrait};
use pancurses::Window;

#[derive(Debug)]
pub struct CursesWindowComponent {
    window: Option<Window>,
}

impl Clone for CursesWindowComponent {
    fn clone(&self) -> Self {
        CursesWindowComponent { window: None }
    }
}

impl<'a> CursesWindowComponent {
    pub fn new() -> Self {
        CursesWindowComponent { window: None }
    }

    pub fn get_window(&self) -> Option<&Window> {
        self.window.as_ref()
    }

    pub fn set_window(&mut self, window: Option<Window>) -> &mut Self {
        self.window = window;
        self
    }
}

impl<'a> Default for CursesWindowComponent {
    fn default() -> Self {
        CursesWindowComponent::new()
    }
}

impl ComponentTrait for CursesWindowComponent {}

impl ComponentDebugTrait for CursesWindowComponent {
    fn get_name() -> String {
        "Pancurses Window".into()
    }

    fn get_description() -> String {
        "Represents a window (or sub-window) inside the Pancurses renderer".into()
    }
}
