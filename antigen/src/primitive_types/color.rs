use std::{
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Color<T>(pub T, pub T, pub T)
where
    T: Copy + Clone + PartialOrd + PartialEq;

pub type ColorRGB8 = Color<u8>;
pub type ColorRGBF = Color<f32>;

impl ColorRGBF {
    pub fn square_length(&self) -> f32 {
        let Color(r, g, b) = self;
        r * r + g * g + b * b
    }

    pub fn length(&self) -> f32 {
        self.square_length().sqrt()
    }

    pub fn square_distance(lhs: &ColorRGBF, rhs: &ColorRGBF) -> f32 {
        (*rhs - *lhs).square_length()
    }

    pub fn distance(lhs: &ColorRGBF, rhs: &ColorRGBF) -> f32 {
        (*rhs - *lhs).length()
    }
}

impl From<ColorRGBF> for ColorRGB8 {
    fn from(color: ColorRGBF) -> Self {
        let Color(r, g, b) = color;
        Color((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
    }
}

impl From<ColorRGB8> for ColorRGBF {
    fn from(color: ColorRGB8) -> Self {
        let Color(r, g, b) = color;
        Color(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
    }
}

impl<T> Default for Color<T>
where
    T: Default + Copy + Clone + PartialOrd + PartialEq,
{
    fn default() -> Self {
        Color(T::default(), T::default(), T::default())
    }
}

impl<T> Add<Color<T>> for Color<T>
where
    T: Add<Output = T> + Copy + Clone + PartialOrd + PartialEq,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let Color(lr, lg, lb) = self;
        let Color(rr, rg, rb) = rhs;
        Color(lr + rr, lg + rg, lb + rb)
    }
}

impl<T> Add<T> for Color<T>
where
    T: Add<Output = T> + Copy + Clone + PartialOrd + PartialEq,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let Color(lr, lg, lb) = self;
        Color(lr + rhs, lg + rhs, lb + rhs)
    }
}

impl<T> AddAssign<Color<T>> for Color<T>
where
    T: Add<Output = T> + Copy + Clone + PartialOrd + PartialEq,
{
    fn add_assign(&mut self, rhs: Self) {
        *self = Color::add(*self, rhs)
    }
}

impl<T> Sub<Color<T>> for Color<T>
where
    T: Sub<Output = T> + Copy + Clone + PartialOrd + PartialEq,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let Color(lr, lg, lb) = self;
        let Color(rr, rg, rb) = rhs;
        Color(lr - rr, lg - rg, lb - rb)
    }
}

impl<T> Sub<T> for Color<T>
where
    T: Sub<Output = T> + Copy + Clone + PartialOrd + PartialEq,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        let Color(lr, lg, lb) = self;
        Color(lr - rhs, lg - rhs, lb - rhs)
    }
}

impl<T> SubAssign<Color<T>> for Color<T>
where
    T: Sub<Output = T> + Copy + Clone + PartialOrd + PartialEq,
{
    fn sub_assign(&mut self, rhs: Self) {
        *self = Color::sub(*self, rhs)
    }
}

impl<T> Mul<Color<T>> for Color<T>
where
    T: Mul<Output = T> + Copy + Clone + PartialOrd + PartialEq,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let Color(lr, lg, lb) = self;
        let Color(rr, rg, rb) = rhs;
        Color(lr * rr, lg * rg, lb * rb)
    }
}

impl<T> Mul<T> for Color<T>
where
    T: Mul<Output = T> + Copy + Clone + PartialOrd + PartialEq,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let Color(lr, lg, lb) = self;
        Color(lr * rhs, lg * rhs, lb * rhs)
    }
}

impl<T> MulAssign<Color<T>> for Color<T>
where
    T: Mul<Output = T> + Copy + Clone + PartialOrd + PartialEq,
{
    fn mul_assign(&mut self, rhs: Self) {
        *self = Color::mul(*self, rhs)
    }
}

impl<T> Div<Color<T>> for Color<T>
where
    T: Div<Output = T> + Copy + Clone + PartialOrd + PartialEq,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let Color(lr, lg, lb) = self;
        let Color(rr, rg, rb) = rhs;
        Color(lr / rr, lg / rg, lb / rb)
    }
}

impl<T> Div<T> for Color<T>
where
    T: Div<Output = T> + Copy + Clone + PartialOrd + PartialEq,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        let Color(lr, lg, lb) = self;
        Color(lr / rhs, lg / rhs, lb / rhs)
    }
}

impl<T> DivAssign<Color<T>> for Color<T>
where
    T: Div<Output = T> + Copy + Clone + PartialOrd + PartialEq,
{
    fn div_assign(&mut self, rhs: Self) {
        *self = Color::div(*self, rhs)
    }
}

impl<T> Sum for Color<T>
where
    T: Add<Output = T> + Default + Copy + Clone + PartialOrd + PartialEq,
{
    fn sum<I: Iterator<Item = Color<T>>>(iter: I) -> Self {
        iter.fold(Color::default(), Self::add)
    }
}
