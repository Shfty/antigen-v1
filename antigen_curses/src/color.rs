use antigen::primitive_types::ColorRGBF;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct CursesColor {
    pub r: i16,
    pub g: i16,
    pub b: i16,
}

impl CursesColor {
    pub fn new(r: i16, g: i16, b: i16) -> Self {
        CursesColor { r, g, b }
    }
}

impl Default for CursesColor {
    fn default() -> Self {
        CursesColor {
            r: 1000,
            g: 1000,
            b: 1000,
        }
    }
}

impl From<ColorRGBF> for CursesColor {
    fn from(color: ColorRGBF) -> Self {
        let r = (color.0 * 1000.0) as i16;
        let g = (color.1 * 1000.0) as i16;
        let b = (color.2 * 1000.0) as i16;
        CursesColor::new(r, g, b)
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct CursesColorPair {
    pub foreground: CursesColor,
    pub background: CursesColor,
}

impl CursesColorPair {
    pub fn new(foreground: CursesColor, background: CursesColor) -> Self {
        CursesColorPair {
            foreground,
            background,
        }
    }
}

impl Default for CursesColorPair {
    fn default() -> Self {
        CursesColorPair::new(
            CursesColor::new(1000, 1000, 1000),
            CursesColor::new(0, 0, 0),
        )
    }
}
