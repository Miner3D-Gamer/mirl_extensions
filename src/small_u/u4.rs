#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]

use super::U2;
#[cfg(not(feature = "std"))]
#[allow(unused_imports)]
use crate::*;
use crate::math::{ConstOne, ConstZero};

#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
/// A custom u4
#[allow(missing_docs)]
#[allow(clippy::struct_excessive_bools)]
pub struct U4 {
    pub b0: bool,
    pub b1: bool,
    pub b2: bool,
    pub b3: bool,
}
#[cfg(feature = "num_traits")]
impl num_traits::ToPrimitive for U4 {
    fn to_f32(&self) -> Option<f32> {
        Some(self.value().into())
    }
    fn to_f64(&self) -> Option<f64> {
        Some(self.value().into())
    }
    fn to_i128(&self) -> Option<i128> {
        Some(self.value().into())
    }
    fn to_i16(&self) -> Option<i16> {
        Some(self.value().into())
    }
    fn to_i32(&self) -> Option<i32> {
        Some(self.value().into())
    }
    fn to_i64(&self) -> Option<i64> {
        Some(self.value().into())
    }
    fn to_i8(&self) -> Option<i8> {
        self.value().to_i8()
    }
    fn to_isize(&self) -> Option<isize> {
        Some(self.value().into())
    }
    fn to_u128(&self) -> Option<u128> {
        Some(self.value().into())
    }
    fn to_u16(&self) -> Option<u16> {
        Some(self.value().into())
    }
    fn to_u32(&self) -> Option<u32> {
        Some(self.value().into())
    }
    fn to_u64(&self) -> Option<u64> {
        Some(self.value().into())
    }
    fn to_u8(&self) -> Option<u8> {
        Some(self.value())
    }
}
#[cfg(feature = "num_traits")]
impl num_traits::NumCast for U4 {
    fn from<T: num_traits::ToPrimitive>(n: T) -> Option<Self> {
        Some(unsafe { Self::new(n.to_u8().unwrap_unchecked()) })
    }
}

impl U4 {
    #[must_use]
    /// Create a U4 from a u8
    ///
    /// # Panics
    /// If the value is not within 0..15
    pub fn new(val: u8) -> Self {
        assert!(val <= 0b1111, "Value out of range for U4 (must be 0..=15)");
        Self {
            b0: (val & 0b0001) != 0,
            b1: (val & 0b0010) != 0,
            b2: (val & 0b0100) != 0,
            b3: (val & 0b1000) != 0,
        }
    }
    #[must_use]
    /// Create a U4 without checking (masking to lower 4 bits)
    pub const fn from_u8_trunc(val: u8) -> Self {
        Self {
            b0: (val & 0b0001) != 0,
            b1: (val & 0b0010) != 0,
            b2: (val & 0b0100) != 0,
            b3: (val & 0b1000) != 0,
        }
    }
    #[must_use]
    /// Get the integer value of this U4
    pub const fn value(self) -> u8 {
        (self.b0 as u8)
            | ((self.b1 as u8) << 1)
            | ((self.b2 as u8) << 2)
            | ((self.b3 as u8) << 3)
    }
    #[must_use]
    /// Returns true if the value is zero
    pub const fn is_zero(self) -> bool {
        !self.b0 && !self.b1 && !self.b2 && !self.b3
    }
    #[must_use]
    /// Returns true if the value is the maximum (15)
    pub const fn is_max(self) -> bool {
        self.b0 && self.b1 && self.b2 && self.b3
    }
    #[must_use]
    /// Wrapping add: (self + other) mod 16
    pub const fn wrapping_add(self, other: Self) -> Self {
        Self::from_u8_trunc(self.value().wrapping_add(other.value()))
    }
    #[must_use]
    /// Wrapping sub: (self - other) mod 16
    pub const fn wrapping_sub(self, other: Self) -> Self {
        Self::from_u8_trunc(self.value().wrapping_sub(other.value()))
    }
    #[must_use]
    /// Convert to U2 by truncating to lower 2 bits
    pub fn to_u2(self) -> U2 {
        self.into()
    }
    #[must_use]
    /// Create a U4 from high and low U2 values
    /// The high U2 will occupy bits 2-3, and the low U2 will occupy bits 0-1
    pub const fn from_u2_pair(high: U2, low: U2) -> Self {
        Self {
            b0: low.b0,
            b1: low.b1,
            b2: high.b0,
            b3: high.b1,
        }
    }
    #[must_use]
    /// Split a U4 into high and low U2 values
    /// Returns (`high_bits`, `low_bits`) where `high_bits` are bits 2-3 and `low_bits` are bits 0-1
    pub const fn split_to_u2_pair(self) -> (U2, U2) {
        let high = U2 {
            b0: self.b2,
            b1: self.b3,
        };

        let low = U2 {
            b0: self.b0,
            b1: self.b1,
        };

        (high, low)
    }
}

// Bitwise and arithmetic traits
impl const core::ops::Not for U4 {
    type Output = Self;
    fn not(self) -> Self {
        Self::from_u8_trunc(!self.value())
    }
}

impl const core::ops::BitAnd for U4 {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self::from_u8_trunc(self.value() & rhs.value())
    }
}

impl const core::ops::BitOr for U4 {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self::from_u8_trunc(self.value() | rhs.value())
    }
}

impl const core::ops::BitXor for U4 {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self {
        Self::from_u8_trunc(self.value() ^ rhs.value())
    }
}

impl const core::ops::Shl<usize> for U4 {
    type Output = Self;
    fn shl(self, rhs: usize) -> Self {
        Self::from_u8_trunc(self.value() << rhs)
    }
}

impl const core::ops::Shr<usize> for U4 {
    type Output = Self;
    fn shr(self, rhs: usize) -> Self {
        Self::from_u8_trunc(self.value() >> rhs)
    }
}

impl const core::ops::Add for U4 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        self.wrapping_add(rhs)
    }
}

impl core::ops::Sub for U4 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        self.wrapping_sub(rhs)
    }
}

impl const core::ops::Mul for U4 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self::from_u8_trunc(self.value().wrapping_mul(rhs.value()))
    }
}

impl const core::ops::Div for U4 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Self::from_u8_trunc(self.value().wrapping_div(rhs.value()))
    }
}
/// Convert a number into a u4
#[macro_export]
macro_rules! u4 {
    ($val:expr) => {
        U4::new($val)
    };
}
macro_rules! impl_u4_conversion {
    ($($t:ty),* $(,)?) => {
        $(
            // From {type} -> U4
            impl From<$t> for U4 {
                fn from(val: $t) -> Self {
                    // Handle signed and unsigned differently
                    assert!((0..=15).contains(&val), "Value out of range for U4 (must be 0..=15)");
                    U4 {
                        b0: (val & 0b0001) != 0,
                        b1: (val & 0b0010) != 0,
                        b2: (val & 0b0100) != 0,
                        b3: (val & 0b1000) != 0,
                    }
                }
            }
#[cfg(feature = "num_traits")]
            // From U4 -> {type}
            impl From<U4> for $t {
                fn from(val: U4) -> Self {
                    let raw = (val.b0 as u8)
                        | ((val.b1 as u8) << 1)
                        | ((val.b2 as u8) << 2)
                        | ((val.b3 as u8) << 3);
                    unsafe{num_traits::NumCast::from(raw).unwrap_unchecked()}
                }
            }
        )*
    };
}

macro_rules! impl_u4_float_conversion {
    ($($f:ty),* $(,)?) => {
        $(
            impl From<$f> for U4 {
                fn from(val: $f) -> Self {
                    assert!(val.is_finite(), "Cannot convert non-finite float to U4");
                    assert!(val.fract() == 0.0, "Cannot convert non-integer float to U4");
                    let as_int = val as i128;
                    assert!((0..=15).contains(&as_int), "Float value out of U4 range (must be 0.0 to 15.0)");
                    let val = as_int as u8;
                    U4 {
                        b0: (val & 0b0001) != 0,
                        b1: (val & 0b0010) != 0,
                        b2: (val & 0b0100) != 0,
                        b3: (val & 0b1000) != 0,
                    }
                }
            }

            impl From<U4> for $f {
                fn from(val: U4) -> Self {
                    let raw = (val.b0 as u8)
                        | ((val.b1 as u8) << 1)
                        | ((val.b2 as u8) << 2)
                        | ((val.b3 as u8) << 3);
                    raw as $f
                }
            }
        )*
    };
}
impl_u4_conversion!(u8, u16, u32, u64, u128, usize);
impl_u4_conversion!(i8, i16, i32, i64, i128, isize);
impl_u4_float_conversion!(f32, f64);

impl core::ops::Rem for U4 {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self::from_u8_trunc(self.value() % rhs.value())
    }
}

#[cfg(feature = "num_traits")]
impl num_traits::One for U4 {
    fn is_one(&self) -> bool
    where
        Self: PartialEq,
    {
        self.value() == 1
    }
    fn one() -> Self {
        Self::from_u8_trunc(1)
    }
}

#[cfg(feature = "num_traits")]
impl num_traits::Zero for U4 {
    fn zero() -> Self {
        Self::from_u8_trunc(0)
    }
    fn is_zero(&self) -> bool {
        self.value() == 0
    }
}

#[cfg(feature = "num_traits")]
impl num_traits::Num for U4 {
    fn from_str_radix(
        str: &str,
        radix: u32,
    ) -> Result<Self, Self::FromStrRadixErr> {
        let result = <u8 as num_traits::Num>::from_str_radix(str, radix);
        match result {
            Ok(r) => Result::Ok(Self::from_u8_trunc(r)),
            Err(e) => Result::Err(e),
        }
    }

    type FromStrRadixErr = ::core::num::ParseIntError;
}

impl ConstOne for U4 {
    const ONE: Self = Self::from_u8_trunc(1);
}

impl ConstZero for U4 {
    const ZERO: Self = Self::from_u8_trunc(0);
}
