use crate::{
    components::pancurses_color_set_component::PancursesColorSetComponent,
    components::{
        pancurses_color_pair_component::PancursesColorPairComponent,
        pancurses_control_component::{ControlData, PancursesControlComponent},
        pancurses_window_component::PancursesWindowComponent,
    },
    pancurses_color::PancursesColorPair,
};
use antigen::{
    components::ParentEntityComponent,
    components::{
        CharComponent, GlobalPositionComponent, PositionComponent, SizeComponent, StringComponent,
    },
    ecs::EntityID,
    ecs::{EntityComponentSystem, SystemEvent, SystemTrait},
    primitive_types::IVector2,
ecs::EntityComponentSystemDebug};
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

impl<T> SystemTrait<T> for PancursesRendererSystem where T: EntityComponentSystem + EntityComponentSystemDebug
{
    fn run(&mut self, ecs: &mut T) -> Result<SystemEvent, String> {
        // Get window entities, update internal window state
        let mut window_entities = ecs.get_entities_by_predicate(|entity_id| {
            ecs.entity_has_component::<PancursesWindowComponent>(entity_id)
                && ecs.entity_has_component::<SizeComponent>(entity_id)
        });

        window_entities.sort_by(|lhs, rhs| {
            let lhs_window_component = ecs
                .get_entity_component::<PancursesWindowComponent>(*lhs)
                .unwrap();
            let lhs_window_id = lhs_window_component.window_id;

            let rhs_window_component = ecs
                .get_entity_component::<PancursesWindowComponent>(*rhs)
                .unwrap();
            let rhs_window_id = rhs_window_component.window_id;

            lhs_window_id.cmp(&rhs_window_id)
        });

        let mut window_sizes: Vec<IVector2> = Vec::new();
        for entity_id in &window_entities {
            let size_component = ecs.get_entity_component::<SizeComponent>(*entity_id)?;
            let size = size_component.data;
            window_sizes.push(size);
        }

        // Gather string and rect data for rendering
        let mut string_data: HashMap<EntityID, HashSet<RenderData>> = HashMap::new();
        let mut rect_data: HashMap<EntityID, HashSet<RenderData>> = HashMap::new();

        let control_entities = ecs.get_entities_by_predicate(|entity_id| {
            ecs.entity_has_component::<PancursesControlComponent>(entity_id)
                && ecs.entity_has_component::<ParentEntityComponent>(entity_id)
                && ecs.entity_has_component::<PositionComponent>(entity_id)
        });

        let color_set_entities = ecs.get_entities_by_predicate(|entity_id| {
            ecs.entity_has_component::<PancursesColorSetComponent>(entity_id)
        });
        let color_set_entity = color_set_entities
            .get(0)
            .expect("Color set entity does not exist");

        for entity_id in control_entities {
            let IVector2(x, y) = if let Ok(global_position_component) =
                ecs.get_entity_component::<GlobalPositionComponent>(entity_id)
            {
                global_position_component.data
            } else {
                match ecs.get_entity_component::<PositionComponent>(entity_id) {
                    Ok(position_component) => position_component.data,
                    Err(err) => return Err(err),
                }
            };

            let color_pair =
                match ecs.get_entity_component::<PancursesColorPairComponent>(entity_id) {
                    Ok(pancurses_color_pair_component) => pancurses_color_pair_component.data,
                    Err(_) => PancursesColorPair::default(),
                };

            let color_set_component =
                ecs.get_entity_component::<PancursesColorSetComponent>(*color_set_entity)?;
            let color_pair_idx = color_set_component.get_color_pair_idx(color_pair);

            // Search up parent chain for window component
            let parent_entity_component =
                match ecs.get_entity_component::<ParentEntityComponent>(entity_id) {
                    Ok(parent_entity_component) => parent_entity_component,
                    Err(err) => return Err(err),
                };

            let mut candidate_id = parent_entity_component.parent_id;
            let mut parent_id: Option<EntityID> = None;

            loop {
                if ecs
                    .get_entity_component::<PancursesWindowComponent>(candidate_id)
                    .is_ok()
                {
                    parent_id = Some(candidate_id);
                    break;
                }

                match ecs.get_entity_component::<ParentEntityComponent>(candidate_id) {
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

            // Extract render data from control component
            let control_component =
                match ecs.get_entity_component::<PancursesControlComponent>(entity_id) {
                    Ok(control_component) => control_component,
                    Err(err) => return Err(err),
                };

            match &control_component.control_data {
                ControlData::String => {
                    let string = if let Ok(string_component) =
                        ecs.get_entity_component::<StringComponent>(entity_id)
                    {
                        string_component.data.clone()
                    } else if let Ok(char_component) =
                        ecs.get_entity_component::<CharComponent>(entity_id)
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
                ControlData::Rect { filled } => {
                    let filled = *filled;

                    let char = match ecs.get_entity_component::<CharComponent>(entity_id) {
                        Ok(char_component) => char_component.data,
                        Err(_) => ' ',
                    };

                    let size_component = ecs.get_entity_component::<SizeComponent>(entity_id)?;
                    let IVector2(w, h) = size_component.data;
                    window_rects.insert(RenderData::Rect(x, y, w, h, char, color_pair_idx, filled));
                }
            }
        }

        // Render window contents
        let mut root_window_entity = None;

        for (entity_id, IVector2(width, height)) in window_entities.iter().zip(window_sizes.iter())
        {
            let parent_entity_id =
                match ecs.get_entity_component::<ParentEntityComponent>(*entity_id) {
                    Ok(parent_entity_component) => Some(parent_entity_component.parent_id),
                    Err(_) => None,
                };

            if parent_entity_id.is_none() {
                root_window_entity = Some(*entity_id);
            }

            let window_component =
                ecs.get_entity_component::<PancursesWindowComponent>(*entity_id)?;
            if let Some(window) = &window_component.window {
                window.erase();

                let width = *width;
                let height = *height;

                if let Some(rect_data) = rect_data.get(&entity_id) {
                    self.render_rects(window, width, height, rect_data);
                }

                if let Some(string_data) = string_data.get(&entity_id) {
                    self.render_strings(window, width, height, string_data);
                }
            }
        }

        // Present
        if let Some(entity_id) = root_window_entity {
            let window_component =
                ecs.get_entity_component::<PancursesWindowComponent>(entity_id)?;
            if let Some(window) = &window_component.window {
                window.refresh();
            }
        }

        Ok(SystemEvent::None)
    }
}
