use crate::components::pancurses_color_component::{
    PancursesColor, PancursesColorPair, PancursesColorPairComponent,
};
use antigen::{
    components::{CharComponent, PositionComponent, StringSliceComponent},
    ecs::{SystemTrait, ECS},
};
use pancurses::ToChtype;
use std::collections::HashMap;

#[derive(Debug)]
pub struct PancursesRendererSystem {
    resolution: (i32, i32),
    background_char: char,
    background_color_pair: PancursesColorPair,

    window: Option<pancurses::Window>,

    color_head: i16,
    colors: HashMap<PancursesColor, i16>,

    color_pair_head: i16,
    color_pairs: HashMap<PancursesColorPair, i16>,
}

impl PancursesRendererSystem {
    pub fn new(
        resolution_x: i32,
        resolution_y: i32,
        background_char: char,
        background_color_pair: PancursesColorPair,
    ) -> PancursesRendererSystem {
        PancursesRendererSystem {
            resolution: (resolution_x, resolution_y),
            background_char,
            background_color_pair,
            window: None,
            color_head: 8,
            colors: HashMap::new(),
            color_pair_head: 1,
            color_pairs: vec![(PancursesColorPair::default(), 0)]
                .into_iter()
                .collect(),
        }
    }

    pub fn initialize(&mut self) {
        let window = pancurses::initscr();
        pancurses::resize_term(self.resolution.1, self.resolution.0);
        pancurses::curs_set(0);
        pancurses::start_color();
        pancurses::noecho();

        window.keypad(true);
        window.nodelay(true);

        for i in 0..8 {
            let (r, g, b) = pancurses::color_content(i);
            self.colors.insert(PancursesColor::new(r, g, b), i);
        }

        window.bkgdset(
            self.background_char.to_chtype()
                | pancurses::COLOR_PAIR(self.get_color_pair_idx(self.background_color_pair) as u64),
        );

        self.window = Some(window);
    }

    pub fn get_input(&self) -> Vec<pancurses::Input> {
        let mut input_buffer: Vec<pancurses::Input> = Vec::new();

        let window = match &self.window {
            Some(window) => window,
            None => return input_buffer,
        };

        while let Some(input) = window.getch() {
            input_buffer.push(input);
        }

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

    pub fn finalize(&mut self) {
        pancurses::endwin();
    }
}

impl SystemTrait for PancursesRendererSystem {
    fn run(&mut self, ecs: &mut ECS) -> Result<(), String> {
        let entities = ecs
            .build_entity_query()
            .component::<PositionComponent>()
            .finish();

        let mut positions: Vec<(i64, i64, String, i16)> = Vec::new();
        for entity_id in entities {
            let string = match ecs.get_entity_component::<StringSliceComponent>(entity_id) {
                Ok(string_component) => string_component.data.to_string(),
                Err(_) => match ecs.get_entity_component::<CharComponent>(entity_id) {
                    Ok(char_component) => char_component.data.to_string(),
                    Err(err) => return Err(err),
                },
            };

            let color_pair_idx =
                match ecs.get_entity_component::<PancursesColorPairComponent>(entity_id) {
                    Ok(pancurses_color_component) => {
                        self.get_color_pair_idx(pancurses_color_component.color_pair)
                    }
                    Err(_) => 0,
                };

            let position_component = ecs.get_entity_component::<PositionComponent>(entity_id)?;
            positions.push((
                position_component.x,
                position_component.y,
                string,
                color_pair_idx,
            ))
        }

        let window = match &self.window {
            Some(window) => window,
            None => return Err("Window has not been initialized".into()),
        };

        window.erase();

        positions
            .iter()
            .map(|(x, y, str, color)| {
                let len = str.len() as i64;

                let mut new_x = *x;
                let mut new_str = str.clone();
                if *x < -len {
                    new_str.clear();
                } else if *x < 0 {
                    new_x = 0;
                    new_str = str[(len - (len + x)) as usize..].into();
                } else if *x > (self.resolution.0 as i64) {
                    new_str.clear();
                } else if *x > (self.resolution.0 as i64) - len {
                    new_str = str[..((self.resolution.0 as i64) - x) as usize].into();
                }

                (new_x, *y, new_str, *color)
            })
            .filter(|(_, y, str, _)| {
                let len = str.len() as i64;
                len > 0 && *y >= 0 && *y < (self.resolution.1 as i64)
            })
            .for_each(|(x, y, str, color_pair)| {
                window.mvaddstr(y as i32, x as i32, &str);
                window.mvchgat(
                    y as i32,
                    x as i32,
                    str.len() as i32,
                    pancurses::A_NORMAL,
                    color_pair,
                );
            });

        window.refresh();

        Ok(())
    }
}
