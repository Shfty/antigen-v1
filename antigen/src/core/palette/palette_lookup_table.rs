use crate::primitive_types::{ColorRGB, ColorRGB8, ColorRGBF};

use super::Palette;

/// Wraps a palette in a 256^3 array to accellerate lookups for 8-bit colors
#[derive(Debug)]
pub struct PaletteLookupTable {
    pub colors: Vec<ColorRGBF>,
    pub indices: Vec<usize>,
}

impl PaletteLookupTable {
    pub fn new(palette: &impl Palette<f32, f32>) -> Self {
        let mut indices: Vec<usize> = Vec::new();
        indices.resize(256 * 256 * 256, 0);
        for r in 0..256usize {
            for g in 0..256usize {
                for b in 0..256usize {
                    let lut_idx = r * (256 * 256) + g * 256 + b;
                    let rgbf: ColorRGBF = ColorRGB(r as u8, g as u8, b as u8).into();
                    let palette_idx = palette.get_color_idx(rgbf);
                    indices[lut_idx] = palette_idx;
                }
            }
        }

        PaletteLookupTable {
            colors: palette.get_colors(),
            indices,
        }
    }
}

impl Palette<f32, f32> for PaletteLookupTable {
    fn get_color_idx(&self, color: ColorRGBF) -> usize {
        let color: ColorRGB8 = color.into();
        let idx = (color.0 as usize) * 256 * 256 + (color.1 as usize) * 256 + (color.2 as usize);
        self.indices[idx]
    }

    fn get_colors(&self) -> Vec<ColorRGBF> {
        self.colors.clone()
    }

    fn get_color(&self, idx: usize) -> ColorRGBF {
        self.colors[idx]
    }
}
