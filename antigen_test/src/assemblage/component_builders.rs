use antigen::{
    assemblage::ComponentBuilder, components::Name, components::SoftwareRasterFramebuffer,
    primitive_types::ColorRGB, primitive_types::Vector2I,
};

pub fn color_framebuffer(builder: ComponentBuilder) -> ComponentBuilder {
    builder.fields((
        Name("Color Framebuffer".into()),
        SoftwareRasterFramebuffer::new(Vector2I::default(), ColorRGB(0.0f32, 0.0f32, 0.0f32)),
        SoftwareRasterFramebuffer::new(Vector2I::default(), -1i64),
    ))
}

pub fn string_framebuffer(builder: ComponentBuilder) -> ComponentBuilder {
    builder.fields((
        Name("String Framebuffer".into()),
        SoftwareRasterFramebuffer::new(Vector2I::default(), ' '),
        SoftwareRasterFramebuffer::new(Vector2I::default(), -1i64),
    ))
}
