use antigen::primitive_types::ColorRGB;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct PancursesColor {
    pub r: i16,
    pub g: i16,
    pub b: i16,
}

impl PancursesColor {
    pub fn new(r: i16, g: i16, b: i16) -> Self {
        PancursesColor { r, g, b }
    }
}

impl Default for PancursesColor {
    fn default() -> Self {
        PancursesColor {
            r: 1000,
            g: 1000,
            b: 1000,
        }
    }
}

impl From<ColorRGB> for PancursesColor {
    fn from(color: ColorRGB) -> Self {
        let r = (color.0 * 1000.0) as i16;
        let g = (color.1 * 1000.0) as i16;
        let b = (color.2 * 1000.0) as i16;
        PancursesColor::new(r, g, b)
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct PancursesColorPair {
    pub foreground: PancursesColor,
    pub background: PancursesColor,
}

impl PancursesColorPair {
    pub fn new(foreground: PancursesColor, background: PancursesColor) -> Self {
        PancursesColorPair {
            foreground,
            background,
        }
    }
}

impl Default for PancursesColorPair {
    fn default() -> Self {
        PancursesColorPair::new(
            PancursesColor::new(1000, 1000, 1000),
            PancursesColor::new(0, 0, 0),
        )
    }
}

pub struct PancursesPalette {
    colors: Vec<(u8, u8, u8)>,

    red_count: u8,
    green_count: u8,
    blue_count: u8,

    red_fac: u8,
    green_fac: u8,
}

impl PancursesPalette {
    fn new(reds: &[u8], greens: &[u8], blues: &[u8]) -> Self {
        let red_count = reds.len();
        let green_count = greens.len();
        let blue_count = blues.len();

        let color_count = red_count * green_count * blue_count;
        assert!(color_count <= 256);

        let red_fac = color_count / reds.len();
        let green_fac = red_fac / greens.len();

        let mut colors: Vec<(u8, u8, u8)> = Vec::new();
        for r in reds {
            for g in greens {
                for b in blues {
                    colors.push((*r, *g, *b))
                }
            }
        }

        let red_fac = red_fac as u8;
        let green_fac = green_fac as u8;

        let red_count = red_count as u8;
        let green_count = green_count as u8;
        let blue_count = blue_count as u8;

        PancursesPalette {
            colors,

            red_count,
            green_count,
            blue_count,

            red_fac,
            green_fac,
        }
    }

    pub fn new_666() -> Self {
        PancursesPalette::new(
            &[0x00, 0x33, 0x66, 0x99, 0xCC, 0xFF],
            &[0x00, 0x33, 0x66, 0x99, 0xCC, 0xFF],
            &[0x00, 0x33, 0x66, 0x99, 0xCC, 0xFF],
        )
    }

    pub fn new_676() -> Self {
        PancursesPalette::new(
            &[0x00, 0x33, 0x66, 0x99, 0xCC, 0xFF],
            &[0x00, 0x2A, 0x55, 0x80, 0xAA, 0xD4, 0xFF],
            &[0x00, 0x33, 0x66, 0x99, 0xCC, 0xFF],
        )
    }

    pub fn new_685() -> Self {
        PancursesPalette::new(
            &[0x00, 0x33, 0x66, 0x99, 0xCC, 0xFF],
            &[0x00, 0x24, 0x49, 0x6D, 0x92, 0xB6, 0xDB, 0xFF],
            &[0x00, 0x40, 0x80, 0xBF, 0xFF],
        )
    }

    pub fn get_color(&self, r: f32, g: f32, b: f32) -> (u8, u8, u8) {
        let red_index = (r * (self.red_count - 1) as f32) as u8;
        let red_index = red_index * self.red_fac;

        let green_index = (g * (self.green_count - 1) as f32) as u8;
        let green_index = green_index * self.green_fac;

        let blue_index = (b * (self.blue_count - 1) as f32) as u8;

        let color_index = red_index + green_index + blue_index;

        self.colors[color_index as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn palette() {
        for palette in [
            PancursesPalette::new_666(),
            PancursesPalette::new_676(),
            PancursesPalette::new_685(),
        ].iter() {
            test_palette(palette);
        }
    }

    fn test_palette(palette: &PancursesPalette) {
        for r in 0..10 {
            for g in 0..10 {
                for b in 0..10 {
                    let r = r as f32 / 9.0;
                    let g = g as f32 / 9.0;
                    let b = b as f32 / 9.0;
                    println!("Color: {}, {}, {}", r, g, b,);
                    let color = palette.get_color(r, g, b);
                    println!("Lookup: {}, {}, {}\n", color.0, color.1, color.2);
                }
            }
        }
    }
}
