use crate::entity_component_system::{ComponentDebugTrait, ComponentTrait};
use std::ops::Range;

#[derive(Debug, Clone)]
pub struct IntRangeComponent {
    index: i64,
    range: Range<i64>,
}

impl IntRangeComponent {
    pub fn new(range: Range<i64>) -> Self {
        IntRangeComponent { index: range.start, range }
    }

    pub fn get_index(&self) -> i64 {
        self.index
    }

    pub fn set_index(&mut self, index: i64) -> &mut Self {
        self.index = std::cmp::min(std::cmp::max(index, self.range.start), self.range.end - 1);
        self
    }

    pub fn set_range(&mut self, range: Range<i64>) -> &mut Self {
        self.range = range;
        self
    }
}

impl Default for IntRangeComponent {
    fn default() -> Self {
        IntRangeComponent::new(0..0)
    }
}

impl ComponentTrait for IntRangeComponent {}

impl ComponentDebugTrait for IntRangeComponent {
    fn get_name() -> String {
        "Int Range".into()
    }

    fn get_description() -> String {
        "Integer clamped to a range".into()
    }
}
