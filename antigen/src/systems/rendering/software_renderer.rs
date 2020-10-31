use std::cell::{Ref, RefMut};

use store::StoreQuery;

use crate::components::{Control, SoftwareFramebuffer};
use crate::{
    components::{
        CPUShader, CPUShaderInput, ChildEntitiesData, GlobalPositionData, Position, Size, Window,
        ZIndex,
    },
    entity_component_system::{ComponentStore, EntityID, SystemError, SystemTrait},
    primitive_types::ColorRGB,
    primitive_types::ColorRGBF,
    primitive_types::Vector2I,
};

type ReadWindowEntity<'a> = (EntityID, Ref<'a, Window>, Ref<'a, Size>);
type WriteSoftwareFramebuffer<'a> = (EntityID, RefMut<'a, SoftwareFramebuffer<ColorRGBF>>);
type ReadControlTransforms<'a> = (
    EntityID,
    Option<Ref<'a, Control>>,
    Option<Ref<'a, Size>>,
    Option<Ref<'a, ZIndex>>,
);
type ReadChildEntities<'a> = (EntityID, Option<Ref<'a, ChildEntitiesData>>);
type ReadControlEntities<'a> = (
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
        if width == 0 || height == 0 {
            return;
        }

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

impl SystemTrait for SoftwareRenderer {
    fn run(&mut self, db: &mut ComponentStore) -> Result<(), SystemError> {
        let (window_entity_id, _window, size) = StoreQuery::<ReadWindowEntity>::iter(db.as_ref())
            .next()
            .expect("No window entity");

        let window_width: i64 = (**size).0;
        let window_height: i64 = (**size).1;

        let (_, mut software_framebuffer) =
            StoreQuery::<WriteSoftwareFramebuffer>::iter(db.as_ref())
                .next()
                .expect("No CPU framebuffer entity");

        // Fetch color buffer entity
        let cell_count = (window_width * window_height) as usize;
        software_framebuffer.resize(cell_count);

        // Recursively traverse parent-child tree and populate Z-ordered list of controls
        let mut control_entities: Vec<(EntityID, i64)> = Vec::new();

        fn populate_control_entities(
            db: &ComponentStore,
            entity_id: EntityID,
            z_layers: &mut Vec<(EntityID, i64)>,
            mut entity_z: i64,
        ) -> Result<(), String> {
            let (_, control, size, z_index) =
                StoreQuery::<ReadControlTransforms>::get(db.as_ref(), &entity_id);

            if let (Some(_), Some(_)) = (control, size) {
                entity_z = if let Some(z_index) = z_index {
                    **z_index
                } else {
                    entity_z
                };

                z_layers.push((entity_id, entity_z));
            }

            let (_, child_entities) = StoreQuery::<ReadChildEntities>::get(db.as_ref(), &entity_id);

            if let Some(child_entities) = child_entities {
                for child_id in child_entities.iter() {
                    populate_control_entities(db, *child_id, z_layers, entity_z)?;
                }
            }

            Ok(())
        };

        populate_control_entities(&db, window_entity_id, &mut control_entities, 0)?;
        control_entities.sort();

        // Render Entities
        software_framebuffer.clear();

        for (entity_id, z) in control_entities {
            let (_, position, size, global_position, color, shader) =
                StoreQuery::<ReadControlEntities>::get(db.as_ref(), &entity_id);

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
                &mut *software_framebuffer,
                Vector2I(window_width, window_height),
                position,
                size,
                color,
                shader,
                z,
            );
        }

        Ok(())
    }
}
