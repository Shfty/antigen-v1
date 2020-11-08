use crate::core::palette::Palette;

use super::ColorRGB;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct ColorIndex(pub usize);

impl ColorIndex {
    pub fn from_rgb<T, U>(color: ColorRGB<U>, palette: &T) -> ColorIndex
    where
        T: Palette<Color = U>,
        U: Copy + Clone + PartialEq + PartialOrd + 'static,
    {
        palette.get_color_idx(color)
    }

    pub fn into_rgb<T, U>(self, palette: &T) -> ColorRGB<U>
    where
        T: Palette<Color = U>,
        U: Copy + Clone + PartialEq + PartialOrd + 'static,
    {
        palette.get_color(self)
    }
}

impl Into<usize> for ColorIndex {
    fn into(self) -> usize {
        self.0
    }
}

impl From<usize> for ColorIndex {
    fn from(data: usize) -> Self {
        ColorIndex(data)
    }
}
