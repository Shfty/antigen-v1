use std::ops::{Range};

use crate::ecs::{ComponentDebugTrait, ComponentTrait};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct AnchorsComponent {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

impl AnchorsComponent {
    pub fn new(horizontal: Range<f32>, vertical: Range<f32>) -> Self {
        AnchorsComponent {
            left: horizontal.start,
            right: horizontal.end,
            top: vertical.start,
            bottom: vertical.end,
        }
    }

    pub fn get_anchors(&self) -> (f32, f32, f32, f32) {
        (self.left, self.right, self.top, self.bottom)
    }

    pub fn set_anchors(&mut self, left: f32, right: f32, top: f32, bottom: f32) -> &mut Self {
        self.left = left;
        self.right = right;
        self.top = top;
        self.bottom = bottom;
        self
    }
}

impl ComponentTrait for AnchorsComponent {}

impl ComponentDebugTrait for AnchorsComponent {
    fn get_name() -> String {
        "Anchors".into()
    }

    fn get_description() -> String {
        "UI anchors".into()
    }
}
