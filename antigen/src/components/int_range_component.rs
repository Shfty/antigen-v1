use crate::ecs::{ComponentMetadataTrait, ComponentTrait};
use std::ops::Range;

#[derive(Debug, Clone)]
pub struct IntRangeComponent {
    pub index: i64,
    pub range: Range<i64>,
}

impl IntRangeComponent {
    pub fn new(range: Range<i64>) -> Self {
        IntRangeComponent { index: 0, range }
    }
}

impl Default for IntRangeComponent {
    fn default() -> Self {
        IntRangeComponent::new(0..0)
    }
}

impl ComponentTrait for IntRangeComponent {}

impl ComponentMetadataTrait for IntRangeComponent {
    fn get_name() -> &'static str {
        "Int Range"
    }

    fn get_description() -> &'static str {
        "Integer clamped to a range"
    }
}
