use crate::primitive_types::{ColorRGB, ColorRGBF, Vector2I};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct CPUShaderInput {
    local_pos: Vector2I,
    size: Vector2I,
    color: ColorRGBF,
}

impl CPUShaderInput {
    pub fn new(local_pos: Vector2I, size: Vector2I, color: ColorRGBF) -> CPUShaderInput {
        CPUShaderInput {
            local_pos,
            size,
            color,
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct CPUShader(pub fn(CPUShaderInput) -> Option<ColorRGBF>);

impl CPUShader {
    fn get_uv(input: CPUShaderInput) -> (f32, f32) {
        let local_pos = input.local_pos;
        let size = input.size;
        let u = (local_pos.0 as f32) / (size.0 - 1) as f32;
        let v = (local_pos.1 as f32) / (size.1 - 1) as f32;

        (u, v)
    }

    pub fn uv(input: CPUShaderInput) -> Option<ColorRGBF> {
        let (u, v) = Self::get_uv(input);
        Some(ColorRGB(u, v, 0.0))
    }

    pub fn gradient_horizontal(input: CPUShaderInput) -> Option<ColorRGBF> {
        let (u, _) = Self::get_uv(input);
        Some(input.color * u)
    }

    pub fn gradient_vertical(input: CPUShaderInput) -> Option<ColorRGBF> {
        let (_, v) = Self::get_uv(input);
        Some(input.color * v)
    }

    pub fn hsv(input: CPUShaderInput) -> Option<ColorRGBF> {
        let (u, v) = Self::get_uv(input);

        let hue = v * 360.0;

        let norm = (u - 0.5) * -2.0;
        let sat = 1.0 - norm.min(0.0).abs();
        let val = 1.0 - norm.max(0.0);

        Some(ColorRGB::from_hsv(hue, sat, val))
    }

    pub fn rect(input: CPUShaderInput) -> Option<ColorRGBF> {
        let local_pos = input.local_pos;
        let size = input.size;

        if local_pos.0 > 0
            && local_pos.0 < size.0 - 1
            && local_pos.1 > 0
            && local_pos.1 < size.1 - 1
        {
            return None;
        }

        Self::color_passthrough(input)
    }

    pub fn color_passthrough(input: CPUShaderInput) -> Option<ColorRGBF> {
        Some(input.color)
    }
}
