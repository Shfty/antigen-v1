use std::cell::{Ref, RefMut};

use store::StoreQuery;

use crate::components::{Control, SoftwareFramebuffer};
use crate::{
    components::{ChildEntitiesData, GlobalPositionData, Position, Size, Window, ZIndex},
    entity_component_system::{
        ComponentStore, EntityID, SystemError, SystemTrait,
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

impl SystemTrait for StringRenderer {
    fn run(&mut self, db: &mut ComponentStore) -> Result<(), SystemError> {
        let (window_entity_id, _window, size) =
            StoreQuery::<(EntityID, Ref<Window>, Ref<Size>)>::iter(db.as_ref())
                .next()
                .expect("No window entity");

        let window_width: i64 = (**size).0;
        let window_height: i64 = (**size).1;

        let (_, mut string_framebuffer) =
            StoreQuery::<(EntityID, RefMut<SoftwareFramebuffer<char>>)>::iter(db.as_ref())
                .next()
                .expect("No CPU framebuffer entity");

        // Fetch color buffer entity
        let cell_count = (window_width * window_height) as usize;
        string_framebuffer.resize(cell_count);

        // Recursively traverse parent-child tree and populate Z-ordered list of controls
        let mut control_entities: Vec<(EntityID, i64)> = Vec::new();

        fn populate_control_entities(
            db: &ComponentStore,
            entity_id: EntityID,
            z_layers: &mut Vec<(EntityID, i64)>,
            mut entity_z: i64,
        ) -> Result<(), String> {
            let (_, control, char, string, z_index) =
                StoreQuery::<(
                    EntityID,
                    Option<Ref<Control>>,
                    Option<Ref<char>>,
                    Option<Ref<String>>,
                    Option<Ref<ZIndex>>,
                )>::get(db.as_ref(), &entity_id);

            if control.is_some() && (char.is_some() || string.is_some()) {
                entity_z = if let Some(z_index) = z_index {
                    **z_index
                } else {
                    entity_z
                };

                z_layers.push((entity_id, entity_z));
            }

            let (_, child_entities) = StoreQuery::<(EntityID, Option<Ref<ChildEntitiesData>>)>::get(
                db.as_ref(),
                &entity_id,
            );

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
        string_framebuffer.clear();

        for (entity_id, z) in control_entities {
            let (_, position, global_position, char, string) =
                StoreQuery::<(
                    EntityID,
                    Ref<Position>,
                    Option<Ref<GlobalPositionData>>,
                    Option<Ref<char>>,
                    Option<Ref<String>>,
                )>::get(db.as_ref(), &entity_id);

            // Get Position
            let Vector2I(x, y) = if let Some(global_position) = global_position {
                **global_position
            } else {
                **position
            };

            let string = if let Some(string) = string {
                (*string).clone()
            } else if let Some(char) = char {
                (*char).to_string()
            } else {
                return Err("No valid string component".into());
            };

            for (i, string) in string.split('\n').enumerate() {
                Self::render_string(
                    &mut *string_framebuffer,
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
