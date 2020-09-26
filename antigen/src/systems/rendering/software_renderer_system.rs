use crate::{
    components::ColorComponent,
    components::{
        CPUShaderComponent, ChildEntitiesComponent, GlobalPositionComponent, PositionComponent,
        SizeComponent, WindowComponent, ZIndexComponent,
    },
    entity_component_system::SystemDebugTrait,
    entity_component_system::{
        system_interface::SystemInterface, ComponentStorage, EntityComponentDirectory, EntityID,
        SystemError, SystemTrait,
    },
    primitive_types::Color,
    primitive_types::ColorRGBF,
    primitive_types::Vector2I,
};
use crate::{
    components::{ControlComponent, SoftwareFramebufferComponent},
    cpu_shader::{CPUShader, CPUShaderInput},
};

#[derive(Debug)]
pub struct SoftwareRendererSystem;

impl SoftwareRendererSystem {
    fn render_rect(
        framebuffer: &mut SoftwareFramebufferComponent<ColorRGBF>,
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

impl<CS, CD> SystemTrait<CS, CD> for SoftwareRendererSystem
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
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
                    .entity_has_component::<WindowComponent>(entity_id)
                    && db
                        .entity_component_directory
                        .entity_has_component::<SizeComponent>(entity_id)
            })
            .ok_or("No window entity")?;

        let window_width: i64;
        let window_height: i64;
        {
            let size_component = db.get_entity_component::<SizeComponent>(window_entity)?;

            let Vector2I(width, height) = size_component.get_size();

            window_width = width as i64;
            window_height = height as i64;
        }

        // Fetch color buffer entity
        let cpu_framebuffer_entity = db
            .entity_component_directory
            .get_entity_by_predicate(|entity_id| {
                db.entity_component_directory
                    .entity_has_component::<SoftwareFramebufferComponent<ColorRGBF>>(entity_id)
            })
            .unwrap_or_else(|| panic!("No CPU framebuffer component"));

        let cell_count = (window_width * window_height) as usize;
        db.get_entity_component_mut::<SoftwareFramebufferComponent<ColorRGBF>>(
            cpu_framebuffer_entity,
        )?
        .resize(cell_count);

        // Recursively traverse parent-child tree and populate Z-ordered list of controls
        let mut control_entities: Vec<(EntityID, i64)> = Vec::new();

        fn populate_control_entities<CS, CD>(
            db: &SystemInterface<CS, CD>,
            entity_id: EntityID,
            z_layers: &mut Vec<(EntityID, i64)>,
            mut z_index: i64,
        ) -> Result<(), String>
        where
            CS: ComponentStorage,
            CD: EntityComponentDirectory,
        {
            if db
                .entity_component_directory
                .entity_has_component::<ControlComponent>(&entity_id)
                && db
                    .entity_component_directory
                    .entity_has_component::<SizeComponent>(&entity_id)
            {
                z_index = match db.get_entity_component::<ZIndexComponent>(entity_id) {
                    Ok(z_index_component) => z_index_component.get_z(),
                    Err(_) => z_index,
                };

                z_layers.push((entity_id, z_index));
            }

            if let Ok(child_entities_component) =
                db.get_entity_component::<ChildEntitiesComponent>(entity_id)
            {
                for child_id in child_entities_component.get_child_ids() {
                    populate_control_entities(db, *child_id, z_layers, z_index)?;
                }
            }

            Ok(())
        };

        populate_control_entities(db, window_entity, &mut control_entities, 0)?;
        control_entities.sort();

        // Render Entities
        db.get_entity_component_mut::<SoftwareFramebufferComponent<ColorRGBF>>(
            cpu_framebuffer_entity,
        )?
        .clear();

        for (entity_id, z) in control_entities {
            // Get Position
            let Vector2I(x, y) = if let Ok(global_position_component) =
                db.get_entity_component::<GlobalPositionComponent>(entity_id)
            {
                global_position_component.get_global_position()
            } else {
                match db.get_entity_component::<PositionComponent>(entity_id) {
                    Ok(position_component) => position_component.get_position(),
                    Err(err) => return Err(err.into()),
                }
            };

            // Get Color
            let color = match db.get_entity_component::<ColorComponent>(entity_id) {
                Ok(color_component) => *color_component.get_data(),
                Err(_) => Color(1.0, 1.0, 1.0),
            };

            // Get shader
            let shader = match db.get_entity_component::<CPUShaderComponent>(entity_id) {
                Ok(cpu_shader_component) => *cpu_shader_component.get_data(),
                Err(_) => CPUShader(CPUShader::color_passthrough),
            };

            // Get size
            let Vector2I(width, height) = db
                .get_entity_component::<SizeComponent>(entity_id)?
                .get_size();

            Self::render_rect(
                db.get_entity_component_mut::<SoftwareFramebufferComponent<ColorRGBF>>(
                    cpu_framebuffer_entity,
                )?,
                Vector2I(window_width, window_height),
                Vector2I(x, y),
                Vector2I(width, height),
                color,
                shader,
                z,
            );
        }

        Ok(())
    }
}

impl SystemDebugTrait for SoftwareRendererSystem {
    fn get_name() -> &'static str {
        "CPU Renderer"
    }
}
