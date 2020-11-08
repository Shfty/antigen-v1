use std::ops::{Deref, DerefMut};

use antigen::{components::RasterFramebuffer, primitive_types::Vector2I};
use pancurses::{Window, chtype};

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

impl RasterFramebuffer<chtype> for CursesWindowData {
    fn get_size(&self) -> Self::Index {
        match &self.0 {
            Some(window) => {
                let (y, x) = window.get_max_yx();
                Vector2I(x.into(), y.into())
            }
            None => panic!("Called get_size() without a valid window"),
        }
    }

    fn get(&self, key: Self::Index) -> chtype {
        match &self.0 {
            Some(window) => {
                let Vector2I(x, y) = key;

                window.mvinch(y as i32, x as i32)
            }
            None => panic!("Called get() without a valid window"),
        }
    }

    fn set(&mut self, key: Self::Index, data: chtype) {
        match &self.0 {
            Some(window) => {
                let Vector2I(x, y) = key;

                window.mvaddch(y as i32, x as i32, data);
            }
            None => panic!("Called set() without a valid window"),
        }
    }

    fn clear(&mut self) {
        match &self.0 {
            Some(window) => {
                window.erase();
            }
            None => panic!("Called clear() without a valid window"),
        }
    }

    fn resize(&mut self, _: Self::Index) {}
}
