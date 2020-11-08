#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ListData {
    selected_index: Option<usize>,
    scroll_offset: usize,
}

impl ListData {
    pub fn new() -> Self {
        ListData {
            selected_index: None,
            scroll_offset: 0,
        }
    }

    pub fn get_selected_index(&self) -> Option<usize> {
        self.selected_index
    }

    pub fn get_scroll_offset(&self) -> usize {
        self.scroll_offset
    }

    pub fn set_selected_index(&mut self, selected_index: Option<usize>) {
        self.selected_index = selected_index
    }

    pub fn add_scroll_offset(&mut self, scroll_offset: i64) {
        let new_offset = std::cmp::max(self.scroll_offset as i64 + scroll_offset, 0);
        self.scroll_offset = new_offset as usize;
    }
}
