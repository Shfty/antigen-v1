use std::borrow::{Borrow, BorrowMut};

use antigen::entity_component_system::{ComponentDebugTrait, ComponentTrait};
use pancurses::Window;

#[derive(Debug, Default)]
pub struct CursesWindow(pub Option<Window>);

impl Borrow<Option<Window>> for CursesWindow {
    fn borrow(&self) -> &Option<Window> {
        &self.0
    }
}

impl BorrowMut<Option<Window>> for CursesWindow {
    fn borrow_mut(&mut self) -> &mut Option<Window> {
        &mut self.0
    }
}

impl ComponentTrait for CursesWindow {}

impl ComponentDebugTrait for CursesWindow {
    fn get_name() -> String {
        "Curses Window".into()
    }

    fn get_description() -> String {
        "Represents a window (or sub-window) inside the Curses renderer".into()
    }
}
