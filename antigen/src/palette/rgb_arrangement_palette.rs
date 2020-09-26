use crate::primitive_types::{Color, ColorRGB8, ColorRGBF};

use super::Palette;

pub struct RGBArrangementPalette {
    colors: Vec<ColorRGB8>,

    red_count: u8,
    green_count: u8,
    blue_count: u8,

    red_fac: u8,
    green_fac: u8,
}

impl RGBArrangementPalette {
    fn new(reds: &[u8], greens: &[u8], blues: &[u8]) -> Self {
        let red_count = reds.len();
        let green_count = greens.len();
        let blue_count = blues.len();

        let color_count = red_count * green_count * blue_count;
        assert!(
            color_count <= 256,
            "R{}, G{}, B{} produces more than 256 colors",
            red_count,
            green_count,
            blue_count
        );

        let red_fac = color_count / reds.len();
        let green_fac = red_fac / greens.len();

        let mut colors: Vec<ColorRGB8> = Vec::new();
        for r in reds {
            for g in greens {
                for b in blues {
                    colors.push(Color(*r, *g, *b))
                }
            }
        }

        for i in color_count..256 {
            let idx = i - color_count;
            let range = 256 - color_count;
            let grey = ((idx as f32 / range as f32) * 255.0) as u8;
            colors.push(Color(grey, grey, grey));
        }

        let red_fac = red_fac as u8;
        let green_fac = green_fac as u8;

        let red_count = red_count as u8;
        let green_count = green_count as u8;
        let blue_count = blue_count as u8;

        RGBArrangementPalette {
            colors,

            red_count,
            green_count,
            blue_count,

            red_fac,
            green_fac,
        }
    }

    /// Homogenous RGB cube: 216 colors, 40 true greys
    pub fn new_666() -> Self {
        RGBArrangementPalette::new(
            &[0x00, 0x33, 0x66, 0x99, 0xCC, 0xFF],
            &[0x00, 0x33, 0x66, 0x99, 0xCC, 0xFF],
            &[0x00, 0x33, 0x66, 0x99, 0xCC, 0xFF],
        )
    }

    /// RGB cube with extended G: 252 colors, 12 true greys
    pub fn new_676() -> Self {
        RGBArrangementPalette::new(
            &[0x00, 0x33, 0x66, 0x99, 0xCC, 0xFF],
            &[0x00, 0x2A, 0x55, 0x80, 0xAA, 0xD4, 0xFF],
            &[0x00, 0x33, 0x66, 0x99, 0xCC, 0xFF],
        )
    }

    /// RGB cube with extended G and contracted B: 240 colors, 16 true greys
    pub fn new_685() -> Self {
        RGBArrangementPalette::new(
            &[0x00, 0x33, 0x66, 0x99, 0xCC, 0xFF],
            &[0x00, 0x24, 0x49, 0x6D, 0x92, 0xB6, 0xDB, 0xFF],
            &[0x00, 0x40, 0x80, 0xBF, 0xFF],
        )
    }

    /// RGB cube with extended RG and contracted B: 256 colors, no true greys
    pub fn new_884() -> Self {
        RGBArrangementPalette::new(
            &[0x00, 0x24, 0x49, 0x6D, 0x92, 0xB6, 0xDB, 0xFF],
            &[0x00, 0x24, 0x49, 0x6D, 0x92, 0xB6, 0xDB, 0xFF],
            &[0x00, 55, 0xAA, 0xFF],
        )
    }
}

impl Palette<f32, f32> for RGBArrangementPalette {
    fn get_colors(&self) -> Vec<ColorRGBF> {
        let colors: Vec<ColorRGBF> = self
            .colors
            .iter()
            .map(|color| {
                let color = *color;
                let rgbf: ColorRGBF = color.into();
                rgbf
            })
            .collect();
        colors
    }

    fn get_color_idx(&self, color: ColorRGBF) -> usize {
        let Color(r, g, b) = color;

        let red_index = (r * (self.red_count - 1) as f32).round() as u8;
        let red_index = red_index * self.red_fac;

        let green_index = (g * (self.green_count - 1) as f32).round() as u8;
        let green_index = green_index * self.green_fac;

        let blue_index = (b * (self.blue_count - 1) as f32).round() as u8;

        let color_index = red_index + green_index + blue_index;

        color_index as usize
    }

    fn get_color(&self, idx: usize) -> ColorRGBF {
        self.colors[idx].into()
    }
}
