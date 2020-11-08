mod adaptive_palette;
mod palette_lookup_table;
mod rgb_arrangement_palette;

pub use adaptive_palette::AdaptivePalette;
pub use palette_lookup_table::PaletteLookupTable;
pub use rgb_arrangement_palette::RGBArrangementPalette;

use crate::primitive_types::{ColorIndex, ColorRGB, ColorRGBF};

pub trait Palette {
    type Color: Copy + PartialOrd;

    fn get_color_idx(&self, color: ColorRGB<Self::Color>) -> ColorIndex;
    fn get_color(&self, idx: ColorIndex) -> ColorRGB<Self::Color>;
    fn get_colors(&self) -> Vec<ColorRGB<Self::Color>>;
}

impl<T> Palette for Vec<ColorRGB<T>>
where
    T: Copy + Clone + PartialOrd + PartialEq,
    ColorRGB<T>: Into<ColorRGBF>,
{
    type Color = T;

    fn get_colors(&self) -> Vec<ColorRGB<T>> {
        self.clone()
    }

    fn get_color(&self, idx: ColorIndex) -> ColorRGB<T> {
        let idx: usize = idx.into();
        self[idx]
    }

    fn get_color_idx(&self, color: ColorRGB<T>) -> ColorIndex {
        let mut colors: Vec<(usize, ColorRGB<T>)> = self.clone().into_iter().enumerate().collect();
        colors.sort_by(|(_, lhs), (_, rhs)| {
            let lhs = *lhs;
            let lhs: ColorRGBF = lhs.into();

            let rhs = *rhs;
            let rhs: ColorRGBF = rhs.into();

            let color: ColorRGBF = color.into();

            let dist_lhs = ColorRGB::distance(&lhs, &color);
            let dist_rhs = ColorRGB::distance(&rhs, &color);

            dist_lhs
                .partial_cmp(&dist_rhs)
                .unwrap_or_else(|| panic!("No valid comparison"))
        });

        ColorIndex(colors[0].0)
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
                    let idx = color.into_index(palette);
                    let color = palette.get_color(idx);
                    println!("Color: {:?}", color);
                }
            }
        }
    }
}
