use std::{collections::VecDeque, fmt::Debug};

use antigen::{
    core::palette::RGBArrangementPalette,
    primitive_types::{ColorIndex, ColorRGB, ColorRGBF},
};

use antigen::core::palette::Palette;

#[derive(Clone)]
pub struct CursesPalette {
    pub colors: [ColorRGBF; 256],
    pub palette: Option<RGBArrangementPalette>,
    pub lut: [usize; 256],
}

impl Default for CursesPalette {
    fn default() -> Self {
        CursesPalette {
            colors: [ColorRGBF::default(); 256],
            palette: None,
            lut: [0; 256],
        }
    }
}

impl Debug for CursesPalette {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CursesPalette")
            .field("colors", &self.colors.to_vec())
            .field("palette", &self.palette)
            .field("lut", &self.lut.to_vec())
            .finish()
    }
}

impl CursesPalette {
    pub fn set_palette(&mut self, palette: RGBArrangementPalette) {
        // Create pancurses > palette map to make sure built-in pancurses colors are respected
        let palette_black: usize = ColorRGBF::new(0.0, 0.0, 0.0).into_index(&palette).into();
        let palette_red: usize = ColorRGBF::new(1.0, 0.0, 0.0).into_index(&palette).into();
        let palette_green: usize = ColorRGBF::new(0.0, 1.0, 0.0).into_index(&palette).into();
        let palette_yellow: usize = ColorRGBF::new(1.0, 1.0, 0.0).into_index(&palette).into();
        let palette_blue: usize = ColorRGBF::new(0.0, 0.0, 1.0).into_index(&palette).into();
        let palette_magenta: usize = ColorRGBF::new(1.0, 0.0, 1.0).into_index(&palette).into();
        let palette_cyan: usize = ColorRGBF::new(0.0, 1.0, 1.0).into_index(&palette).into();
        let palette_white: usize = ColorRGBF::new(1.0, 1.0, 1.0).into_index(&palette).into();

        let mut indices: VecDeque<usize> = (0..256).collect();
        indices.remove(indices.iter().position(|i| *i == palette_black).unwrap());
        indices.remove(indices.iter().position(|i| *i == palette_red).unwrap());
        indices.remove(indices.iter().position(|i| *i == palette_green).unwrap());
        indices.remove(indices.iter().position(|i| *i == palette_yellow).unwrap());
        indices.remove(indices.iter().position(|i| *i == palette_blue).unwrap());
        indices.remove(indices.iter().position(|i| *i == palette_magenta).unwrap());
        indices.remove(indices.iter().position(|i| *i == palette_cyan).unwrap());
        indices.remove(indices.iter().position(|i| *i == palette_white).unwrap());

        indices.push_front(palette_white);
        indices.push_front(palette_cyan);
        indices.push_front(palette_magenta);
        indices.push_front(palette_blue);
        indices.push_front(palette_yellow);
        indices.push_front(palette_green);
        indices.push_front(palette_red);
        indices.push_front(palette_black);

        let mut lut: [usize; 256] = [0; 256];
        (0..256).for_each(|i| lut[i] = indices.iter().position(|idx| *idx == i).unwrap());

        let mut colors: [ColorRGBF; 256] = [ColorRGBF::default(); 256];
        indices
            .into_iter()
            .enumerate()
            .for_each(|(i, index)| colors[i] = palette.get_color(ColorIndex(index)));

        self.colors = colors;
        self.palette = Some(palette);
        self.lut = lut;
    }
}

impl Palette for CursesPalette {
    type Color = f32;

    fn get_color_idx(&self, color: ColorRGB<Self::Color>) -> ColorIndex {
        let palette = self
            .palette
            .as_ref()
            .expect("Called get_color_idx() without setting a palette");

        let idx: usize = palette.get_color_idx(color).into();
        let idx = self.lut[idx];
        ColorIndex(idx)
    }

    fn get_colors(&self) -> Vec<ColorRGB<Self::Color>> {
        self.colors.iter().copied().collect()
    }

    fn get_color(&self, idx: ColorIndex) -> ColorRGB<Self::Color> {
        let idx: usize = idx.into();
        self.colors[idx]
    }
}
