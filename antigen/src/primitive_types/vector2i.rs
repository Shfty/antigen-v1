use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Vector2I(pub i64, pub i64);

impl Vector2I {
    pub const ZERO: Vector2I = Vector2I(0, 0);
    pub const ONE: Vector2I = Vector2I(1, 1);
}

impl Add for Vector2I {
    type Output = Vector2I;

    fn add(self, rhs: Self) -> Self::Output {
        let Vector2I(self_x, self_y) = self;
        let Vector2I(rhs_x, rhs_y) = rhs;
        Vector2I(self_x + rhs_x, self_y + rhs_y)
    }
}

impl AddAssign for Vector2I {
    fn add_assign(&mut self, rhs: Self) {
        *self = Vector2I::add(*self, rhs);
    }
}

impl Sub for Vector2I {
    type Output = Vector2I;

    fn sub(self, rhs: Self) -> Self::Output {
        let Vector2I(self_x, self_y) = self;
        let Vector2I(rhs_x, rhs_y) = rhs;
        Vector2I(self_x - rhs_x, self_y - rhs_y)
    }
}

impl SubAssign for Vector2I {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Vector2I::sub(*self, rhs);
    }
}

impl Mul for Vector2I {
    type Output = Vector2I;

    fn mul(self, rhs: Self) -> Self::Output {
        let Vector2I(self_x, self_y) = self;
        let Vector2I(rhs_x, rhs_y) = rhs;
        Vector2I(self_x * rhs_x, self_y * rhs_y)
    }
}

impl MulAssign for Vector2I {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Vector2I::mul(*self, rhs);
    }
}

impl Div for Vector2I {
    type Output = Vector2I;

    fn div(self, rhs: Self) -> Self::Output {
        let Vector2I(self_x, self_y) = self;
        let Vector2I(rhs_x, rhs_y) = rhs;
        Vector2I(self_x / rhs_x, self_y / rhs_y)
    }
}

impl DivAssign for Vector2I {
    fn div_assign(&mut self, rhs: Self) {
        *self = Vector2I::div(*self, rhs);
    }
}
