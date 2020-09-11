use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

pub type UID = i64;

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct IVector2(pub i64, pub i64);

impl Add for IVector2 {
    type Output = IVector2;

    fn add(self, rhs: Self) -> Self::Output {
        let IVector2(self_x, self_y) = self;
        let IVector2(rhs_x, rhs_y) = rhs;
        IVector2(self_x + rhs_x, self_y + rhs_y)
    }
}

impl AddAssign for IVector2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = IVector2::add(*self, rhs);
    }
}

impl Sub for IVector2 {
    type Output = IVector2;

    fn sub(self, rhs: Self) -> Self::Output {
        let IVector2(self_x, self_y) = self;
        let IVector2(rhs_x, rhs_y) = rhs;
        IVector2(self_x - rhs_x, self_y - rhs_y)
    }
}

impl SubAssign for IVector2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = IVector2::sub(*self, rhs);
    }
}

impl Mul for IVector2 {
    type Output = IVector2;

    fn mul(self, rhs: Self) -> Self::Output {
        let IVector2(self_x, self_y) = self;
        let IVector2(rhs_x, rhs_y) = rhs;
        IVector2(self_x * rhs_x, self_y * rhs_y)
    }
}

impl MulAssign for IVector2 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = IVector2::mul(*self, rhs);
    }
}

impl Div for IVector2 {
    type Output = IVector2;

    fn div(self, rhs: Self) -> Self::Output {
        let IVector2(self_x, self_y) = self;
        let IVector2(rhs_x, rhs_y) = rhs;
        IVector2(self_x / rhs_x, self_y / rhs_y)
    }
}

impl DivAssign for IVector2 {
    fn div_assign(&mut self, rhs: Self) {
        *self = IVector2::div(*self, rhs);
    }
}
