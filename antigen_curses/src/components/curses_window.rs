use pancurses::Window;

#[derive(Debug, Default)]
pub struct CursesWindowData(pub Option<Window>);

impl AsRef<Option<Window>> for CursesWindowData {
    fn as_ref(&self) -> &Option<Window> {
        &self.0
    }
}

impl AsMut<Option<Window>> for CursesWindowData {
    fn as_mut(&mut self) -> &mut Option<Window> {
        &mut self.0
    }
}
