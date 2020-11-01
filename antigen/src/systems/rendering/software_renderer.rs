use std::cell::{Ref, RefMut};

use store::StoreQuery;

use crate::components::{Control, SoftwareFramebuffer};
use crate::{
    components::{CPUShader, CPUShaderInput, GlobalPositionData, Position, Size},
    entity_component_system::{ComponentStore, EntityID},
    primitive_types::ColorRGB,
    primitive_types::ColorRGBF,
    primitive_types::Vector2I,
};

use super::Renderer;

type ReadControlTransform<'a> = (
    EntityID,
    Option<Ref<'a, Control>>,
    Option<Ref<'a, Size>>,
);
type ReadControlEntity<'a> = (
    EntityID,
    Ref<'a, Position>,
    Ref<'a, Size>,
    Option<Ref<'a, GlobalPositionData>>,
    Option<Ref<'a, ColorRGBF>>,
    Option<Ref<'a, CPUShader>>,
);

#[derive(Debug)]
pub struct SoftwareRenderer;

impl SoftwareRenderer {
    fn render_rect(
        framebuffer: &mut SoftwareFramebuffer<ColorRGBF>,
        window_size: Vector2I,
        position: Vector2I,
        size: Vector2I,
        color: ColorRGBF,
        color_shader: CPUShader,
        z: i64,
    ) {
        let Vector2I(width, height) = size;

        let Vector2I(window_width, window_height) = window_size;
        let Vector2I(pos_x, pos_y) = position;

        let min_x = std::cmp::max(pos_x, 0);
        let max_x = std::cmp::min(pos_x + width, window_width);

        let min_y = std::cmp::max(pos_y, 0);
        let max_y = std::cmp::min(pos_y + height, window_height);

        let grid_iter = (min_y..max_y).flat_map(move |y| {
            (min_x..max_x).map(move |x| {
                let world_pos = Vector2I(x, y);
                let local_pos = Vector2I(x - pos_x, y - pos_y);
                (world_pos, local_pos)
            })
        });

        for (Vector2I(rx, ry), local_pos) in grid_iter {
            let CPUShader(color_shader) = color_shader;
            if let Some(color) = color_shader(CPUShaderInput::new(local_pos, size, color)) {
                framebuffer.draw(rx, ry, window_width, color, z);
            }
        }
    }
}

impl Renderer for SoftwareRenderer {
    type Data = ColorRGBF;

    fn entity_predicate(db: &ComponentStore, entity_id: EntityID) -> bool {
        let (_, control, size) =
            StoreQuery::<ReadControlTransform>::get(db.as_ref(), &entity_id);

        control.is_some() && size.is_some()
    }

    fn render(
        &self,
        db: &ComponentStore,
        framebuffer: &mut RefMut<SoftwareFramebuffer<ColorRGBF>>,
        window_size: Vector2I,
        entity_id: EntityID,
        z: i64,
    ) {
        let (_, position, size, global_position, color, shader) =
            StoreQuery::<ReadControlEntity>::get(db.as_ref(), &entity_id);

        // Get Position
        let position = if let Some(global_position) = global_position {
            **global_position
        } else {
            **position
        };

        // Get Size
        let size = **size;

        // Get Color
        let color: ColorRGB<f32> = if let Some(color) = color {
            *color
        } else {
            ColorRGB(1.0, 1.0, 1.0)
        };

        // Get shader
        let shader = if let Some(shader) = shader {
            *shader
        } else {
            CPUShader(CPUShader::color_passthrough)
        };

        Self::render_rect(
            &mut *framebuffer,
            window_size,
            position,
            size,
            color,
            shader,
            z,
        );
    }
}
