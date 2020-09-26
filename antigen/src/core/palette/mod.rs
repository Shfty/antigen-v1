mod adaptive_palette;
mod palette_lookup_table;
mod rgb_arrangement_palette;

pub use adaptive_palette::AdaptivePalette;
pub use palette_lookup_table::PaletteLookupTable;
pub use rgb_arrangement_palette::RGBArrangementPalette;

use crate::primitive_types::{Color, ColorRGBF};

pub trait Palette<F, T>
where
    F: Copy + Clone + PartialOrd + PartialEq,
    T: Copy + Clone + PartialOrd + PartialEq,
{
    fn get_color_idx(&self, color: Color<F>) -> usize;
    fn get_color(&self, idx: usize) -> Color<T>;
    fn get_colors(&self) -> Vec<Color<T>>;
}

impl Palette<f32, f32> for Vec<ColorRGBF> {
    fn get_colors(&self) -> Vec<Color<f32>> {
        self.clone()
    }

    fn get_color(&self, idx: usize) -> ColorRGBF {
        self[idx]
    }

    fn get_color_idx(&self, color: ColorRGBF) -> usize {
        let mut colors: Vec<(usize, ColorRGBF)> = self.clone().into_iter().enumerate().collect();
        colors.sort_by(|(_, lhs), (_, rhs)| {
            let dist_lhs = Color::square_distance(lhs, &color);
            let dist_rhs = Color::square_distance(rhs, &color);

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
            test_lut(palette);
        }
        panic!();
    }

    fn test_palette(palette: &RGBArrangementPalette) {
        for r in 0..10 {
            for g in 0..10 {
                for b in 0..10 {
                    let color = Color(r as f32, g as f32, b as f32) / Color(9.0f32, 9.0f32, 9.0f32);
                    let idx = palette.get_color_idx(color);
                    let color = palette.get_color(idx);
                    println!("Color: {:?}", color);
                }
            }
        }
    }

    fn test_lut(palette: &RGBArrangementPalette) {
        let lut = PaletteLookupTable::new(palette);
        println!(
            "LUT: {:?}",
            lut.indices[16777015..].iter().copied().collect::<Vec<usize>>()
        );
    }
}
