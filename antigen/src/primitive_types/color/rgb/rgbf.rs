use crate::primitive_types::{
    color::{ColorHSV, ColorHSVF},
    ColorRGB8,
};

use super::ColorRGB;

pub type ColorRGBF = ColorRGB<f32>;

impl ColorRGBF {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        ColorRGB(r, g, b)
    }

    pub fn distance(lhs: &ColorRGB<f32>, rhs: &ColorRGB<f32>) -> f32 {
        let rmean = (lhs.0 + rhs.0) / 2.0;

        let r = lhs.0 - rhs.0;
        let g = lhs.1 - rhs.1;
        let b = lhs.2 - rhs.2;

        let square =
            (2.0 + rmean) * r.powi(2) + 4.0 * g.powi(2) + (2.0 + (1.0 - rmean)) * b.powi(2);

        square.sqrt()
    }
}

impl From<ColorRGB8> for ColorRGBF {
    fn from(color: ColorRGB8) -> Self {
        let ColorRGB(r, g, b) = color;
        ColorRGB(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
    }
}

impl From<ColorHSVF> for ColorRGBF {
    fn from(ColorHSV(hue, sat, val): ColorHSVF) -> Self {
        let chroma: f32 = val * sat;
        let primary_hue = (hue / 60.0) % 6.0;
        let secondary_hue = chroma * (1.0 - ((primary_hue % 2.0) - 1.0).abs());
        let delta = val - chroma;

        let mut r: f32;
        let mut g: f32;
        let mut b: f32;
        if 0.0 <= primary_hue && primary_hue < 1.0 {
            r = chroma;
            g = secondary_hue;
            b = 0.0;
        } else if 1.0 <= primary_hue && primary_hue < 2.0 {
            r = secondary_hue;
            g = chroma;
            b = 0.0;
        } else if 2.0 <= primary_hue && primary_hue < 3.0 {
            r = 0.0;
            g = chroma;
            b = secondary_hue;
        } else if 3.0 <= primary_hue && primary_hue < 4.0 {
            r = 0.0;
            g = secondary_hue;
            b = chroma;
        } else if 4.0 <= primary_hue && primary_hue < 5.0 {
            r = secondary_hue;
            g = 0.0;
            b = chroma;
        } else if 5.0 <= primary_hue && primary_hue < 6.0 {
            r = chroma;
            g = 0.0;
            b = secondary_hue;
        } else {
            r = 0.0;
            g = 0.0;
            b = 0.0;
        }

        r += delta;
        g += delta;
        b += delta;

        ColorRGB(r, g, b)
    }
}
