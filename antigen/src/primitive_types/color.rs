use std::{
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct ColorRGB<T>(pub T, pub T, pub T)
where
    T: Copy + Clone + PartialOrd + PartialEq;

pub type ColorRGB8 = ColorRGB<u8>;
pub type ColorRGBF = ColorRGB<f32>;

impl ColorRGBF {
    pub fn distance(lhs: &ColorRGBF, rhs: &ColorRGBF) -> f32 {
        let rmean = (lhs.0 + rhs.0) / 2.0;

        let r = lhs.0 - rhs.0;
        let g = lhs.1 - rhs.1;
        let b = lhs.2 - rhs.2;

        let square =
            (2.0 + rmean) * r.powi(2) + 4.0 * g.powi(2) + (2.0 + (1.0 - rmean)) * b.powi(2);

        square.sqrt()
    }

    pub fn from_hsv(h: f32, s: f32, v: f32) -> Self {
        let c: f32 = v * s; // Chroma
        let hue_prime = (h / 60.0) % 6.0;
        let x = c * (1.0 - ((hue_prime % 2.0) - 1.0).abs());
        let m = v - c;

        let mut r: f32;
        let mut g: f32;
        let mut b: f32;
        if 0.0 <= hue_prime && hue_prime < 1.0 {
            r = c;
            g = x;
            b = 0.0;
        } else if 1.0 <= hue_prime && hue_prime < 2.0 {
            r = x;
            g = c;
            b = 0.0;
        } else if 2.0 <= hue_prime && hue_prime < 3.0 {
            r = 0.0;
            g = c;
            b = x;
        } else if 3.0 <= hue_prime && hue_prime < 4.0 {
            r = 0.0;
            g = x;
            b = c;
        } else if 4.0 <= hue_prime && hue_prime < 5.0 {
            r = x;
            g = 0.0;
            b = c;
        } else if 5.0 <= hue_prime && hue_prime < 6.0 {
            r = c;
            g = 0.0;
            b = x;
        } else {
            r = 0.0;
            g = 0.0;
            b = 0.0;
        }

        r += m;
        g += m;
        b += m;

        ColorRGB(r, g, b)
    }

    pub fn hsv(&self) -> (f32, f32, f32) {
        let h: f32;
        let s: f32;
        let v: f32;

        let r = self.0;
        let g = self.1;
        let b = self.2;

        let cmax = r.max(g.max(b));
        let cmin = r.min(g.min(b));
        let diff = cmax - cmin;

        if cmax == cmin {
            h = 0.0;
        } else if cmax == r {
            h = (60.0 * ((g - b) / diff) + 360.0) % 360.0;
        } else if cmax == g {
            h = (60.0 * ((b - r) / diff) + 120.0) % 360.0;
        } else if cmax == b {
            h = (60.0 * ((r - g) / diff) + 240.0) % 360.0;
        } else {
            panic!("Failed to convert color {:?} to HSV", self);
        }

        if cmax == 0.0 {
            s = 0.0;
        } else {
            s = (diff / cmax) * 100.0;
        }

        v = cmax * 100.0;

        (h, s, v)
    }
}

impl From<ColorRGBF> for ColorRGB8 {
    fn from(color: ColorRGBF) -> Self {
        let ColorRGB(r, g, b) = color;
        ColorRGB((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
    }
}

impl From<ColorRGB8> for ColorRGBF {
    fn from(color: ColorRGB8) -> Self {
        let ColorRGB(r, g, b) = color;
        ColorRGB(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
    }
}

impl<T> Default for ColorRGB<T>
where
    T: Default + Copy + Clone + PartialOrd + PartialEq,
{
    fn default() -> Self {
        ColorRGB(T::default(), T::default(), T::default())
    }
}

impl<T> Add<ColorRGB<T>> for ColorRGB<T>
where
    T: Add<Output = T> + Copy + Clone + PartialOrd + PartialEq,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let ColorRGB(lr, lg, lb) = self;
        let ColorRGB(rr, rg, rb) = rhs;
        ColorRGB(lr + rr, lg + rg, lb + rb)
    }
}

impl<T> Add<T> for ColorRGB<T>
where
    T: Add<Output = T> + Copy + Clone + PartialOrd + PartialEq,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let ColorRGB(lr, lg, lb) = self;
        ColorRGB(lr + rhs, lg + rhs, lb + rhs)
    }
}

impl<T> AddAssign<ColorRGB<T>> for ColorRGB<T>
where
    T: Add<Output = T> + Copy + Clone + PartialOrd + PartialEq,
{
    fn add_assign(&mut self, rhs: Self) {
        *self = ColorRGB::add(*self, rhs)
    }
}

impl<T> Sub<ColorRGB<T>> for ColorRGB<T>
where
    T: Sub<Output = T> + Copy + Clone + PartialOrd + PartialEq,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let ColorRGB(lr, lg, lb) = self;
        let ColorRGB(rr, rg, rb) = rhs;
        ColorRGB(lr - rr, lg - rg, lb - rb)
    }
}

impl<T> Sub<T> for ColorRGB<T>
where
    T: Sub<Output = T> + Copy + Clone + PartialOrd + PartialEq,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        let ColorRGB(lr, lg, lb) = self;
        ColorRGB(lr - rhs, lg - rhs, lb - rhs)
    }
}

impl<T> SubAssign<ColorRGB<T>> for ColorRGB<T>
where
    T: Sub<Output = T> + Copy + Clone + PartialOrd + PartialEq,
{
    fn sub_assign(&mut self, rhs: Self) {
        *self = ColorRGB::sub(*self, rhs)
    }
}

impl<T> Mul<ColorRGB<T>> for ColorRGB<T>
where
    T: Mul<Output = T> + Copy + Clone + PartialOrd + PartialEq,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let ColorRGB(lr, lg, lb) = self;
        let ColorRGB(rr, rg, rb) = rhs;
        ColorRGB(lr * rr, lg * rg, lb * rb)
    }
}

impl<T> Mul<T> for ColorRGB<T>
where
    T: Mul<Output = T> + Copy + Clone + PartialOrd + PartialEq,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let ColorRGB(lr, lg, lb) = self;
        ColorRGB(lr * rhs, lg * rhs, lb * rhs)
    }
}

impl<T> MulAssign<ColorRGB<T>> for ColorRGB<T>
where
    T: Mul<Output = T> + Copy + Clone + PartialOrd + PartialEq,
{
    fn mul_assign(&mut self, rhs: Self) {
        *self = ColorRGB::mul(*self, rhs)
    }
}

impl<T> Div<ColorRGB<T>> for ColorRGB<T>
where
    T: Div<Output = T> + Copy + Clone + PartialOrd + PartialEq,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let ColorRGB(lr, lg, lb) = self;
        let ColorRGB(rr, rg, rb) = rhs;
        ColorRGB(lr / rr, lg / rg, lb / rb)
    }
}

impl<T> Div<T> for ColorRGB<T>
where
    T: Div<Output = T> + Copy + Clone + PartialOrd + PartialEq,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        let ColorRGB(lr, lg, lb) = self;
        ColorRGB(lr / rhs, lg / rhs, lb / rhs)
    }
}

impl<T> DivAssign<ColorRGB<T>> for ColorRGB<T>
where
    T: Div<Output = T> + Copy + Clone + PartialOrd + PartialEq,
{
    fn div_assign(&mut self, rhs: Self) {
        *self = ColorRGB::div(*self, rhs)
    }
}

impl<T> Sum for ColorRGB<T>
where
    T: Add<Output = T> + Default + Copy + Clone + PartialOrd + PartialEq,
{
    fn sum<I: Iterator<Item = ColorRGB<T>>>(iter: I) -> Self {
        iter.fold(ColorRGB::default(), Self::add)
    }
}
