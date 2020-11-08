use std::cell::Ref;

use store::StoreQuery;

use crate::{
    components::{
        StringShader, Framebuffer, GlobalPosition, GlobalZIndex, Position, SoftwareRasterFramebuffer,
    },
    entity_component_system::{ComponentStore, EntityID},
    primitive_types::Vector2I,
};

use super::SoftwareRasterRenderer;

const TAB_WIDTH: i64 = 4;

type ReadStringControlEntity<'a> = (
    EntityID,
    Ref<'a, Position>,
    Option<Ref<'a, GlobalPosition>>,
    Option<Ref<'a, GlobalZIndex>>,
    Option<Ref<'a, char>>,
    Option<Ref<'a, String>>,
);

#[derive(Debug)]
pub struct StringRenderer;

impl StringRenderer {
    fn render_string(
        framebuffer: &mut SoftwareRasterFramebuffer<char>,
        depth_buffer: &mut SoftwareRasterFramebuffer<i64>,
        position: Vector2I,
        z: i64,
        string: &str,
    ) {
        let Vector2I(x, mut y) = position;
        let Vector2I(window_width, window_height) = framebuffer.get_size();

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
                    let rx = new_x + x;
                    let ry = y;
                    let pos = Vector2I(rx, ry);
                    let existing_z = depth_buffer.get(pos);
                    if z < existing_z {
                        continue;
                    }

                    framebuffer.set(pos, char);
                    depth_buffer.set(pos, z);
                    x += 1;
                }
            }
        }
    }
}

impl SoftwareRasterRenderer for StringRenderer {
    type Output = char;

    fn gather_entities(&self, db: &ComponentStore) -> Vec<EntityID> {
        StoreQuery::<(EntityID, Ref<StringShader>, Ref<char>)>::iter(db.as_ref())
            .map(|(entity_id, _, _)| entity_id)
            .chain(
                StoreQuery::<(EntityID, Ref<StringShader>, Ref<String>)>::iter(db.as_ref())
                    .map(|(entity_id, _, _)| entity_id),
            )
            .collect()
    }

    fn render(
        &self,
        db: &ComponentStore,
        framebuffer: &mut SoftwareRasterFramebuffer<char>,
        depth_buffer: &mut SoftwareRasterFramebuffer<i64>,
        entity_id: EntityID,
    ) {
        let (_, position, global_position, global_z, char, string) =
            StoreQuery::<ReadStringControlEntity>::get(db.as_ref(), &entity_id);

        // Get Position
        let Vector2I(x, y) = if let Some(global_position) = global_position {
            **global_position
        } else {
            **position
        };

        // Get Z Index
        let z = if let Some(global_z) = global_z {
            **global_z
        } else {
            0
        };

        let string = if let Some(string) = string {
            (*string).clone()
        } else if let Some(char) = char {
            (*char).to_string()
        } else {
            panic!("No valid string component");
        };

        for (i, string) in string.split('\n').enumerate() {
            Self::render_string(
                &mut *framebuffer,
                &mut *depth_buffer,
                Vector2I(x, y + i as i64),
                z,
                string,
            )
        }
    }
}
