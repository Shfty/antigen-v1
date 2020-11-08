use std::{cell::Ref, marker::PhantomData};

use store::StoreQuery;

use crate::components::{Framebuffer, GlobalZIndex, SoftwareRasterFramebuffer};
use crate::{
    components::{GlobalPosition, Position, Size, SoftwareShader},
    entity_component_system::{ComponentStore, EntityID},
    primitive_types::Vector2I,
};

use super::{RasterInput, SoftwareRasterRenderer};

#[derive(Debug, Default)]
pub struct ColorRenderer<T> {
    _phantom: PhantomData<T>,
}

impl<T> ColorRenderer<T> {
    fn rasterize_rect(
        position: Vector2I,
        size: Vector2I,
        window_size: Vector2I,
    ) -> impl Iterator<Item = Vector2I> {
        // Fetch rect data
        let Vector2I(x, y) = position;
        let Vector2I(width, height) = size;
        let Vector2I(window_width, window_height) = window_size;

        // Clip rect to window
        let min_x = std::cmp::max(x, 0);
        let max_x = std::cmp::min(x + width, window_width);

        let min_y = std::cmp::max(y, 0);
        let max_y = std::cmp::min(y + height, window_height);

        // Convert into a Vector2I iterator
        (min_y..max_y).flat_map(move |y| (min_x..max_x).map(move |x| Vector2I(x, y)))
    }
}

impl<T> SoftwareRasterRenderer for ColorRenderer<T>
where
    T: Copy + 'static,
{
    type Output = T;

    fn gather_entities(&self, db: &ComponentStore) -> Vec<EntityID> {
        StoreQuery::<(
            EntityID,
            Ref<Position>,
            Ref<Size>,
            Ref<SoftwareShader<RasterInput, T>>,
        )>::iter(db.as_ref())
        .map(|(entity_id, _, _, _)| entity_id)
        .collect()
    }

    fn render(
        &self,
        db: &ComponentStore,
        framebuffer: &mut SoftwareRasterFramebuffer<T>,
        depth_buffer: &mut SoftwareRasterFramebuffer<i64>,
        entity_id: EntityID,
    ) {
        let (_, position, size, shader, global_position, global_z) =
            StoreQuery::<(
                EntityID,
                Ref<Position>,
                Ref<Size>,
                Ref<SoftwareShader<RasterInput, T>>,
                Option<Ref<GlobalPosition>>,
                Option<Ref<GlobalZIndex>>,
            )>::get(db.as_ref(), &entity_id);

        // Fetch rect data
        let position = if let Some(global_position) = global_position {
            **global_position
        } else {
            **position
        };

        let size = **size;
        let window_size = framebuffer.get_size();
        let z = if let Some(global_z) = global_z {
            **global_z
        } else {
            0
        };

        // Rasterize
        let raster = Self::rasterize_rect(position, size, window_size);
        
        // Render
        for pos in raster {
            let existing_z = depth_buffer.get(pos);
            if z < existing_z {
                continue;
            }

            let Vector2I(x, y) = pos;
            let local_pos = Vector2I(x - position.0, y - position.1);

            if let Some(color) = (*shader)(RasterInput::new(local_pos, size)) {
                framebuffer.set(pos, color);
                depth_buffer.set(pos, z);
            }
        }
    }
}
