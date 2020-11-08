use crate::systems::{CursesColors, CursesKeyboard, CursesMouse, CursesWindow, TextColorMode};
use antigen::entity_component_system::SystemBuilder;

use crate::systems::CursesInputBuffer;

pub fn curses(builder: SystemBuilder) -> SystemBuilder {
    builder
        .system(CursesInputBuffer)
        .system(CursesKeyboard)
        .system(CursesMouse::default())
        .system(CursesWindow)
        .system(CursesColors::new(TextColorMode::BlackWhite))
}
