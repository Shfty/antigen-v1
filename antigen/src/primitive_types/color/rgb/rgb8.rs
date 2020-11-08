use super::{ColorRGB, ColorRGBF};

pub type ColorRGB8 = ColorRGB<u8>;

impl ColorRGB8 {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        ColorRGB(r, g, b)
    }
}

impl From<ColorRGBF> for ColorRGB8 {
    fn from(color: ColorRGBF) -> Self {
        let ColorRGB(r, g, b) = color;
        ColorRGB((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
    }
}
