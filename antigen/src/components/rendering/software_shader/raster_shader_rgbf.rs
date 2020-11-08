use crate::{
    primitive_types::{ColorHSV, ColorRGBF},
    systems::RasterInput,
};

use super::SoftwareShader;

pub type RasterShaderRGBF = SoftwareShader<RasterInput, ColorRGBF>;

impl RasterShaderRGBF {
    pub fn uv(input: RasterInput) -> Option<ColorRGBF> {
        let (u, v) = input.get_uv();
        Some(ColorRGBF::new(u, v, 0.0))
    }

    pub fn gradient_horizontal(color: ColorRGBF) -> impl Fn(RasterInput) -> Option<ColorRGBF> {
        move |input: RasterInput| {
            let (u, _) = input.get_uv();
            Some(color * u)
        }
    }

    pub fn gradient_vertical(color: ColorRGBF) -> Self {
        SoftwareShader::new(move |input: RasterInput| {
            let (_, v) = input.get_uv();
            Some(color * v)
        })
    }

    pub fn hsv() -> Self {
        SoftwareShader::new(move |input: RasterInput| {
            let (u, v) = input.get_uv();

            let hue = v * 360.0;

            let norm = (u - 0.5) * -2.0;
            let sat = 1.0 - norm.min(0.0).abs();
            let val = 1.0 - norm.max(0.0);

            Some(ColorHSV(hue, sat, val).into())
        })
    }

    pub fn color(color: ColorRGBF) -> Self {
        SoftwareShader::new(move |_: RasterInput| Some(color))
    }

    pub fn rect(color: ColorRGBF) -> Self {
        SoftwareShader::new(move |input: RasterInput| {
            let local_pos = input.local_pos;
            let size = input.size;

            if local_pos.0 > 0
                && local_pos.0 < size.0 - 1
                && local_pos.1 > 0
                && local_pos.1 < size.1 - 1
            {
                return None;
            }

            Some(color)
        })
    }
}
