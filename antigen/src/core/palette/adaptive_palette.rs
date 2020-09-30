use crate::primitive_types::{ColorRGB, ColorRGBF};

use super::Palette;

pub struct AdaptivePalette {
    colors: Vec<ColorRGBF>,
}

impl AdaptivePalette {
    pub fn new(source_palette: impl Palette<From = f32, To = f32>) -> Self {
        let mut colors: Vec<ColorRGBF> = source_palette.get_colors().iter().copied().collect();
        let colors = Self::median_cut(colors.len(), 256, &mut colors);
        AdaptivePalette { colors }
    }

    fn median_cut(
        source_size: usize,
        target_size: usize,
        colors: &mut [ColorRGBF],
    ) -> Vec<ColorRGBF> {
        if colors.len() <= 2 {
            return colors.iter().copied().collect();
        }

        let sort_red = |lhs: &ColorRGBF, rhs: &ColorRGBF| {
            let ColorRGB(lhs, _, _) = lhs;
            let ColorRGB(rhs, _, _) = rhs;
            lhs.partial_cmp(&rhs)
                .unwrap_or_else(|| panic!("No valid comparison"))
        };

        let sort_green = |lhs: &ColorRGBF, rhs: &ColorRGBF| {
            let ColorRGB(_, lhs, _) = lhs;
            let ColorRGB(_, rhs, _) = rhs;
            lhs.partial_cmp(&rhs)
                .unwrap_or_else(|| panic!("No valid comparison"))
        };

        let sort_blue = |lhs: &ColorRGBF, rhs: &ColorRGBF| {
            let ColorRGB(_, _, lhs) = lhs;
            let ColorRGB(_, _, rhs) = rhs;
            lhs.partial_cmp(&rhs)
                .unwrap_or_else(|| panic!("No valid comparison"))
        };

        let ColorRGB(total_r, total_g, total_b) = colors
            .iter()
            .fold(ColorRGBF::default(), |acc, next| acc + *next);

        let sort = if total_r > total_g && total_r > total_b {
            sort_red
        } else if total_g > total_r && total_g > total_b {
            sort_green
        } else if total_b > total_r && total_b > total_g {
            sort_blue
        } else if total_b < 1.0 {
            sort_red
        } else if total_g < 1.0 {
            sort_green
        } else if total_r < 1.0 {
            sort_blue
        } else {
            panic!("No dominant color");
        };

        let color_count = colors.len();
        let half_count = color_count / 2;

        colors.sort_by(sort);
        let (left_colors, right_colors) = colors.split_at_mut(half_count);

        let threshold = source_size / target_size;
        if left_colors.len() == 1
            || right_colors.len() == 1
            || left_colors.len() <= threshold
            || right_colors.len() <= threshold
        {
            let left_color = left_colors
                .iter()
                .fold(ColorRGB(0.0, 0.0, 0.0), |acc, next| acc + *next)
                / ColorRGB(
                    left_colors.len() as f32,
                    left_colors.len() as f32,
                    left_colors.len() as f32,
                );

            let right_color = right_colors
                .iter()
                .fold(ColorRGB(0.0, 0.0, 0.0), |acc, next| acc + *next)
                / ColorRGB(
                    right_colors.len() as f32,
                    right_colors.len() as f32,
                    right_colors.len() as f32,
                );

            vec![left_color, right_color]
        } else {
            Self::median_cut(source_size, target_size, left_colors)
                .iter()
                .chain(Self::median_cut(source_size, target_size, right_colors).iter())
                .copied()
                .collect()
        }
    }
}

impl Palette for AdaptivePalette {
    type From = f32;
    type To = f32;
    fn get_colors(&self) -> Vec<ColorRGBF> {
        self.colors.clone()
    }

    fn get_color_idx(&self, color: ColorRGBF) -> usize {
        let mut colors: Vec<(usize, ColorRGBF)> =
            self.colors.clone().into_iter().enumerate().collect();
        colors.sort_by(|(_, lhs), (_, rhs)| {
            let lhs_distance = ColorRGB::distance(&color, lhs);
            let rhs_distance = ColorRGB::distance(&color, rhs);
            lhs_distance
                .partial_cmp(&rhs_distance)
                .unwrap_or_else(|| panic!("No valid comparison"))
        });
        colors[0].0
    }

    fn get_color(&self, idx: usize) -> ColorRGBF {
        self.colors[idx]
    }
}
