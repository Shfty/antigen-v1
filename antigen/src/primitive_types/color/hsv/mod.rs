mod hsv8;
mod hsvf;

pub use hsv8::*;
pub use hsvf::*;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct ColorHSV<T>(pub T, pub T, pub T)
where
    T: Copy + Clone + PartialOrd + PartialEq;
