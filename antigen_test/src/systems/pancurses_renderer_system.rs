use crate::{
    components::fill_component::FillComponent,
    components::pancurses_color_set_component::PancursesColorSetComponent,
    components::{
        control_component::ControlComponent,
        pancurses_color_pair_component::PancursesColorPairComponent,
        pancurses_window_component::PancursesWindowComponent,
    },
    pancurses_color::PancursesColorPair,
};
use antigen::{
    components::ParentEntityComponent,
    components::WindowComponent,
    components::{
        CharComponent, GlobalPositionComponent, PositionComponent, SizeComponent, StringComponent,
    },
    ecs::EntityComponentDatabaseDebug,
    ecs::EntityID,
    ecs::{EntityComponentDatabase, SystemEvent, SystemTrait},
    primitive_types::IVector2,
};
use pancurses::{ToChtype, Window};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct PancursesRendererSystem;

impl PancursesRendererSystem {
    pub fn new() -> PancursesRendererSystem {
        PancursesRendererSystem
    }

    fn render_strings(
        &self,
        window: &Window,
        width: i64,
        height: i64,
        strings: &HashSet<RenderData>,
    ) {
        strings
            .iter()
            .flat_map(|render_data| {
                if let RenderData::String(x, y, str, color) = render_data {
                    let len = str.len() as i64;

                    let mut new_x = *x;
                    let mut new_str = str.clone();
                    if *x < -len {
                        new_str.clear();
                    } else if *x < 0 {
                        new_x = 0;
                        new_str = str[(len - (len + x)) as usize..].into();
                    }

                    if new_x > width {
                        new_str.clear();
                    } else if new_x > width - new_str.len() as i64 {
                        new_str = new_str[..(width - new_x) as usize].into();
                    }

                    return Some((new_x, *y, new_str, *color));
                }

                None
            })
            .filter(|(_, y, str, _)| {
                let len = str.len() as i64;
                len > 0 && *y >= 0 && *y < height
            })
            .for_each(|(x, y, string, color_pair)| {
                let mut y = y as i32;
                window.mv(y, x as i32);
                for char in string.chars() {
                    if char == '\n' {
                        y += 1;
                        window.mv(y, x as i32);
                    } else {
                        window.addch(char.to_chtype() | pancurses::COLOR_PAIR(color_pair as u64));
                    }
                }
            });
    }

    fn render_rects(&self, window: &Window, width: i64, height: i64, rects: &HashSet<RenderData>) {
        rects.iter().for_each(|render_data| {
            if let RenderData::Rect(x, y, w, h, char, color_pair, filled) = render_data {
                let mut w = *w;
                let width_delta = (x + w) - width;
                if width_delta > 0 {
                    w -= width_delta;
                }

                let mut h = *h;
                let height_delta = (y + h) - height;
                if height_delta > 0 {
                    h -= height_delta;
                }

                let mut x = *x;
                if x < 0 {
                    w += x;
                    x = 0;
                }

                let mut y = *y;
                if y < 0 {
                    h += y;
                    y = 0;
                }

                let char = *char;
                let color_pair = *color_pair;

                if w == 0 || h == 0 {
                    return;
                }

                if *filled {
                    if w >= h {
                        for y in y..y + h {
                            window.mv(y as i32, x as i32);
                            window.hline(
                                char.to_chtype() | pancurses::COLOR_PAIR(color_pair as u64),
                                w as i32,
                            );
                        }
                    } else {
                        for x in x..x + w {
                            window.mv(y as i32, x as i32);
                            window.vline(
                                char.to_chtype() | pancurses::COLOR_PAIR(color_pair as u64),
                                h as i32,
                            );
                        }
                    }
                } else {
                    window.mv(y as i32, x as i32);
                    window.hline(
                        char.to_chtype() | pancurses::COLOR_PAIR(color_pair as u64),
                        w as i32,
                    );

                    window.mv((y + h - 1) as i32, x as i32);
                    window.hline(
                        char.to_chtype() | pancurses::COLOR_PAIR(color_pair as u64),
                        w as i32,
                    );

                    window.mv((y + 1) as i32, x as i32);
                    window.vline(
                        char.to_chtype() | pancurses::COLOR_PAIR(color_pair as u64),
                        (h - 2) as i32,
                    );

                    window.mv((y + 1) as i32, (x + w - 1) as i32);
                    window.vline(
                        char.to_chtype() | pancurses::COLOR_PAIR(color_pair as u64),
                        (h - 2) as i32,
                    );
                }
            }
        });
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
enum RenderData {
    String(i64, i64, String, i16),
    Rect(i64, i64, i64, i64, char, i16, bool),
}

impl<T> SystemTrait<T> for PancursesRendererSystem
where
    T: EntityComponentDatabase + EntityComponentDatabaseDebug,
{
    fn run(&mut self, db: &mut T) -> Result<SystemEvent, String> {
        // Get window entities
        let window_entities = db.get_entities_by_predicate(|entity_id| {
            db.entity_has_component::<WindowComponent>(entity_id)
                && db.entity_has_component::<PancursesWindowComponent>(entity_id)
                && db.entity_has_component::<SizeComponent>(entity_id)
        });

        let mut window_sizes: Vec<IVector2> = Vec::new();
        for entity_id in &window_entities {
            let size_component = db.get_entity_component::<SizeComponent>(*entity_id)?;
            let size = size_component.data;
            window_sizes.push(size);
        }

        // Gather string and rect data for rendering
        let mut string_data: HashMap<EntityID, HashSet<RenderData>> = HashMap::new();
        let mut rect_data: HashMap<EntityID, HashSet<RenderData>> = HashMap::new();

        let color_set_entities = db.get_entities_by_predicate(|entity_id| {
            db.entity_has_component::<PancursesColorSetComponent>(entity_id)
        });
        let color_set_entity = color_set_entities
            .get(0)
            .expect("Color set entity does not exist");

        let control_entities = db.get_entities_by_predicate(|entity_id| {
            db.entity_has_component::<ControlComponent>(entity_id)
                && db.entity_has_component::<ParentEntityComponent>(entity_id)
                && db.entity_has_component::<PositionComponent>(entity_id)
        });

        for entity_id in control_entities {
            let IVector2(x, y) = if let Ok(global_position_component) =
                db.get_entity_component::<GlobalPositionComponent>(entity_id)
            {
                global_position_component.data
            } else {
                match db.get_entity_component::<PositionComponent>(entity_id) {
                    Ok(position_component) => position_component.data,
                    Err(err) => return Err(err),
                }
            };

            let color_pair = match db.get_entity_component::<PancursesColorPairComponent>(entity_id)
            {
                Ok(pancurses_color_pair_component) => pancurses_color_pair_component.data,
                Err(_) => PancursesColorPair::default(),
            };

            let color_set_component =
                db.get_entity_component_mut::<PancursesColorSetComponent>(*color_set_entity)?;
            let color_pair_idx = color_set_component.get_color_pair_idx(color_pair);

            // Search up parent chain for window component
            let parent_entity_component =
                match db.get_entity_component::<ParentEntityComponent>(entity_id) {
                    Ok(parent_entity_component) => parent_entity_component,
                    Err(err) => return Err(err),
                };

            let mut candidate_id = parent_entity_component.parent_id;
            let mut parent_id: Option<EntityID> = None;

            loop {
                if db
                    .get_entity_component::<PancursesWindowComponent>(candidate_id)
                    .is_ok()
                {
                    parent_id = Some(candidate_id);
                    break;
                }

                match db.get_entity_component::<ParentEntityComponent>(candidate_id) {
                    Ok(parent_entity_component) => candidate_id = parent_entity_component.parent_id,
                    Err(_) => break,
                }
            }

            // Skip rendering this entity if it has no window ancestor
            let parent_id = match parent_id {
                Some(parent_id) => parent_id,
                None => continue,
            };

            // Create render data storage for this entity
            if string_data.get(&parent_id).is_none() {
                string_data.insert(parent_id, HashSet::new());
            }

            if rect_data.get(&parent_id).is_none() {
                rect_data.insert(parent_id, HashSet::new());
            }

            let window_strings = string_data.get_mut(&parent_id).unwrap();
            let window_rects = rect_data.get_mut(&parent_id).unwrap();

            // Extract render data from component
            if db.entity_has_component::<SizeComponent>(&entity_id) {
                //let filled = *filled;
                let filled = db.entity_has_component::<FillComponent>(&entity_id);

                let char = match db.get_entity_component::<CharComponent>(entity_id) {
                    Ok(char_component) => char_component.data,
                    Err(_) => ' ',
                };

                let size_component = db.get_entity_component::<SizeComponent>(entity_id)?;
                let IVector2(w, h) = size_component.data;
                window_rects.insert(RenderData::Rect(x, y, w, h, char, color_pair_idx, filled));
            } else if db.entity_has_component::<StringComponent>(&entity_id)
                || db.entity_has_component::<CharComponent>(&entity_id)
            {
                let string = if let Ok(string_component) =
                    db.get_entity_component::<StringComponent>(entity_id)
                {
                    string_component.data.clone()
                } else if let Ok(char_component) =
                    db.get_entity_component::<CharComponent>(entity_id)
                {
                    char_component.data.to_string()
                } else {
                    return Err("No valid string component".into());
                };

                for (i, string) in string.split('\n').enumerate() {
                    window_strings.insert(RenderData::String(
                        x,
                        y + i as i64,
                        string.to_string(),
                        color_pair_idx,
                    ));
                }
            }
        }

        // Render window contents
        let (root_window_entities, sub_window_entities): (Vec<EntityID>, Vec<EntityID>) =
            window_entities.iter().copied().partition(|entity_id| {
                db.get_entity_component::<ParentEntityComponent>(*entity_id)
                    .is_err()
            });

        for entity_id in root_window_entities
            .iter()
            .copied()
            .chain(sub_window_entities.into_iter())
        {
            let size_component = db.get_entity_component::<SizeComponent>(entity_id).unwrap();
            let IVector2(width, height) = size_component.data;

            let window_component = db
                .get_entity_component::<PancursesWindowComponent>(entity_id)
                .unwrap();
            if let Some(window) = &window_component.window {
                window.erase();

                if let Some(rect_data) = rect_data.get(&entity_id) {
                    self.render_rects(window, width, height, rect_data);
                }

                if let Some(string_data) = string_data.get(&entity_id) {
                    self.render_strings(window, width, height, string_data);
                }
            }
        }

        // Present
        for entity_id in root_window_entities {
            let window_component =
                db.get_entity_component::<PancursesWindowComponent>(entity_id)?;
            if let Some(window) = &window_component.window {
                window.refresh();
            }
        }

        Ok(SystemEvent::None)
    }
}
