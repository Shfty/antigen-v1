use std::cell::{Ref, RefMut};

use store::StoreQuery;

use crate::components::{Control, SoftwareFramebuffer};
use crate::{
    components::{
        CPUShader, CPUShaderInput, ChildEntitiesData, GlobalPositionData, Position, Size, Window,
        ZIndex,
    },
    entity_component_system::{
        system_interface::SystemInterface, EntityComponentDirectory, EntityID, SystemError,
        SystemTrait,
    },
    primitive_types::ColorRGB,
    primitive_types::ColorRGBF,
    primitive_types::Vector2I,
};

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

        let x_range = min_x..max_x;
        let y_range = min_y..max_y;
        for ry in y_range {
            for rx in x_range.clone() {
                let local_pos = Vector2I(rx - pos_x, ry - pos_y);
                let CPUShader(color_shader) = color_shader;
                if let Some(color) = color_shader(CPUShaderInput::new(local_pos, size, color)) {
                    framebuffer.draw(rx, ry, window_width, color, z);
                }
            }
        }
    }
}

impl<CD> SystemTrait<CD> for SoftwareRenderer
where
    CD: EntityComponentDirectory,
{
    fn run(&mut self, db: &mut SystemInterface<CD>) -> Result<(), SystemError>
    where
        CD: EntityComponentDirectory,
    {
        let (window_entity_id, _window, size) =
            StoreQuery::<(EntityID, Ref<Window>, Ref<Size>)>::iter(db.component_store)
                .next()
                .expect("No window entity");

        let window_width: i64 = (**size).0;
        let window_height: i64 = (**size).1;

        let (_, mut software_framebuffer) = StoreQuery::<(
            EntityID,
            RefMut<SoftwareFramebuffer<ColorRGBF>>,
        )>::iter(db.component_store)
        .next()
        .expect("No CPU framebuffer entity");

        // Fetch color buffer entity
        let cell_count = (window_width * window_height) as usize;
        software_framebuffer.resize(cell_count);

        // Recursively traverse parent-child tree and populate Z-ordered list of controls
        let mut control_entities: Vec<(EntityID, i64)> = Vec::new();

        fn populate_control_entities<CD>(
            db: &SystemInterface<CD>,
            entity_id: EntityID,
            z_layers: &mut Vec<(EntityID, i64)>,
            mut entity_z: i64,
        ) -> Result<(), String>
        where
            CD: EntityComponentDirectory,
        {
            let (_, control, size, z_index) = StoreQuery::<(
                EntityID,
                Option<Ref<Control>>,
                Option<Ref<Size>>,
                Option<Ref<ZIndex>>,
            )>::get(db.component_store, &entity_id);

            if let (Some(_), Some(_)) = (control, size) {
                entity_z = if let Some(z_index) = z_index {
                    **z_index
                } else {
                    entity_z
                };

                z_layers.push((entity_id, entity_z));
            }

            let (_, child_entities) = StoreQuery::<(EntityID, Option<Ref<ChildEntitiesData>>)>::get(
                db.component_store,
                &entity_id,
            );

            if let Some(child_entities) = child_entities {
                for child_id in child_entities.iter() {
                    populate_control_entities(db, *child_id, z_layers, entity_z)?;
                }
            }

            Ok(())
        };

        populate_control_entities(db, window_entity_id, &mut control_entities, 0)?;
        control_entities.sort();

        // Render Entities
        software_framebuffer.clear();

        for (entity_id, z) in control_entities {
            let (_, position, size, global_position, color, shader) =
                StoreQuery::<(
                    EntityID,
                    Ref<Position>,
                    Ref<Size>,
                    Option<Ref<GlobalPositionData>>,
                    Option<Ref<ColorRGBF>>,
                    Option<Ref<CPUShader>>,
                )>::get(db.component_store, &entity_id);

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
