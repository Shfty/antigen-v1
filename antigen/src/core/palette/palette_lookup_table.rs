use crate::primitive_types::{ColorIndex, ColorRGB8, ColorRGBF};

use super::Palette;

/// Wraps a palette in a 256^3 array to accellerate lookups for 8-bit colors
#[derive(Debug)]
pub struct PaletteLookupTable {
    pub colors: Vec<ColorRGBF>,
    pub indices: Vec<ColorIndex>,
}

impl PaletteLookupTable {
    pub fn new(palette: &impl Palette<Color = f32>) -> Self {
        let mut indices: Vec<ColorIndex> = Vec::new();
        indices.resize(256 * 256 * 256, ColorIndex(0));
        for r in 0..256usize {
            for g in 0..256usize {
                for b in 0..256usize {
                    let lut_idx = r * (256 * 256) + g * 256 + b;
                    let rgbf: ColorRGBF = ColorRGB8::new(r as u8, g as u8, b as u8).into();
                    let palette_idx = rgbf.into_index(palette);
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

impl Palette for PaletteLookupTable {
    type Color = f32;

    fn get_color_idx(&self, color: ColorRGBF) -> ColorIndex {
        let color: ColorRGB8 = color.into();
        let idx = (color.0 as usize) * 256 * 256 + (color.1 as usize) * 256 + (color.2 as usize);
        self.indices[idx]
    }

    fn get_colors(&self) -> Vec<ColorRGBF> {
        self.colors.clone()
    }

    fn get_color(&self, idx: ColorIndex) -> ColorRGBF {
        let idx: usize = idx.into();
        self.colors[idx]
    }
}
