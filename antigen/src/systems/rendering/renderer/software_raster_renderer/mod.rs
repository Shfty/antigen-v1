mod color_renderer;
mod string_renderer;

pub use color_renderer::*;
pub use string_renderer::*;

use std::{cell::Ref, cell::RefMut, fmt::Debug};

use store::StoreQuery;

use crate::{
    components::{Framebuffer, Size, SoftwareRasterFramebuffer, Window},
    entity_component_system::{ComponentStore, EntityID, SystemError, SystemTrait},
    primitive_types::Vector2I,
};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct RasterInput {
    pub local_pos: Vector2I,
    pub size: Vector2I,
}

impl RasterInput {
    pub fn new(local_pos: Vector2I, size: Vector2I) -> RasterInput {
        RasterInput { local_pos, size }
    }

    pub fn get_uv(self) -> (f32, f32) {
        let Vector2I(x, y) = self.local_pos;
        let Vector2I(width, height) = self.size;
        let u = (x as f32) / (width - 1) as f32;
        let v = (y as f32) / (height - 1) as f32;
        (u, v)
    }
}

/// Renderer subtrait for drawing into a 2D grid framebuffer
pub trait SoftwareRasterRenderer {
    type Output: Copy + 'static;

    fn prepare_framebuffer(&self, db: &ComponentStore) -> Option<EntityID> {
        // Fetch window size
        let (_, _, size) = StoreQuery::<(EntityID, Ref<Window>, Ref<Size>)>::iter(db.as_ref())
            .next()
            .expect("No window entity");

        let size = **size;

        // Fetch data buffer and depth buffer
        let (framebuffer_entity, mut data_buffer, mut depth_buffer) = StoreQuery::<(
            EntityID,
            RefMut<SoftwareRasterFramebuffer<Self::Output>>,
            RefMut<SoftwareRasterFramebuffer<i64>>,
        )>::iter(db.as_ref())
        .next()
        .expect("No software framebuffer entity");

        // Update data buffer and depth buffer
        data_buffer.resize(size);
        depth_buffer.resize(size);

        data_buffer.clear();
        depth_buffer.clear();

        // Render Entities
        if size.0 == 0 || size.1 == 0 {
            return None;
        }

        Some(framebuffer_entity)
    }

    fn gather_entities(&self, db: &ComponentStore) -> Vec<EntityID>;

    fn render(
        &self,
        db: &ComponentStore,
        data_buffer: &mut SoftwareRasterFramebuffer<<Self as SoftwareRasterRenderer>::Output>,
        depth_buffer: &mut SoftwareRasterFramebuffer<i64>,
        entity_id: EntityID,
    );
}

impl<T> SystemTrait for T
where
    T: Debug + SoftwareRasterRenderer + 'static,
    <T as SoftwareRasterRenderer>::Output: Clone,
{
    fn run(&mut self, db: &mut ComponentStore) -> Result<(), SystemError> {
        if let Some(framebuffer_entity) = SoftwareRasterRenderer::prepare_framebuffer(self, &db) {
            let control_entities: Vec<EntityID> =
                SoftwareRasterRenderer::gather_entities(self, &db);

            let (_, mut framebuffer, mut depth_buffer) =
                StoreQuery::<(
                    EntityID,
                    RefMut<SoftwareRasterFramebuffer<<T as SoftwareRasterRenderer>::Output>>,
                    RefMut<SoftwareRasterFramebuffer<i64>>,
                )>::get(db.as_ref(), &framebuffer_entity);

            for entity_id in control_entities {
                SoftwareRasterRenderer::render(
                    self,
                    db,
                    &mut *framebuffer,
                    &mut *depth_buffer,
                    entity_id,
                )
            }
        }

        Ok(())
    }
}
