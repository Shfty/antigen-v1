use std::collections::HashMap;

use crate::{
    components::fill_component::FillComponent,
    components::pancurses_color_set_component::PancursesColorSetComponent,
    components::{
        control_component::ControlComponent, pancurses_window_component::PancursesWindowComponent,
    },
    pancurses_color::PancursesColor,
    pancurses_color::PancursesColorPair,
};
use antigen::{
    components::ColorComponent,
    components::{
        CharComponent, ChildEntitiesComponent, GlobalPositionComponent, ParentEntityComponent,
        PositionComponent, SizeComponent, StringComponent, WindowComponent, ZIndexComponent,
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
    framebuffer: HashMap<Vector2I, u64>,
}

impl PancursesRendererSystem {
    pub fn new() -> PancursesRendererSystem {
        PancursesRendererSystem {
            framebuffer: HashMap::new(),
        }
    }

    fn render_string(
        &mut self,
        window_size: Vector2I,
        position: Vector2I,
        string: &str,
        color_pair: i16,
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
                    let char = char.to_chtype() | pancurses::COLOR_PAIR(color_pair as u64);
                    self.framebuffer.insert(Vector2I(new_x + x as i64, y), char);
                    x += 1;
                }
            }
        }
    }

    fn render_rect_filled(
        &mut self,
        window_size: Vector2I,
        position: Vector2I,
        size: Vector2I,
        char: char,
        color_pair: i16,
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

        let pancurses_char = char.to_chtype() | pancurses::COLOR_PAIR(color_pair as u64);

        for ry in pos_y..pos_y + height {
            for rx in pos_x..pos_x + width {
                if rx < 0 || ry < 0 {
                    continue;
                }

                if rx >= window_width || ry >= window_height {
                    continue;
                }

                self.framebuffer.insert(Vector2I(rx, ry), pancurses_char);
            }
        }
    }

    fn render_rect(
        &mut self,
        window_size: Vector2I,
        position: Vector2I,
        size: Vector2I,
        char: char,
        color_pair: i16,
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

        let pancurses_char = char.to_chtype() | pancurses::COLOR_PAIR(color_pair as u64);

        for rx in pos_x..pos_x + width {
            if rx < 0 || rx >= window_width {
                continue;
            }

            self.framebuffer.insert(Vector2I(rx, pos_y), pancurses_char);
            self.framebuffer
                .insert(Vector2I(rx, pos_y + height - 1), pancurses_char);
        }

        for ry in pos_y..pos_y + height {
            if ry < 0 || ry >= window_height {
                continue;
            }

            self.framebuffer.insert(Vector2I(pos_x, ry), pancurses_char);
            self.framebuffer
                .insert(Vector2I(pos_x + width - 1, ry), pancurses_char);
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

        // Recursively traverse parent-child tree and populate Z-ordered list of controls
        let mut z_layers: HashMap<i64, Vec<EntityID>> = HashMap::new();

        fn populate_z_layers<CS, CD>(
            db: &SystemInterface<CS, CD>,
            entity_id: EntityID,
            z_layers: &mut HashMap<i64, Vec<EntityID>>,
            mut z_index: i64,
        ) -> Result<(), String>
        where
            CS: ComponentStorage,
            CD: EntityComponentDirectory,
        {
            let z_layer = match z_layers.get_mut(&z_index) {
                Some(z_layer) => z_layer,
                None => {
                    z_layers.insert(z_index, Vec::new());
                    match z_layers.get_mut(&z_index) {
                        Some(z_layer) => z_layer,
                        None => return Err(format!("Failed to get Z layer {}", z_index)),
                    }
                }
            };

            if db
                .get_entity_component::<ControlComponent>(entity_id)
                .is_ok()
            {
                z_index = match db.get_entity_component::<ZIndexComponent>(entity_id) {
                    Ok(z_index_component) => z_index_component.get_z(),
                    Err(_) => z_index,
                };

                z_layer.push(entity_id);
            }

            if let Ok(child_entities_component) =
                db.get_entity_component::<ChildEntitiesComponent>(entity_id)
            {
                for child_id in child_entities_component.get_child_ids() {
                    populate_z_layers(db, *child_id, z_layers, z_index)?;
                }
            }

            Ok(())
        };

        populate_z_layers(db, window_entity, &mut z_layers, 0)?;

        let mut control_entities: Vec<EntityID> = Vec::new();
        let mut layer_keys: Vec<i64> = z_layers.keys().copied().collect();
        layer_keys.sort();
        for i in layer_keys {
            let z_layer = match z_layers.get_mut(&i) {
                Some(z_layer) => z_layer,
                None => return Err(format!("Failed to get Z layer {}", i).into()),
            };
            control_entities.append(z_layer);
        }

        // Render Entities
        self.framebuffer.clear();

        for entity_id in control_entities {
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

            if db
                .entity_component_directory
                .entity_has_component::<SizeComponent>(&entity_id)
            {
                // Get color pair index
                let rect_color_pair_idx = db
                    .get_entity_component_mut::<PancursesColorSetComponent>(color_set_entity)?
                    .get_color_pair_idx(&PancursesColorPair::new(
                        PancursesColor::new(0, 0, 0),
                        color.into(),
                    ));

                // Get size
                let Vector2I(width, height) = db
                    .get_entity_component::<SizeComponent>(entity_id)?
                    .get_size();

                // Get filled
                let filled = db
                    .entity_component_directory
                    .entity_has_component::<FillComponent>(&entity_id);

                if filled {
                    self.render_rect_filled(
                        Vector2I(window_width, window_height),
                        Vector2I(x, y),
                        Vector2I(width, height),
                        char,
                        rect_color_pair_idx,
                    );
                } else {
                    self.render_rect(
                        Vector2I(window_width, window_height),
                        Vector2I(x, y),
                        Vector2I(width, height),
                        char,
                        rect_color_pair_idx,
                    );
                }
            } else if db
                .entity_component_directory
                .entity_has_component::<StringComponent>(&entity_id)
                || db
                    .entity_component_directory
                    .entity_has_component::<CharComponent>(&entity_id)
            {
                // Get color pair index
                let string_color_pair_idx = db
                    .get_entity_component_mut::<PancursesColorSetComponent>(color_set_entity)?
                    .get_color_pair_idx(&PancursesColorPair::new(
                        color.into(),
                        PancursesColor::new(0, 0, 0),
                    ));

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
                        string_color_pair_idx,
                    )
                }
            }
        }

        let window_component =
            db.get_entity_component::<PancursesWindowComponent>(window_entity)?;
        let window = window_component
            .get_window()
            .ok_or("Error fetching window handle")?;

        window.erase();
        for (Vector2I(x, y), char) in &self.framebuffer {
            window.mvaddch(*y as i32, *x as i32, *char);
        }

        Ok(())
    }
}

impl SystemDebugTrait for PancursesRendererSystem {
    fn get_name() -> &'static str {
        "Pancurses Renderer"
    }
}
