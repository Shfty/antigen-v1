use std::cell::{Ref, RefMut};

use store::StoreQuery;

use crate::components::{Control, SoftwareFramebuffer};
use crate::{
    components::{GlobalPositionData, Position},
    entity_component_system::{ComponentStore, EntityID},
    primitive_types::Vector2I,
};

use super::Renderer;

const TAB_WIDTH: i64 = 4;

type ReadStringControlTransforms<'a> = (
    EntityID,
    Option<Ref<'a, Control>>,
    Option<Ref<'a, char>>,
    Option<Ref<'a, String>>,
);
type ReadStringControlEntities<'a> = (
    EntityID,
    Ref<'a, Position>,
    Option<Ref<'a, GlobalPositionData>>,
    Option<Ref<'a, char>>,
    Option<Ref<'a, String>>,
);

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

impl Renderer for StringRenderer {
    type Data = char;

    fn entity_predicate(db: &ComponentStore, entity_id: EntityID) -> bool {
        let (_, control, char, string) =
            StoreQuery::<ReadStringControlTransforms>::get(db.as_ref(), &entity_id);

        control.is_some() && (char.is_some() || string.is_some())
    }

    fn render(
        &self,
        db: &ComponentStore,
        framebuffer: &mut RefMut<SoftwareFramebuffer<Self::Data>>,
        window_size: Vector2I,
        entity_id: EntityID,
        z: i64,
    ) {
        let (_, position, global_position, char, string) =
            StoreQuery::<ReadStringControlEntities>::get(db.as_ref(), &entity_id);

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
            panic!("No valid string component");
        };

        for (i, string) in string.split('\n').enumerate() {
            Self::render_string(
                &mut *framebuffer,
                window_size,
                Vector2I(x, y + i as i64),
                string,
                z,
            )
        }
    }
}
