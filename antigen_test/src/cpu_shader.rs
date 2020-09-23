use antigen::{
    components::PrimitiveComponent,
    primitive_types::{ColorRGB, Vector2I},
};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct CPUShaderInput {
    local_pos: Vector2I,
    size: Vector2I,
    color: ColorRGB,
}

impl CPUShaderInput {
    pub fn new(local_pos: Vector2I, size: Vector2I, color: ColorRGB) -> CPUShaderInput {
        CPUShaderInput {
            local_pos,
            size,
            color,
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct CPUShader(pub fn(CPUShaderInput) -> Option<ColorRGB>);

pub type CPUShaderComponent = PrimitiveComponent<CPUShader>;

impl CPUShader {
    pub fn uv(input: CPUShaderInput) -> Option<ColorRGB> {
        let local_pos = input.local_pos;
        let size = input.size;

        let u = local_pos.0 as f32 / size.0 as f32;
        let v = local_pos.1 as f32 / size.1 as f32;
        Some(ColorRGB(u, v, 0.0))
    }

    pub fn rect(input: CPUShaderInput) -> Option<ColorRGB> {
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

    pub fn color_passthrough(input: CPUShaderInput) -> Option<ColorRGB> {
        Some(input.color)
    }
}
