#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct ColorRGB<T: Default + Ord> {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl<T> ColorRGB<T>
where
    T: Default + Ord,
{
    pub fn new(r: T, g: T, b: T) -> Self {
        ColorRGB { r, g, b }
    }
}

impl<T> Default for ColorRGB<T>
where
    T: Default + Ord,
{
    fn default() -> Self {
        ColorRGB {
            r: T::default(),
            g: T::default(),
            b: T::default(),
        }
    }
}

pub type PancursesColor = ColorRGB<i16>;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct PancursesColorPair {
    pub foreground: PancursesColor,
    pub background: PancursesColor,
}

impl PancursesColorPair {
    pub fn new(foreground: PancursesColor, background: PancursesColor) -> Self {
        PancursesColorPair {
            foreground,
            background,
        }
    }
}

impl Default for PancursesColorPair {
    fn default() -> Self {
        PancursesColorPair::new(
            PancursesColor::new(1000, 1000, 1000),
            PancursesColor::new(0, 0, 0),
        )
    }
}
