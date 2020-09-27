use std::borrow::Borrow;

use crate::CursesWindow;
use antigen::{
    components::{Size, SoftwareFramebuffer, Window},
    core::palette::Palette,
    entity_component_system::SystemDebugTrait,
    entity_component_system::{
        system_interface::SystemInterface, ComponentStorage, EntityComponentDirectory, SystemError,
        SystemTrait,
    },
    primitive_types::Color,
    primitive_types::ColorRGBF,
};
use pancurses::ToChtype;

#[derive(Debug, Copy, Clone)]
pub enum TextColorMode {
    BlackWhite,
    Invert,
    Color(ColorRGBF),
}

#[derive(Debug)]
pub struct CursesRendererSystem<T>
where
    T: Palette<f32, f32>,
{
    palette: T,
    text_color_mode: TextColorMode,
}

impl<T> CursesRendererSystem<T>
where
    T: Palette<f32, f32>,
{
    pub fn new(palette: T, text_color_mode: TextColorMode) -> Self {
        CursesRendererSystem {
            palette,
            text_color_mode,
        }
    }
}

impl<CS, CD, T> SystemTrait<CS, CD> for CursesRendererSystem<T>
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
    T: Palette<f32, f32>,
{
    fn run(&mut self, db: &mut SystemInterface<CS, CD>) -> Result<(), SystemError>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
    {
        // Fetch window entity
        let window_entity = db
            .entity_component_directory
            .get_entity_by_predicate(|entity_id| {
                db.entity_component_directory
                    .entity_has_component::<Window>(entity_id)
                    && db
                        .entity_component_directory
                        .entity_has_component::<CursesWindow>(entity_id)
                    && db
                        .entity_component_directory
                        .entity_has_component::<Size>(entity_id)
            })
            .ok_or("No window entity")?;

        let window_width: i64;
        let window_height: i64;
        {
            let window_component = db.get_entity_component::<CursesWindow>(window_entity)?;

            let window: &Option<pancurses::Window> = window_component.borrow();
            if let Some(window) = window {
                let (height, width) = window.get_max_yx();

                window_width = width as i64;
                window_height = height as i64;
            } else {
                panic!("Error fetching window handle");
            }
        }

        // Fetch CPU framebuffer entity
        let cpu_framebuffer_entity = db
            .entity_component_directory
            .get_entity_by_predicate(|entity_id| {
                db.entity_component_directory
                    .entity_has_component::<SoftwareFramebuffer<ColorRGBF>>(entity_id)
            })
            .expect("CPU framebuffer entity does not exist");

        let color_buffer = db
            .get_entity_component::<SoftwareFramebuffer<ColorRGBF>>(cpu_framebuffer_entity)?
            .get_color_buffer();

        let color_z_buffer = db
            .get_entity_component::<SoftwareFramebuffer<ColorRGBF>>(cpu_framebuffer_entity)?
            .get_z_buffer();

        // Fetch string framebuffer entity
        let string_framebuffer_enity = db
            .entity_component_directory
            .get_entity_by_predicate(|entity_id| {
                db.entity_component_directory
                    .entity_has_component::<SoftwareFramebuffer<char>>(entity_id)
            })
            .expect("String framebuffer entity does not exist");

        let char_buffer = db
            .get_entity_component::<SoftwareFramebuffer<char>>(string_framebuffer_enity)?
            .get_color_buffer();

        let char_z_buffer = db
            .get_entity_component::<SoftwareFramebuffer<char>>(string_framebuffer_enity)?
            .get_z_buffer();

        // Create pancurses > palette map to make sure built-in pancurses colors are respected
        let indices = [
            (
                pancurses::COLOR_BLACK,
                self.palette.get_color_idx(Color(0.0, 0.0, 0.0)),
            ),
            (
                pancurses::COLOR_BLUE,
                self.palette.get_color_idx(Color(0.0, 0.0, 1.0)),
            ),
            (
                pancurses::COLOR_CYAN,
                self.palette.get_color_idx(Color(0.0, 1.0, 1.0)),
            ),
            (
                pancurses::COLOR_GREEN,
                self.palette.get_color_idx(Color(0.0, 1.0, 0.0)),
            ),
            (
                pancurses::COLOR_MAGENTA,
                self.palette.get_color_idx(Color(1.0, 1.0, 0.0)),
            ),
            (
                pancurses::COLOR_RED,
                self.palette.get_color_idx(Color(1.0, 0.0, 0.0)),
            ),
            (
                pancurses::COLOR_YELLOW,
                self.palette.get_color_idx(Color(1.0, 0.0, 1.0)),
            ),
            (
                pancurses::COLOR_WHITE,
                self.palette.get_color_idx(Color(1.0, 1.0, 1.0)),
            ),
        ];

        let mut colors = self.palette.get_colors();
        for (pancurses_idx, palette_idx) in indices.iter() {
            colors.swap(*palette_idx, *pancurses_idx as usize);
        }

        // Register colors
        for (i, color) in colors.iter().enumerate() {
            let Color(r, g, b) = color;
            let i = i as i16;

            pancurses::init_color(
                i,
                (r * 1000.0) as i16,
                (g * 1000.0) as i16,
                (b * 1000.0) as i16,
            );

            let foreground_color = match self.text_color_mode {
                TextColorMode::Color(color) => self.palette.get_color_idx(color),
                TextColorMode::BlackWhite => {
                    if Color::square_distance(color, &Color(1.0f32, 1.0f32, 1.0f32))
                        > Color::square_distance(color, &Color(0.0f32, 0.0f32, 0.0f32))
                    {
                        self.palette.get_color_idx(Color(1.0f32, 1.0f32, 1.0f32))
                    } else {
                        self.palette.get_color_idx(Color(0.0f32, 0.0f32, 0.0f32))
                    }
                }
                TextColorMode::Invert => self
                    .palette
                    .get_color_idx(Color(1.0f32, 1.0f32, 1.0f32) - *color),
            };

            let mut foreground_color = foreground_color as i16;
            for (pancurses_idx, palette_idx) in indices.iter() {
                if foreground_color == *pancurses_idx {
                    foreground_color = *palette_idx as i16;
                } else if foreground_color == *palette_idx as i16 {
                    foreground_color = *pancurses_idx;
                }
            }

            let background_color = i;

            pancurses::init_pair(i, foreground_color, background_color);
        }

        let mut cells: Vec<(i32, i32, char, i16)> = Vec::new();
        let window_width = window_width as i32;
        let window_height = window_height as i32;
        for y in 0..window_height as i32 {
            for x in 0..window_width as i32 {
                let idx = (y * window_width + x) as usize;

                let color = color_buffer[idx];
                let color_z = color_z_buffer[idx];

                let char = char_buffer[idx];
                let char_z = char_z_buffer[idx];

                if color == Color(0.0, 0.0, 0.0) && char == ' ' {
                    continue;
                }

                if color_z.is_none() && char_z.is_none() {
                    continue;
                }

                let mut color_pair = self.palette.get_color_idx(color) as i16;
                for (pancurses_idx, palette_idx) in indices.iter() {
                    if color_pair == *pancurses_idx {
                        color_pair = *palette_idx as i16;
                    } else if color_pair == *palette_idx as i16 {
                        color_pair = *pancurses_idx;
                    }
                }

                let char = match char_z.cmp(&color_z) {
                    std::cmp::Ordering::Less => ' ',
                    std::cmp::Ordering::Equal => char,
                    std::cmp::Ordering::Greater => char,
                };

                cells.push((x, y, char, color_pair));
            }
        }

        let window_component = db.get_entity_component::<CursesWindow>(window_entity)?;
        let window: &Option<pancurses::Window> = window_component.borrow();
        if let Some(window) = window {
            window.erase();
            for (x, y, char, color_pair) in cells {
                window.mvaddch(
                    y as i32,
                    x as i32,
                    char.to_chtype() | pancurses::COLOR_PAIR(color_pair as u64),
                );
            }

            Ok(())
        } else {
            Err("Error fetching window handle".into())
        }
    }
}

impl<T> SystemDebugTrait for CursesRendererSystem<T>
where
    T: Palette<f32, f32>,
{
    fn get_name() -> &'static str {
        "Curses Renderer"
    }
}
