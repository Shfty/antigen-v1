use std::ops::Range;

/// Integer clamped to a range
#[derive(Debug, Clone)]
pub struct IntRange {
    index: i64,
    range: Range<i64>,
}

impl IntRange {
    pub fn new(range: Range<i64>) -> Self {
        IntRange {
            index: range.start,
            range,
        }
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

impl Default for IntRange {
    fn default() -> Self {
        IntRange::new(0..0)
    }
}
