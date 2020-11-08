mod rgb8;
mod rgbf;

pub use rgb8::*;
pub use rgbf::*;

use std::{
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use crate::core::palette::Palette;

use super::ColorIndex;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct ColorRGB<T>(pub T, pub T, pub T)
where
    T: Copy + Clone + PartialOrd + PartialEq;

impl<T> ColorRGB<T>
where
    T: Copy + Clone + PartialOrd + PartialEq + 'static,
{
    pub fn into_index<U>(self, palette: &U) -> ColorIndex
    where
        U: Palette<Color = T>,
    {
        palette.get_color_idx(self)
    }

    pub fn from_index<U>(index: ColorIndex, palette: &U) -> ColorRGB<T>
    where
        U: Palette<Color = T>,
    {
        palette.get_color(index)
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
