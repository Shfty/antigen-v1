// TODO:    Color / pair registration shouldn't happen every frame
//          Leverage event system to handle on-update callbacks?

use std::{cell::Ref, cell::RefMut, fmt::Debug};

use antigen::{
    components::EventQueue,
    core::palette::{Palette, RGBArrangementPalette},
    entity_component_system::{ComponentStore, EntityID, SystemError, SystemTrait},
    primitive_types::{ColorRGB, ColorRGBF},
};
use store::StoreQuery;

use crate::components::{CursesPalette, CursesWindowData};

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum CursesColorsEvent {
    SetPalette(RGBArrangementPalette),
}

#[derive(Debug, Copy, Clone)]
pub enum TextColorMode {
    BlackWhite,
    Invert,
    Color(ColorRGBF),
    Function(fn(ColorRGBF) -> ColorRGBF),
}

#[derive(Debug)]
pub struct CursesColors {
    text_color_mode: TextColorMode,
}

impl CursesColors {
    pub fn new(text_color_mode: TextColorMode) -> Self {
        CursesColors { text_color_mode }
    }
}

impl SystemTrait for CursesColors {
    fn run(&mut self, db: &mut ComponentStore) -> Result<(), SystemError> {
        let (_, mut curses_palette, mut event_queue) = StoreQuery::<(
            EntityID,
            RefMut<CursesPalette>,
            RefMut<EventQueue<CursesColorsEvent>>,
        )>::iter(db.as_ref())
        .next()
        .expect("Failed to get curses palette");

        for event in event_queue.drain(..) {
            match event {
                CursesColorsEvent::SetPalette(palette) => {
                    curses_palette.set_palette(palette);
                    let colors = curses_palette.get_colors();

                    let (_, curses_window) =
                        StoreQuery::<(EntityID, Ref<CursesWindowData>)>::iter(db.as_ref())
                            .next()
                            .expect("Failed to get CursesWindowData");

                    let curses_window = (**curses_window)
                        .as_ref()
                        .expect("Failed to get curses window");

                    curses_window.erase();
                    curses_window.refresh();

                    for (i, color) in colors.into_iter().enumerate() {
                        let ColorRGB(r, g, b) = color;
                        let i = i as i16;

                        // Register color with pancurses
                        pancurses::init_color(
                            i,
                            (r * 1000.0) as i16,
                            (g * 1000.0) as i16,
                            (b * 1000.0) as i16,
                        );

                        // Calculate text color
                        let foreground_color = match self.text_color_mode {
                            TextColorMode::Color(color) => color,
                            TextColorMode::BlackWhite => {
                                if ColorRGB::distance(&color, &ColorRGBF::new(1.0, 1.0, 1.0))
                                    > ColorRGB::distance(&color, &ColorRGBF::new(0.0, 0.0, 0.0))
                                {
                                    ColorRGBF::new(1.0, 1.0, 1.0)
                                } else {
                                    ColorRGBF::new(0.0, 0.0, 0.0)
                                }
                            }
                            TextColorMode::Invert => ColorRGBF::new(1.0, 1.0, 1.0) - color,
                            TextColorMode::Function(func) => func(color),
                        }
                        .into_index(&*curses_palette);

                        // Register color pair with pancurses
                        let foreground_color: usize = foreground_color.into();
                        let foreground_color: i16 = foreground_color as i16;
                        let background_color = i;
                        pancurses::init_pair(i, foreground_color, background_color);
                    }
                }
            }
        }

        Ok(())
    }
}
