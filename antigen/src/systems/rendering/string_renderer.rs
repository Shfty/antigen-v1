use crate::components::{Control, SoftwareFramebuffer};
use crate::{
    components::{ChildEntitiesData, GlobalPositionData, Position, Size, Window, ZIndex},
    entity_component_system::{
        system_interface::SystemInterface, ComponentStorage, EntityComponentDirectory, EntityID,
        SystemError, SystemTrait,
    },
    primitive_types::Vector2I,
};

const TAB_WIDTH: i64 = 4;

#[derive(Debug)]
pub struct StringRenderer;

impl StringRenderer {
    fn render_string(
        framebuffer: &mut SoftwareFramebuffer<char>,
        window_size: Vector2I,
        position: Vector2I,
        string: &str,
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

        let mut x = 0i64;
        for char in new_str.chars() {
            if x >= window_width || y >= window_height {
                break;
            }

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
                    framebuffer.draw(new_x + x, y, window_width, char, z);
                    x += 1;
                }
            }
        }
    }
}

impl<CS, CD> SystemTrait<CS, CD> for StringRenderer
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
                    .entity_has_component::<Window>(entity_id)
                    && db
                        .entity_component_directory
                        .entity_has_component::<Size>(entity_id)
            })
            .ok_or("No window entity")?;

        let window_width: i64;
        let window_height: i64;
        {
            let size_component = db.get_entity_component::<Size>(window_entity)?;
            let Vector2I(width, height) = **size_component;

            window_width = width;
            window_height = height;
        }

        // Fetch string framebuffer entity
        let string_framebuffer_entity = db
            .entity_component_directory
            .get_entity_by_predicate(|entity_id| {
                db.entity_component_directory
                    .entity_has_component::<SoftwareFramebuffer<char>>(entity_id)
            })
            .unwrap_or_else(|| panic!("No string framebuffer component"));

        let cell_count = (window_width * window_height) as usize;
        db.get_entity_component_mut::<SoftwareFramebuffer<char>>(string_framebuffer_entity)?
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
                .entity_has_component::<Control>(&entity_id)
                && (db
                    .entity_component_directory
                    .entity_has_component::<String>(&entity_id)
                    || db
                        .entity_component_directory
                        .entity_has_component::<char>(&entity_id))
            {
                z_index = match db.get_entity_component::<ZIndex>(entity_id) {
                    Ok(z_index) => **z_index,
                    Err(_) => z_index,
                };

                z_layers.push((entity_id, z_index));
            }

            if let Ok(child_entities) =
                db.get_entity_component::<ChildEntitiesData>(entity_id)
            {
                for child_id in child_entities.iter() {
                    populate_control_entities(db, *child_id, z_layers, z_index)?;
                }
            }

            Ok(())
        };

        populate_control_entities(db, window_entity, &mut control_entities, 0)?;
        control_entities.sort();

        // Render Entities
        db.get_entity_component_mut::<SoftwareFramebuffer<char>>(string_framebuffer_entity)?
            .clear();

        for (entity_id, z) in control_entities {
            // Get Position
            let Vector2I(x, y) = if let Ok(global_position) =
                db.get_entity_component::<GlobalPositionData>(entity_id)
            {
                **global_position
            } else {
                match db.get_entity_component::<Position>(entity_id) {
                    Ok(position) => **position,
                    Err(err) => return Err(err.into()),
                }
            };

            // Get String
            let string = if let Ok(string) = db.get_entity_component::<String>(entity_id) {
                string.clone()
            } else if let Ok(char) = db.get_entity_component::<char>(entity_id) {
                char.to_string()
            } else {
                return Err("No valid string component".into());
            };

            for (i, string) in string.split('\n').enumerate() {
                Self::render_string(
                    db.get_entity_component_mut::<SoftwareFramebuffer<char>>(
                        string_framebuffer_entity,
                    )?,
                    Vector2I(window_width, window_height),
                    Vector2I(x, y + i as i64),
                    string,
                    z,
                )
            }
        }

        Ok(())
    }
}
