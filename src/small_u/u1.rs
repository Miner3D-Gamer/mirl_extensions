#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]


#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// U1 - Just a fancy bool honestly
pub struct U1 {
    /// Yep, that's the bool right there
    pub b0: bool,
}

#[cfg(not(feature = "std"))]
#[allow(unused_imports)]
use crate::*;
use crate::math::{ConstOne, ConstZero};
#[cfg(feature = "num_traits")]
impl num_traits::ToPrimitive for U1 {
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
impl num_traits::NumCast for U1 {
    fn from<T: num_traits::ToPrimitive>(n: T) -> Option<Self> {
        Some(Self::new(unsafe { n.to_u8().unwrap_unchecked() }))
    }
}

impl U1 {
    /// Create a U1 from a u8
    ///
    /// # Panics
    /// If the value is not within 0..1
    #[must_use]
    pub fn new(val: u8) -> Self {
        assert!(val <= 0b1, "Value out of range for U1 (must be 0..=1)");
        Self {
            b0: (val & 0b1) != 0,
        }
    }

    /// Create a U1 without checking (masking to lowest bit)
    #[must_use]
    pub const fn from_u8_trunc(val: u8) -> Self {
        Self {
            b0: (val & 0b1) != 0,
        }
    }

    /// Get the integer value of this U1
    #[must_use]
    pub const fn value(self) -> u8 {
        self.b0 as u8
    }

    /// Returns true if the value is zero
    #[must_use]
    pub const fn is_zero(self) -> bool {
        !self.b0
    }

    /// Returns true if the value is the maximum (1)
    #[must_use]
    pub const fn is_max(self) -> bool {
        self.b0
    }

    /// Wrapping add: (self + other) mod 2
    #[must_use]
    pub const fn wrapping_add(self, other: Self) -> Self {
        Self::from_u8_trunc(self.value().wrapping_add(other.value()))
    }

    /// Wrapping sub: (self - other) mod 2
    #[must_use]
    pub const fn wrapping_sub(self, other: Self) -> Self {
        Self::from_u8_trunc(self.value().wrapping_sub(other.value()))
    }

    /// Create a U1 directly from a boolean
    #[must_use]
    pub const fn from_bool(val: bool) -> Self {
        Self {
            b0: val,
        }
    }

    /// Convert to boolean
    #[must_use]
    pub const fn to_bool(self) -> bool {
        self.b0
    }
}

// Bitwise and arithmetic traits
impl const core::ops::Not for U1 {
    type Output = Self;
    fn not(self) -> Self {
        Self::from_u8_trunc(!self.value())
    }
}

impl const core::ops::BitAnd for U1 {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self::from_u8_trunc(self.value() & rhs.value())
    }
}

impl const core::ops::BitOr for U1 {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self::from_u8_trunc(self.value() | rhs.value())
    }
}

impl const core::ops::BitXor for U1 {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self {
        Self::from_u8_trunc(self.value() ^ rhs.value())
    }
}

impl const core::ops::Shl<usize> for U1 {
    type Output = Self;
    fn shl(self, rhs: usize) -> Self {
        Self::from_u8_trunc(self.value() << rhs)
    }
}

impl const core::ops::Shr<usize> for U1 {
    type Output = Self;
    fn shr(self, rhs: usize) -> Self {
        Self::from_u8_trunc(self.value() >> rhs)
    }
}

impl const core::ops::Add for U1 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        self.wrapping_add(rhs)
    }
}

impl const core::ops::Sub for U1 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        self.wrapping_sub(rhs)
    }
}

impl const core::ops::Mul for U1 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self::from_u8_trunc(self.value().wrapping_mul(rhs.value()))
    }
}

impl const core::ops::Div for U1 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Self::from_u8_trunc(self.value().wrapping_div(rhs.value()))
    }
}

// Direct conversion between U1 and bool
impl const From<bool> for U1 {
    fn from(val: bool) -> Self {
        Self {
            b0: val,
        }
    }
}

impl const From<U1> for bool {
    fn from(val: U1) -> Self {
        val.b0
    }
}
/// Convert a number into a u1
#[macro_export]
macro_rules! u1 {
    ($val:expr) => {
        Self::new($val)
    };
}
macro_rules! impl_u1_conversion {
    ($($t:ty),* $(,)?) => {
        $(
            // From {type} -> U1
            impl From<$t> for U1 {
                fn from(val: $t) -> Self {
                    // Handle signed and unsigned differently
                    assert!((0..=1).contains(&val), "Value out of range for U1 (must be 0..=1)");
                    Self {
                        b0: (val & 0b1) != 0,
                    }
                }
            }
#[cfg(feature = "num_traits")]
            // From U1 -> {type}
            impl From<U1> for $t {
                fn from(val: U1) -> Self {
                    let raw = val.value();
                    unsafe{
                    num_traits::NumCast::from(raw).unwrap_unchecked()}
                }
            }
        )*
    };
}

macro_rules! impl_u1_float_conversion {
    ($($f:ty),* $(,)?) => {
        $(
            impl From<$f> for U1 {
                fn from(val: $f) -> Self {
                    assert!(val.is_finite(), "Cannot convert non-finite float to U1");
                    assert!(val.fract() == 0.0, "Cannot convert non-integer float to U1");
                    let as_int = val as i128;
                    assert!((0..=1).contains(&as_int), "Float value out of U1 range (must be 0.0 to 1.0)");
                    let val = as_int as u8;
                    U1 {
                        b0: (val & 0b1) != 0,
                    }
                }
            }

            impl From<U1> for $f {
                fn from(val: U1) -> Self {
                    let raw = val.b0 as u8;
                    raw as $f
                }
            }
        )*
    };
}
impl_u1_conversion!(u8, u16, u32, u64, u128, usize);
impl_u1_conversion!(i8, i16, i32, i64, i128, isize);
impl_u1_float_conversion!(f32, f64);

impl const core::ops::Rem for U1 {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self::from_u8_trunc(self.value() % rhs.value())
    }
}

#[cfg(feature = "num_traits")]
impl num_traits::One for U1 {
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
impl num_traits::Zero for U1 {
    fn zero() -> Self {
        Self::from_u8_trunc(0)
    }
    fn is_zero(&self) -> bool {
        self.value() == 0
    }
}

#[cfg(feature = "num_traits")]
impl num_traits::Num for U1 {
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
impl ConstOne for U1 {
    const ONE: Self = Self::from_u8_trunc(1);
}

impl ConstZero for U1 {
    const ZERO: Self = Self::from_u8_trunc(0);
}
