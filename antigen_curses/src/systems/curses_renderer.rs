// TODO:    Color / pair registration shouldn't happen every frame
//          Leverage event system to handle on-update callbacks?

use std::{cell::Ref, cell::RefMut, fmt::Debug};

use antigen::{
    components::{Framebuffer, Size, SoftwareRasterFramebuffer, Window},
    entity_component_system::{ComponentStore, EntityID, SystemError, SystemTrait},
    primitive_types::{ColorRGBF, Vector2I},
};
use pancurses::{ToChtype};
use store::StoreQuery;

use crate::{components::{CursesPalette, CursesWindowData}};

type ReadCursesWindow<'a> = (
    EntityID,
    Ref<'a, Window>,
    Ref<'a, Size>,
    RefMut<'a, CursesWindowData>,
);
type ReadColorFramebuffer<'a> = (
    EntityID,
    Ref<'a, SoftwareRasterFramebuffer<ColorRGBF>>,
    Ref<'a, SoftwareRasterFramebuffer<i64>>,
);
type ReadStringFramebuffer<'a> = (
    EntityID,
    Ref<'a, SoftwareRasterFramebuffer<char>>,
    Ref<'a, SoftwareRasterFramebuffer<i64>>,
);

#[derive(Debug, Default)]
pub struct CursesRenderer;

impl SystemTrait for CursesRenderer {
    fn run(&mut self, db: &mut ComponentStore) -> Result<(), SystemError> {
        let (_, curses_palette) = StoreQuery::<(EntityID, Ref<CursesPalette>)>::iter(db.as_ref())
            .next()
            .expect("Failed to get curses palette");

        // Fetch framebuffer entities
        let (_, color_framebuffer, color_depth_buffer) =
            StoreQuery::<ReadColorFramebuffer>::iter(db.as_ref())
                .next()
                .expect("No software framebuffer entity");

        let (_, string_framebuffer, string_depth_buffer) =
            StoreQuery::<ReadStringFramebuffer>::iter(db.as_ref())
                .next()
                .expect("No string framebuffer entity");

        // Fetch window entity
        let (_, _, _, mut curses_window) = StoreQuery::<ReadCursesWindow>::iter(db.as_ref())
            .next()
            .expect("No curses window entity");

        // Composite color and string buffers into a chtype buffer
        // Fetch size and assert that all framebuffer sizes match
        let framebuffer_size = color_framebuffer.get_size();
        assert!(
            framebuffer_size == color_depth_buffer.get_size()
                && framebuffer_size == string_framebuffer.get_size()
                && framebuffer_size == string_depth_buffer.get_size()
        );

        let Vector2I(width, height) = framebuffer_size;

        let window_width = width as i32;
        let window_height = height as i32;

        let color_framebuffer = color_framebuffer.get_buffer();
        let color_depth_buffer = color_depth_buffer.get_buffer();
        let string_framebuffer = string_framebuffer.get_buffer();
        let string_depth_buffer = string_depth_buffer.get_buffer();

        let raster = (0..window_height)
            .flat_map(move |y| (0..window_width).map(move |x| Vector2I(x as i64, y as i64)))
            .enumerate()
            .flat_map(move |(idx, position)| {
                let color = color_framebuffer[idx];
                let color_z = color_depth_buffer[idx];

                let char = string_framebuffer[idx];
                let char_z = string_depth_buffer[idx];

                if color == ColorRGBF::new(0.0, 0.0, 0.0) && char == ' ' {
                    return None;
                }

                if color_z == -1 && char_z == -1 {
                    return None;
                }

                let color_idx = color.into_index(&*curses_palette);
                let color_idx: usize = color_idx.into();

                let char = match char_z.cmp(&color_z) {
                    std::cmp::Ordering::Less => ' ',
                    std::cmp::Ordering::Equal => char,
                    std::cmp::Ordering::Greater => char,
                };

                let cell = char.to_chtype() | pancurses::COLOR_PAIR(color_idx as u64);

                Some((position, cell))
            });

        // Render
        curses_window.clear();
        for (pos, cell) in raster {
            curses_window.set(pos, cell);
        }

        Ok(())
    }
}
