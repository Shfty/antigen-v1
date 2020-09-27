use antigen::entity_component_system::ComponentDebugTrait;
use pancurses::Window;

#[derive(Debug, Default)]
pub struct CursesWindow(pub Option<Window>);

impl AsRef<Option<Window>> for CursesWindow {
    fn as_ref(&self) -> &Option<Window> {
        &self.0
    }
}

impl AsMut<Option<Window>> for CursesWindow {
    fn as_mut(&mut self) -> &mut Option<Window> {
        &mut self.0
    }
}

impl ComponentDebugTrait for CursesWindow {
    fn get_name() -> String {
        "Curses Window".into()
    }

    fn get_description() -> String {
        "Represents a window (or sub-window) inside the Curses renderer".into()
    }
}
