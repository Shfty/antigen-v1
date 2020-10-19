use std::ops::{Deref, DerefMut};

use pancurses::Window;

#[derive(Debug, Default)]
pub struct CursesWindowData(pub Option<Window>);

impl Deref for CursesWindowData {
    type Target = Option<Window>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CursesWindowData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for CursesWindowData {
    fn drop(&mut self) {
        if self.0.is_some() {
            pancurses::endwin();
        }
    }
}
