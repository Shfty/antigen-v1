use std::fmt::Debug;

use crate::primitive_types::Vector2I;

use super::RasterFramebuffer;

#[derive(Clone, PartialEq, PartialOrd)]
pub struct SoftwareRasterFramebuffer<T> {
    size: Vector2I,
    clear_data: T,
    buffer: Vec<T>,
}

impl<T> Debug for SoftwareRasterFramebuffer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SoftwareRasterFramebuffer")
            .field("size", &self.size)
            .finish()
    }
}

impl<T> SoftwareRasterFramebuffer<T>
where
    T: Copy,
{
    pub fn new(size: Vector2I, clear_data: T) -> SoftwareRasterFramebuffer<T> {
        SoftwareRasterFramebuffer {
            size,
            clear_data,
            buffer: Vec::with_capacity((size.0 * size.1) as usize),
        }
    }

    pub fn get_buffer(&self) -> &Vec<T> {
        &self.buffer
    }

    fn get_idx(&self, Vector2I(x, y): Vector2I) -> usize {
        let idx = y * self.size.0 + x;
        idx as usize
    }
}

impl<T> RasterFramebuffer<T> for SoftwareRasterFramebuffer<T>
where
    T: Copy,
{
    fn get_size(&self) -> Vector2I {
        self.size
    }

    fn get(&self, key: Vector2I) -> T {
        let idx = self.get_idx(key);
        self.buffer[idx]
    }

    fn set(&mut self, key: Vector2I, data: T) {
        let idx = self.get_idx(key);

        self.buffer[idx] = data;
    }

    fn clear(&mut self) {
        let clear_data = self.clear_data;

        self.buffer
            .iter_mut()
            .for_each(|color| *color = clear_data);
    }

    fn resize(&mut self, size: Vector2I) {
        self.size = size;

        let Vector2I(width, height) = size;
        let new_size = width * height;
        let new_size = new_size as usize;

        if self.buffer.len() != new_size {
            self.buffer.resize(new_size, self.clear_data);
        }
    }
}
