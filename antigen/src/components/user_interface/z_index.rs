#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ZIndex(pub i64);

impl From<i64> for ZIndex {
    fn from(z: i64) -> Self {
        ZIndex(z)
    }
}

impl Into<i64> for ZIndex {
    fn into(self) -> i64 {
        self.0
    }
}
