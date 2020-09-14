use std::collections::HashMap;

use antigen::ecs::{ComponentDebugTrait, ComponentTrait};

use crate::pancurses_color::{PancursesColor, PancursesColorPair};

#[derive(Debug, Clone)]
pub struct PancursesColorSetComponent {
    color_head: i16,
    color_pair_head: i16,
    colors: HashMap<PancursesColor, i16>,
    color_pairs: HashMap<PancursesColorPair, i16>,
}

impl PancursesColorSetComponent {
    pub fn new(
        colors: HashMap<PancursesColor, i16>,
        color_pairs: HashMap<PancursesColorPair, i16>,
    ) -> Self {
        PancursesColorSetComponent {
            color_head: colors.len() as i16,
            color_pair_head: color_pairs.len() as i16,
            colors,
            color_pairs,
        }
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

    pub fn get_color_pair_idx(&mut self, color_pair: &PancursesColorPair) -> i16 {
        match self.color_pairs.get(color_pair) {
            Some(color_pair) => *color_pair,
            None => {
                let idx = self.color_pair_head;
                pancurses::init_pair(
                    idx as i16,
                    self.get_color_idx(color_pair.foreground),
                    self.get_color_idx(color_pair.background),
                );
                self.color_pairs.insert(*color_pair, idx);
                self.color_pair_head += 1;
                idx
            }
        }
    }
}

impl Default for PancursesColorSetComponent {
    fn default() -> Self {
        PancursesColorSetComponent::new(HashMap::new(), HashMap::new())
    }
}

impl ComponentTrait for PancursesColorSetComponent {}

impl ComponentDebugTrait for PancursesColorSetComponent {
    fn get_name() -> String {
        "Pancurses Color Set".into()
    }

    fn get_description() -> String {
        "Color set data for the Pancurses renderer".into()
    }
}
