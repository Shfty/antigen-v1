use std::ops::{Add, AddAssign};

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
