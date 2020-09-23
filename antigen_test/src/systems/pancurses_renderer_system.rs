use crate::{
    components::pancurses_color_set_component::PancursesColorSetComponent,
    components::{
        control_component::ControlComponent, pancurses_window_component::PancursesWindowComponent,
    },
    cpu_shader::CPUShader,
    cpu_shader::CPUShaderComponent,
    cpu_shader::CPUShaderInput,
};
use antigen::{
    components::ColorComponent,
    components::{
        CharComponent, ChildEntitiesComponent, GlobalPositionComponent, PositionComponent,
        SizeComponent, StringComponent, WindowComponent, ZIndexComponent,
    },
    entity_component_system::SystemDebugTrait,
    entity_component_system::{
        system_interface::SystemInterface, ComponentStorage, EntityComponentDirectory, EntityID,
        SystemError, SystemTrait,
    },
    primitive_types::ColorRGB,
    primitive_types::Vector2I,
};
use pancurses::ToChtype;

const TAB_WIDTH: i32 = 4;

#[derive(Debug)]
pub struct PancursesRendererSystem {
    color_buffer: Vec<(ColorRGB, Option<i64>)>,
    char_buffer: Vec<(char, ColorRGB, Option<i64>)>,
}

impl PancursesRendererSystem {
    pub fn new() -> PancursesRendererSystem {
        PancursesRendererSystem {
            color_buffer: Vec::new(),
            char_buffer: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.color_buffer
            .iter_mut()
            .for_each(|color| *color = (ColorRGB(0.0, 0.0, 0.0), None));

        self.char_buffer
            .iter_mut()
            .for_each(|char| *char = (' ', ColorRGB(1.0, 1.0, 1.0), None));
    }

    pub fn draw_color(&mut self, x: i64, y: i64, window_width: i64, color: ColorRGB, z: i64) {
        let idx = y * window_width + x;
        let (existing_color, existing_z) = self.color_buffer[idx as usize];

        if let Some(existing_z) = existing_z {
            if existing_z > z {
                return;
            }
        }

        if color == existing_color {
            return;
        }

        self.color_buffer[idx as usize] = (color, Some(z));
    }

    pub fn draw_char(
        &mut self,
        x: i64,
        y: i64,
        window_width: i64,
        char: char,
        color: ColorRGB,
        z: i64,
    ) {
        let idx = y * window_width + x;
        let (existing_char, existing_color, existing_z) = self.char_buffer[idx as usize];

        if let Some(existing_z) = existing_z {
            if existing_z > z {
                return;
            }
        }

        if char == existing_char && color == existing_color {
            return;
        }

        self.char_buffer[idx as usize] = (char, color, Some(z));
    }

    fn render_string(
        &mut self,
        window_size: Vector2I,
        position: Vector2I,
        string: &str,
        color: ColorRGB,
        z: i64,
    ) {
        let Vector2I(window_width, window_height) = window_size;
        let Vector2I(x, mut y) = position;

        let len = string.len() as i64;

        let mut new_x = x;
        let mut new_str = string.to_string();
        if x < -len {
            new_str.clear();
        } else if x < 0 {
            new_x = 0;
            new_str = string[(len - (len + x)) as usize..].into();
        }

        if new_x > window_width {
            new_str.clear();
        } else if new_x > window_width - new_str.len() as i64 {
            new_str = new_str[..(window_width - new_x) as usize].into();
        }

        let len = new_str.len() as i64;
        if len <= 0 || y < 0 || y >= window_height {
            return;
        }

        let mut x = 0;
        for char in new_str.chars() {
            match char {
                '\0' => continue,
                '\n' => {
                    x = 0;
                    y += 1;
                }
                '\t' => {
                    x += TAB_WIDTH - (x % TAB_WIDTH);
                }
                _ => {
                    self.draw_char(new_x + x as i64, y, window_width, char, color, z);
                    x += 1;
                }
            }
        }
    }

    fn render_rect(
        &mut self,
        window_size: Vector2I,
        position: Vector2I,
        size: Vector2I,
        color: ColorRGB,
        char: char,
        color_shader: CPUShader,
        z: i64,
    ) {
        let Vector2I(width, height) = size;
        if width == 0 || height == 0 {
            return;
        }

        let Vector2I(window_width, window_height) = window_size;
        let Vector2I(pos_x, pos_y) = position;

        let char = match char {
            '\0' => ' ',
            '\n' => ' ',
            '\t' => ' ',
            _ => char,
        };

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
                    self.draw_color(rx, ry, window_width, color, z);
                }

                if char != ' ' {
                    self.draw_char(rx, ry, window_width, char, ColorRGB(1.0, 1.0, 1.0), z);
                }
            }
        }
    }
}

impl<CS, CD> SystemTrait<CS, CD> for PancursesRendererSystem
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
{
    fn run(&mut self, db: &mut SystemInterface<CS, CD>) -> Result<(), SystemError>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
    {
        // Fetch color set entity
        let color_set_entity = db
            .entity_component_directory
            .get_entity_by_predicate(|entity_id| {
                db.entity_component_directory
                    .entity_has_component::<PancursesColorSetComponent>(entity_id)
            });
        let color_set_entity = color_set_entity.expect("Color set entity does not exist");

        // Fetch window entity
        let window_entity = db
            .entity_component_directory
            .get_entity_by_predicate(|entity_id| {
                db.entity_component_directory
                    .entity_has_component::<WindowComponent>(entity_id)
                    && db
                        .entity_component_directory
                        .entity_has_component::<PancursesWindowComponent>(entity_id)
                    && db
                        .entity_component_directory
                        .entity_has_component::<SizeComponent>(entity_id)
            })
            .ok_or("No window entity")?;

        let window_width: i64;
        let window_height: i64;
        {
            let window_component =
                db.get_entity_component::<PancursesWindowComponent>(window_entity)?;

            let window = window_component
                .get_window()
                .ok_or("Error fetching window handle")?;

            let (height, width) = window.get_max_yx();

            window_width = width as i64;
            window_height = height as i64;
        }

        let cell_count = (window_width * window_height) as usize;
        if self.color_buffer.len() != cell_count {
            self.color_buffer
                .resize(cell_count, (ColorRGB(0.0, 0.0, 0.0), None));
        }
        if self.char_buffer.len() != cell_count {
            self.char_buffer
                .resize(cell_count, (' ', ColorRGB(1.0, 1.0, 1.0), None));
        }

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
                .get_entity_component::<ControlComponent>(entity_id)
                .is_ok()
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
        self.clear();

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
                Err(_) => ColorRGB(1.0, 1.0, 1.0),
            };

            // Get char
            let char = match db.get_entity_component::<CharComponent>(entity_id) {
                Ok(char_component) => *char_component.get_data(),
                Err(_) => ' ',
            };

            // Get shader
            let shader = match db.get_entity_component::<CPUShaderComponent>(entity_id) {
                Ok(cpu_shader_component) => *cpu_shader_component.get_data(),
                Err(_) => CPUShader(CPUShader::color_passthrough),
            };

            if db
                .entity_component_directory
                .entity_has_component::<SizeComponent>(&entity_id)
            {
                // Get size
                let Vector2I(width, height) = db
                    .get_entity_component::<SizeComponent>(entity_id)?
                    .get_size();

                self.render_rect(
                    Vector2I(window_width, window_height),
                    Vector2I(x, y),
                    Vector2I(width, height),
                    color,
                    char,
                    shader,
                    z,
                );
            } else if db
                .entity_component_directory
                .entity_has_component::<StringComponent>(&entity_id)
                || db
                    .entity_component_directory
                    .entity_has_component::<CharComponent>(&entity_id)
            {
                // Get string
                let string = if let Ok(string_component) =
                    db.get_entity_component::<StringComponent>(entity_id)
                {
                    string_component.get_data().clone()
                } else if let Ok(char_component) =
                    db.get_entity_component::<CharComponent>(entity_id)
                {
                    char_component.get_data().to_string()
                } else {
                    return Err("No valid string component".into());
                };

                for (i, string) in string.split('\n').enumerate() {
                    self.render_string(
                        Vector2I(window_width, window_height),
                        Vector2I(x, y + i as i64),
                        string,
                        color,
                        z,
                    )
                }
            }
        }

        let color_set_component = db
            .get_entity_component_mut::<PancursesColorSetComponent>(color_set_entity)
            .unwrap();

        let mut cells: Vec<(i32, i32, char, i16)> = Vec::new();
        let window_width = window_width as i32;
        let window_height = window_height as i32;
        for y in 0..window_height as i32 {
            for x in 0..window_width as i32 {
                let idx = (y * window_width + x) as usize;
                let (color, color_z) = self.color_buffer[idx];
                let (char, char_color, char_z) = self.char_buffer[idx];

                if color_z.is_none() && char_z.is_none() {
                    continue;
                }

                let (char, color_pair) = match char_z.cmp(&color_z) {
                    std::cmp::Ordering::Less => {
                        let color_pair_idx =
                            color_set_component.get_color_pair_idx(ColorRGB(1.0, 1.0, 1.0), color);
                        (' ', color_pair_idx)
                    }
                    std::cmp::Ordering::Equal => {
                        let color_pair_idx =
                            color_set_component.get_color_pair_idx(char_color, color);
                        (char, color_pair_idx)
                    }
                    std::cmp::Ordering::Greater => {
                        let color_pair_idx = color_set_component
                            .get_color_pair_idx(char_color, ColorRGB(0.0, 0.0, 0.0));
                        (char, color_pair_idx)
                    }
                };

                cells.push((x, y, char, color_pair));
            }
        }

        let window_component =
            db.get_entity_component::<PancursesWindowComponent>(window_entity)?;
        let window = window_component
            .get_window()
            .ok_or("Error fetching window handle")?;

        window.erase();
        for (x, y, char, color_pair) in cells {
            window.mvaddch(
                y as i32,
                x as i32,
                char.to_chtype() | pancurses::COLOR_PAIR(color_pair as u64),
            );
        }

        Ok(())
    }
}

impl SystemDebugTrait for PancursesRendererSystem {
    fn get_name() -> &'static str {
        "Pancurses Renderer"
    }
}
