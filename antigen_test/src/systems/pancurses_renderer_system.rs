use crate::{
    components::{
        pancurses_color_pair_component::PancursesColorPairComponent,
        pancurses_control_component::{ControlData, PancursesControlComponent},
        pancurses_window_component::PancursesWindowComponent,
    },
    pancurses_color::{PancursesColor, PancursesColorPair},
};
use antigen::{
    components::{
        CharComponent, GlobalPositionComponent, PositionComponent, SizeComponent, StringComponent,
    },
    ecs::{SystemTrait, ECS},
    primitive_types::IVector2,
};
use pancurses::{ToChtype, Window};
use std::collections::{HashMap, HashSet};

type WindowID = i64;

#[derive(Debug)]
struct PancursesWindow {
    window: Window,
    width: i64,
    height: i64,
    background_char: char,
    background_color: PancursesColorPair,
}

impl PancursesWindow {
    fn new(
        window: Window,
        width: i64,
        height: i64,
        background_char: char,
        background_color: PancursesColorPair,
    ) -> PancursesWindow {
        PancursesWindow {
            window,
            width,
            height,
            background_char,
            background_color,
        }
    }
}

#[derive(Debug)]
pub struct PancursesRendererSystem {
    input_buffer_size: i64,

    window_head: i64,
    windows: HashMap<WindowID, PancursesWindow>,

    color_head: i16,
    colors: HashMap<PancursesColor, i16>,

    color_pair_head: i16,
    color_pairs: HashMap<PancursesColorPair, i16>,
}

impl PancursesRendererSystem {
    pub fn new(input_buffer_size: i64) -> PancursesRendererSystem {
        PancursesRendererSystem {
            input_buffer_size,
            window_head: 0,
            windows: HashMap::new(),
            color_head: 8,
            colors: HashMap::new(),
            color_pair_head: 1,
            color_pairs: vec![(PancursesColorPair::default(), 0)]
                .into_iter()
                .collect(),
        }
    }

    fn create_window(
        &mut self,
        window_id: WindowID,
        IVector2(pos_x, pos_y): IVector2,
        IVector2(width, height): IVector2,
        background_char: char,
        background_color_pair: PancursesColorPair,
    ) {
        let window: Window;
        if window_id == 0 {
            window = pancurses::initscr();
            pancurses::resize_term(height as i32, width as i32);
            pancurses::curs_set(0);
            pancurses::start_color();
            pancurses::noecho();
            for i in 0..8 {
                let (r, g, b) = pancurses::color_content(i);
                self.colors.insert(PancursesColor::new(r, g, b), i);
            }
        } else {
            window = self
                .windows
                .get(&0)
                .expect("No main window")
                .window
                .derwin(height as i32, width as i32, pos_y as i32, pos_x as i32)
                .unwrap();
        }

        window.keypad(true);
        window.nodelay(true);

        window.bkgdset(
            background_char.to_chtype()
                | pancurses::COLOR_PAIR(self.get_color_pair_idx(background_color_pair) as u64),
        );

        self.windows.insert(
            window_id,
            PancursesWindow::new(
                window,
                width,
                height,
                background_char,
                background_color_pair,
            ),
        );
    }

    fn destroy_window(&mut self, window_id: WindowID) {
        if window_id == 0 {
            pancurses::endwin();
            self.windows.clear();
        } else {
            self.windows.remove(&window_id);
        }
    }

    pub fn get_input(&self) -> Vec<pancurses::Input> {
        let mut input_buffer: Vec<pancurses::Input> = Vec::new();

        let main_window = match self.windows.get(&0) {
            Some(main_window) => main_window,
            None => return input_buffer,
        };

        for _ in 0..self.input_buffer_size {
            if let Some(input) = main_window.window.getch() {
                input_buffer.push(input);
            } else {
                break;
            }
        }

        pancurses::flushinp();

        input_buffer
    }

    pub fn get_color_idx(&mut self, color: PancursesColor) -> i16 {
        match self.colors.get(&color) {
            Some(color) => *color,
            None => {
                let idx = self.color_head;
                pancurses::init_color(idx, color.r, color.g, color.b);
                self.colors.insert(color, idx);
                self.color_head += 1;
                idx
            }
        }
    }

    pub fn get_color_pair_idx(&mut self, color_pair: PancursesColorPair) -> i16 {
        match self.color_pairs.get(&color_pair) {
            Some(color_pair) => *color_pair,
            None => {
                let idx = self.color_pair_head;
                pancurses::init_pair(
                    idx,
                    self.get_color_idx(color_pair.foreground),
                    self.get_color_idx(color_pair.background),
                );
                self.color_pairs.insert(color_pair, idx);
                self.color_pair_head += 1;
                idx
            }
        }
    }

    fn render_strings(&self, pancurses_window: &PancursesWindow, strings: &HashSet<RenderData>) {
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

                    if new_x > pancurses_window.width {
                        new_str.clear();
                    } else if new_x > pancurses_window.width - new_str.len() as i64 {
                        new_str = new_str[..(pancurses_window.width - new_x) as usize].into();
                    }

                    return Some((new_x, *y, new_str, *color));
                }

                None
            })
            .filter(|(_, y, str, _)| {
                let len = str.len() as i64;
                len > 0 && *y >= 0 && *y < pancurses_window.height
            })
            .for_each(|(x, y, string, color_pair)| {
                let mut y = y as i32;
                pancurses_window.window.mv(y, x as i32);
                for char in string.chars() {
                    if char == '\n' {
                        y += 1;
                        pancurses_window.window.mv(y, x as i32);
                    } else {
                        pancurses_window
                            .window
                            .addch(char.to_chtype() | pancurses::COLOR_PAIR(color_pair as u64));
                    }
                }
            });
    }

    fn render_rects(&self, pancurses_window: &PancursesWindow, rects: &HashSet<RenderData>) {
        rects.iter().for_each(|render_data| {
            if let RenderData::Rect(x, y, w, h, char, color_pair, filled) = render_data {
                let x = *x;
                let y = *y;
                let w = *w;
                let h = *h;
                let char = *char;
                let color_pair = *color_pair;

                if *filled {
                    if w >= h {
                        for y in y..y + h {
                            pancurses_window.window.mv(y as i32, x as i32);
                            pancurses_window.window.hline(
                                char.to_chtype() | pancurses::COLOR_PAIR(color_pair as u64),
                                w as i32,
                            );
                        }
                    } else {
                        for x in x..x + w {
                            pancurses_window.window.mv(y as i32, x as i32);
                            pancurses_window.window.vline(
                                char.to_chtype() | pancurses::COLOR_PAIR(color_pair as u64),
                                h as i32,
                            );
                        }
                    }
                } else {
                    pancurses_window.window.mv(y as i32, x as i32);
                    pancurses_window.window.hline(
                        char.to_chtype() | pancurses::COLOR_PAIR(color_pair as u64),
                        w as i32,
                    );

                    pancurses_window.window.mv((y + h - 1) as i32, x as i32);
                    pancurses_window.window.hline(
                        char.to_chtype() | pancurses::COLOR_PAIR(color_pair as u64),
                        w as i32,
                    );

                    pancurses_window.window.mv((y + 1) as i32, x as i32);
                    pancurses_window.window.vline(
                        char.to_chtype() | pancurses::COLOR_PAIR(color_pair as u64),
                        (h - 2) as i32,
                    );

                    pancurses_window
                        .window
                        .mv((y + 1) as i32, (x + w - 1) as i32);
                    pancurses_window.window.vline(
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

impl<T> SystemTrait<T> for PancursesRendererSystem where T: ECS {
    fn run(&mut self, ecs: &mut T) -> Result<(), String> {
        // Get window entities, update internal window state
        let window_entities = ecs.get_entities_by_predicate(|entity_id| {
            ecs.entity_has_component::<PancursesWindowComponent>(entity_id)
                && ecs.entity_has_component::<SizeComponent>(entity_id)
        });

        let mut active_window_ids: Vec<WindowID> = Vec::new();
        let mut active_window_data: HashMap<
            WindowID,
            (IVector2, IVector2, char, PancursesColorPair),
        > = HashMap::new();
        for entity_id in window_entities {
            let position = match ecs.get_entity_component::<PositionComponent>(entity_id) {
                Ok(position_component) => position_component.data,
                Err(_) => IVector2(0, 0),
            };

            let char = match ecs.get_entity_component::<CharComponent>(entity_id) {
                Ok(char_component) => char_component.data,
                Err(_) => ' ',
            };

            let color_pair =
                match ecs.get_entity_component::<PancursesColorPairComponent>(entity_id) {
                    Ok(pancurses_color_pair_component) => pancurses_color_pair_component.data,
                    Err(_) => PancursesColorPair::default(),
                };

            let size_component = ecs.get_entity_component::<SizeComponent>(entity_id)?;
            let size = size_component.data;

            let window_component =
                ecs.get_entity_component::<PancursesWindowComponent>(entity_id)?;

            active_window_ids.push(window_component.window_id);
            active_window_data.insert(
                window_component.window_id,
                (position, size, char, color_pair),
            );
        }

        active_window_ids.sort();

        for window_id in &active_window_ids {
            if self.windows.get(&window_id).is_none() {
                let (position, size, background_char, background_color_pair) =
                    active_window_data[&window_id];
                self.create_window(
                    *window_id,
                    position,
                    size,
                    background_char,
                    background_color_pair,
                );
            }
        }

        let inactive_window_ids: Vec<WindowID> = self
            .windows
            .keys()
            .filter(|window_id| !active_window_ids.contains(window_id))
            .copied()
            .collect();

        for window_id in inactive_window_ids {
            self.destroy_window(window_id);
        }

        // Gather string and rect data for rendering
        let mut string_data: HashMap<WindowID, HashSet<RenderData>> = HashMap::new();
        let mut rect_data: HashMap<WindowID, HashSet<RenderData>> = HashMap::new();

        let control_entities = ecs.get_entities_by_predicate(|entity_id| {
            ecs.entity_has_component::<PancursesControlComponent>(entity_id)
                && ecs.entity_has_component::<PositionComponent>(entity_id)
        });

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

            let color_pair_idx = self.get_color_pair_idx(color_pair);

            let control_component =
                match ecs.get_entity_component::<PancursesControlComponent>(entity_id) {
                    Ok(control_component) => control_component,
                    Err(err) => return Err(err),
                };

            if string_data.get(&control_component.window_id).is_none() {
                string_data.insert(control_component.window_id, HashSet::new());
            }

            if rect_data.get(&control_component.window_id).is_none() {
                rect_data.insert(control_component.window_id, HashSet::new());
            }

            let window_strings = string_data.get_mut(&control_component.window_id).unwrap();
            let window_rects = rect_data.get_mut(&control_component.window_id).unwrap();

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
        for window_id in &active_window_ids {
            let pancurses_window = &self.windows[&window_id];
            let window = &pancurses_window.window;

            window.erase();

            if let Some(rect_data) = rect_data.get(window_id) {
                self.render_rects(pancurses_window, rect_data);
            }

            if let Some(string_data) = string_data.get(window_id) {
                self.render_strings(pancurses_window, string_data);
            }
        }

        // Present
        pancurses::doupdate();

        Ok(())
    }
}
