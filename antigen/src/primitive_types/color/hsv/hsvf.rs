use crate::primitive_types::ColorRGBF;

use super::ColorHSV;

pub type ColorHSVF = ColorHSV<f32>;

impl From<ColorRGBF> for ColorHSVF {
    fn from(rgbf: ColorRGBF) -> Self {
        let hue: f32;
        let sat: f32;
        let val: f32;

        let r = rgbf.0;
        let g = rgbf.1;
        let b = rgbf.2;

        let cmax = r.max(g.max(b));
        let cmin = r.min(g.min(b));
        let diff = cmax - cmin;

        if let Some(std::cmp::Ordering::Equal) = cmax.partial_cmp(&cmin) {
            hue = 0.0;
        } else if let Some(std::cmp::Ordering::Equal) = cmax.partial_cmp(&r) {
            hue = (60.0 * ((g - b) / diff) + 360.0) % 360.0;
        } else if let Some(std::cmp::Ordering::Equal) = cmax.partial_cmp(&g) {
            hue = (60.0 * ((b - r) / diff) + 120.0) % 360.0;
        } else if let Some(std::cmp::Ordering::Equal) = cmax.partial_cmp(&b) {
            hue = (60.0 * ((r - g) / diff) + 240.0) % 360.0;
        } else {
            panic!("Failed to convert color {:?} to HSV", rgbf);
        }

        if cmax == 0.0 {
            sat = 0.0;
        } else {
            sat = (diff / cmax) * 100.0;
        }

        val = cmax * 100.0;

        ColorHSV(hue, sat, val)
    }
}
