use std::{fmt::Debug, ops::Range};

#[derive(Debug, Clone, PartialEq)]
pub struct Anchors {
    horizontal: Range<f32>,
    vertical: Range<f32>,
}

impl Anchors {
    pub fn new(horizontal: Range<f32>, vertical: Range<f32>) -> Self {
        Anchors {
            horizontal,
            vertical,
        }
    }

    pub fn get_anchors(&self) -> (f32, f32, f32, f32) {
        (
            self.horizontal.start,
            self.horizontal.end,
            self.vertical.start,
            self.vertical.end,
        )
    }

    pub fn set_anchors(&mut self, left: f32, right: f32, top: f32, bottom: f32) {
        self.horizontal.start = left;
        self.horizontal.end = right;
        self.vertical.start = top;
        self.vertical.end = bottom;
    }
}

impl Default for Anchors {
    fn default() -> Self {
        Anchors {
            horizontal: 0.0..1.0,
            vertical: 0.0..1.0,
        }
    }
}
