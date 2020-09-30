mod adaptive_palette;
mod palette_lookup_table;
mod rgb_arrangement_palette;

pub use adaptive_palette::AdaptivePalette;
pub use palette_lookup_table::PaletteLookupTable;
pub use rgb_arrangement_palette::RGBArrangementPalette;

use crate::primitive_types::{ColorRGB, ColorRGBF};

pub trait Palette {
    type From: Copy + Clone + PartialOrd + PartialEq;
    type To: Copy + Clone + PartialOrd + PartialEq;

    fn get_color_idx(&self, color: ColorRGB<Self::From>) -> usize;
    fn get_color(&self, idx: usize) -> ColorRGB<Self::To>;
    fn get_colors(&self) -> Vec<ColorRGB<Self::To>>;
}

impl Palette for Vec<ColorRGBF> {
    type From = f32;
    type To = f32;

    fn get_colors(&self) -> Vec<ColorRGB<f32>> {
        self.clone()
    }

    fn get_color(&self, idx: usize) -> ColorRGBF {
        self[idx]
    }

    fn get_color_idx(&self, color: ColorRGBF) -> usize {
        let mut colors: Vec<(usize, ColorRGBF)> = self.clone().into_iter().enumerate().collect();
        colors.sort_by(|(_, lhs), (_, rhs)| {
            let dist_lhs = ColorRGB::distance(lhs, &color);
            let dist_rhs = ColorRGB::distance(rhs, &color);

            dist_lhs
                .partial_cmp(&dist_rhs)
                .unwrap_or_else(|| panic!("No valid comparison"))
        });
        colors[0].0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn palette() {
        for palette in [
            RGBArrangementPalette::new_666(),
            RGBArrangementPalette::new_676(),
            RGBArrangementPalette::new_685(),
            RGBArrangementPalette::new_884(),
        ]
        .iter()
        {
            test_palette(palette);
        }
        panic!();
    }

    fn test_palette(palette: &RGBArrangementPalette) {
        for r in 0..10 {
            for g in 0..10 {
                for b in 0..10 {
                    let color =
                        ColorRGB(r as f32, g as f32, b as f32) / ColorRGB(9.0f32, 9.0f32, 9.0f32);
                    let idx = palette.get_color_idx(color);
                    let color = palette.get_color(idx);
                    println!("Color: {:?}", color);
                }
            }
        }
    }
}
