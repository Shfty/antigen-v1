mod software_raster_framebuffer;

pub use software_raster_framebuffer::*;

use crate::primitive_types::Vector2I;

use super::Framebuffer;

/// Framebuffer subtrait representing a rectangular grid of pixels
pub trait RasterFramebuffer<T>: Framebuffer<T> {
    fn get_size(&self) -> Self::Index;
    fn get(&self, key: Self::Index) -> T;
    fn set(&mut self, key: Self::Index, data: T);
    fn clear(&mut self);
    fn resize(&mut self, size: Self::Index);
}

impl<T, U> Framebuffer<T> for U
where
    U: RasterFramebuffer<T>,
{
    type Index = Vector2I;

    fn get_size(&self) -> Self::Index {
        RasterFramebuffer::get_size(self)
    }

    fn get(&self, key: Self::Index) -> T {
        RasterFramebuffer::get(self, key)
    }

    fn set(&mut self, key: Self::Index, data: T) {
        RasterFramebuffer::set(self, key, data)
    }

    fn clear(&mut self) {
        RasterFramebuffer::clear(self)
    }

    fn resize(&mut self, size: Self::Index) {
        RasterFramebuffer::resize(self, size)
    }
}
