use crate::*;

// To implement a new number, one must implement these traits:
// ConstOne (One)
// ConstZero (Zero)
// Abs

impl ConstZero for num_bigint::BigInt {
    const ZERO: Self = Self::ZERO;
}

impl Abs for num_bigint::BigInt {
    fn abs(self) -> Self {
        if self > Self::ZERO {
            self
        } else {
            -self
        }
    }
}
