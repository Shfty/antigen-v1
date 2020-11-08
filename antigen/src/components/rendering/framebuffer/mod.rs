mod raster_framebuffer;

pub use raster_framebuffer::*;

/// Trait representing a data buffer containing T that can be rendered into
pub trait Framebuffer<T> {
    type Index;

    fn get_size(&self) -> Self::Index;
    fn get(&self, key: Self::Index) -> T;
    fn set(&mut self, key: Self::Index, data: T);
    fn clear(&mut self);
    fn resize(&mut self, size: Self::Index);
}
