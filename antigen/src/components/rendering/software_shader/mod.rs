mod raster_shader_rgbf;

pub use raster_shader_rgbf::*;

use std::{fmt::Debug, ops::Deref};

pub trait SoftwareShaderTrait<I, O>: Fn(I) -> Option<O> {}
impl<I, O, T> SoftwareShaderTrait<I, O> for T where T: Fn(I) -> Option<O> {}

pub struct SoftwareShader<I, O>(Box<dyn SoftwareShaderTrait<I, O>>);

impl<I, O> Debug for SoftwareShader<I, O> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("SoftwareShader").finish()
    }
}

impl<I, O> Deref for SoftwareShader<I, O> {
    type Target = Box<dyn SoftwareShaderTrait<I, O>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<I, O> SoftwareShader<I, O> {
    pub fn new<F>(f: F) -> Self
    where
        F: SoftwareShaderTrait<I, O> + 'static,
    {
        SoftwareShader(Box::new(f))
    }
}
