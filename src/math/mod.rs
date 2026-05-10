#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_possible_truncation)]

#[cfg(feature = "bigfloat")]
mod bigfloat;
#[cfg(feature = "bigint")]
mod bigint;

// #[cfg(test)]
// /// A list of tests
// pub mod tests;

/// Implemented for types that support negative numbers
pub const trait SupportsNegative {}
impl_empty_trait!(SupportsNegative for i8, i16, i32, i64, i128, f16, f32, f64, f128);

/// The evil version of [`ConstOne`]
pub trait ConstNegativeOne {
    /// The value of -1 for the respective type
    const NEGATIVE_ONE: Self;
}
impl<
    T: ConstOne + ConstZero + SupportsNegative + const core::ops::Sub<Output = T>,
> ConstNegativeOne for T
{
    const NEGATIVE_ONE: Self = Self::ZERO - Self::ONE;
}

/// A custom [`ConstOne`] as [num-traits](crate::math::ConstOne) does not support f16 or f128
pub trait ConstOne {
    /// The value of 1 in the respective type
    const ONE: Self;
}
macro_rules! impl_const_one {
    ($t:ty, $v:expr) => {
        impl ConstOne for $t {
            const ONE: Self = $v;
        }
    };
}

impl_const_one!(usize, 1);
impl_const_one!(u8, 1);
impl_const_one!(u16, 1);
impl_const_one!(u32, 1);
impl_const_one!(u64, 1);
impl_const_one!(u128, 1);

impl_const_one!(isize, 1);
impl_const_one!(i8, 1);
impl_const_one!(i16, 1);
impl_const_one!(i32, 1);
impl_const_one!(i64, 1);
impl_const_one!(i128, 1);

impl_const_one!(f16, 1.0);
impl_const_one!(f32, 1.0);
impl_const_one!(f64, 1.0);
impl_const_one!(f128, 1.0);

/// A custom [`ConstZero`] as [num-traits](crate::math::ConstZero) does not support f16 or f128
pub const trait ConstZero {
    /// The value of 0 in the respective type
    const ZERO: Self;
}
macro_rules! impl_const_zero {
    ($t:ty, $v:expr) => {
        impl ConstZero for $t {
            const ZERO: Self = $v;
        }
    };
}

impl_const_zero!(usize, 0);
impl_const_zero!(u8, 0);
impl_const_zero!(u16, 0);
impl_const_zero!(u32, 0);
impl_const_zero!(u64, 0);
impl_const_zero!(u128, 0);

impl_const_zero!(isize, 0);
impl_const_zero!(i8, 0);
impl_const_zero!(i16, 0);
impl_const_zero!(i32, 0);
impl_const_zero!(i64, 0);
impl_const_zero!(i128, 0);

impl_const_zero!(f16, 0.0);
impl_const_zero!(f32, 0.0);
impl_const_zero!(f64, 0.0);
impl_const_zero!(f128, 0.0);

/// A trait that sets the current value to 0
pub const trait SetZero {
    /// Set the current value to 0
    fn set_zero(&mut self);
}
/// A trait that sets the current value to 1
pub const trait SetOne {
    /// Set the current value to 1
    fn set_one(&mut self);
}
impl<T: ConstZero + AutoImplTryFromPatch + [const] core::marker::Destruct> const
    SetZero for T
{
    fn set_zero(&mut self) {
        *self = Self::ZERO;
    }
}
impl<T: ConstOne + AutoImplTryFromPatch + [const] core::marker::Destruct> const
    SetOne for T
{
    fn set_one(&mut self) {
        *self = Self::ONE;
    }
}
/// Check if the value is zero
pub const trait IsZero {
    /// Check if the value is zero
    fn is_zero(&self) -> bool;
}
impl<T: [const] PartialEq + ConstZero + [const] core::marker::Destruct> const
    IsZero for T
{
    fn is_zero(&self) -> bool {
        Self::ZERO.eq(self)
    }
}

/// The upper and lower bound of a value
pub const trait Bounded {
    /// The minimal value this type can represent
    const MIN: Self;
    /// The maximum value this type can represent
    const MAX: Self;
}

macro_rules! bounded_impl {
    ($t:ty, $min:expr, $max:expr) => {
        impl const Bounded for $t {
            const MIN: Self = $min;
            const MAX: Self = $max;
        }
    };
}

bounded_impl!(usize, usize::MIN, usize::MAX);
bounded_impl!(u8, u8::MIN, u8::MAX);
bounded_impl!(u16, u16::MIN, u16::MAX);
bounded_impl!(u32, u32::MIN, u32::MAX);
bounded_impl!(u64, u64::MIN, u64::MAX);
bounded_impl!(u128, u128::MIN, u128::MAX);

bounded_impl!(isize, isize::MIN, isize::MAX);
bounded_impl!(i8, i8::MIN, i8::MAX);
bounded_impl!(i16, i16::MIN, i16::MAX);
bounded_impl!(i32, i32::MIN, i32::MAX);
bounded_impl!(i64, i64::MIN, i64::MAX);
bounded_impl!(i128, i128::MIN, i128::MAX);

bounded_impl!(f32, f32::MIN, f32::MAX);
bounded_impl!(f64, f64::MIN, f64::MAX);
bounded_impl!(f128, f128::MIN, f128::MAX);
bounded_impl!(f16, f16::MIN, f16::MAX);

use mirl_core::{
    graphics::{ColorManipulation, rgba_to_u32},
    impl_empty_trait, impl_trait,
};

use crate::{AutoImplTryFromPatch, ConstRotationRatio, IntoPatch};

/// A helper trait for the people who are used to `.signum()`
pub const trait SigNum {
    /// Returns the sign of the number -> -1, 0, 1
    #[must_use]
    fn signum(self) -> Self;
}
impl<T: [const] Sign> const SigNum for T {
    fn signum(self) -> Self {
        self.sign()
    }
}

/// A trait for numbers to support `sign()`
pub const trait Sign {
    /// Returns the sign of the number -> -1, 0, 1
    #[must_use]
    fn sign(self) -> Self;
}

macro_rules! impl_sign_u {
    ($($t:ty),*) => {
        // See those two spaces between the `const` and `Sign`?
        $(impl const  Sign for $t {
            fn sign(self) -> Self {
                if self > 0 { 1  }
                else { 0  }
            }
        })*
    };
}
macro_rules! impl_sign {
    ($($t:ty),*) => {
        // See those two spaces between the `const` and `Sign`?
        $(impl const  Sign for $t {
            fn sign(self) -> Self {
                if self > 0 { 1  }
                else if self < 0  { -1 }
                else { 0  }
            }
        })*
    };
}
macro_rules! impl_sign_float {
    ($($t:ty),*) => {
        // See those two spaces between the `const` and `Sign`?
        $(impl const  Sign for $t {
            fn sign(self) -> Self {
                if self > 0.0 { 1.0  }
                else if self < 0.0  { -1.0 }
                else { 0.0  }
            }
        })*
    };
}

impl_sign!(i8, i16, i32, i64, i128, isize);
impl_sign_u!(u8, u16, u32, u64, u128, usize);
impl_sign_float!(f16, f32, f64, f128);

// use core::f32;
// use core::convert::TryFrom;

// pub fn from_u8<T: TryFrom<u8>>(value: u8) -> T {
//     T::try_from(value).ok().expect("constant u8 conversion failed")
// }

macro_rules! impl_sqrt {
    ($($t:ty),*) => {
        $(
            impl Sqrt for $t {
                fn sqrt(self) -> Self {
                    // There has to be a better way
                    core::f64::math::sqrt(self as f64) as Self
                }
            }
        )*
    };
}

impl_sqrt!(i8);
impl_sqrt!(i16);
impl_sqrt!(i32);
impl_sqrt!(i64);
impl_sqrt!(i128);
impl_sqrt!(isize);
impl_sqrt!(u8);
impl_sqrt!(u16);
impl_sqrt!(u32);
impl_sqrt!(u64);
impl_sqrt!(u128);
impl_sqrt!(usize);

// use crate::{U1, U2, U4};
/// Core trait: addition of a signed number to an unsigned number (returning)
pub const trait AddSignLogic<T> {
    /// Simple addition of the possibly negative number (returning)
    #[must_use]
    fn add_sign(&self, value: T) -> Self;

    /// Addition of the possibly negative number without over/underflowing (returning)
    #[must_use]
    fn saturated_add_sign(&self, value: T) -> Self;
}

/// Trait for mutating setters, automatically implemented using `AddSignLogic`
pub const trait AddSignSetter<T>: AddSignLogic<T> {
    /// Simple addition of the possibly negative number (mutating)
    fn set_add_sign(&mut self, value: T);

    /// Addition of the possibly negative number without over/underflowing (mutating)
    fn set_saturated_add_sign(&mut self, value: T);
}

impl<U, S> AddSignLogic<S> for U
where
    U: Copy
        + WrapOps
        + SaturatingAdd<Output = U>
        + SaturatingSub<Output = U>
        + core::ops::Add<U, Output = U>
        + Bounded
        + ConstZero
        + core::ops::Sub<U, Output = U>,
    S: Copy + core::cmp::PartialOrd + Abs + ConstZero + TryIntoPatch<U>,
{
    fn add_sign(&self, value: S) -> Self {
        if value >= S::ZERO {
            (value).try_into_value().map_or_else(
                || self.wrapping_add(U::MAX),
                |pos_val| self.wrapping_add(pos_val),
            )
        } else {
            (value.abs()).try_into_value().map_or_else(
                || self.wrapping_sub(U::MAX),
                |sub_val| self.wrapping_sub(sub_val),
            )
        }
    }

    fn saturated_add_sign(&self, value: S) -> Self {
        if value >= S::ZERO {
            (value)
                .try_into_value()
                .map_or_else(|| U::MAX, |pos_val| self.saturating_add(pos_val))
        } else {
            (value.abs())
                .try_into_value()
                .map_or_else(|| U::ZERO, |sub_val| self.saturating_sub(sub_val))
        }
    }
}

impl<U, S> AddSignSetter<S> for U
where
    U: AddSignLogic<S>,
{
    fn set_add_sign(&mut self, value: S) {
        *self = self.add_sign(value);
    }

    fn set_saturated_add_sign(&mut self, value: S) {
        *self = self.saturated_add_sign(value);
    }
}

/// Trait for mapping between signed and unsigned integer types
pub const trait MapToSign {
    /// Signed Version
    type Signed;
    /// Unsigned Version
    type Unsigned;

    /// Map from unsigned back to signed by flipping the sign bit
    fn map_non_sign_to_sign(self) -> Self::Signed;
}

/// Trait for mapping between signed and unsigned integer types
pub const trait MapToUnSign {
    /// Signed Version
    type Signed;
    /// Unsigned Version
    type Unsigned;

    /// Map from signed to unsigned by flipping the sign bit
    fn map_sign_to_non_sign(self) -> Self::Unsigned;
}
use crate::TryIntoPatch;
/// Macro to implement `SignMapping` for integer type pairs
macro_rules! impl_sign_mapping {
    ($signed:ty, $unsigned:ty) => {
        impl const MapToUnSign for $signed {
            type Signed = $signed;
            type Unsigned = $unsigned;

            #[allow(clippy::cast_sign_loss)]
            #[allow(trivial_numeric_casts)]
            fn map_sign_to_non_sign(self) -> Self::Unsigned {
                (self as Self::Unsigned).wrapping_add(
                    <$unsigned>::MAX / (<$unsigned>::ONE + <$unsigned>::ONE)
                        + <$unsigned>::ONE,
                )
            }
        }
        impl const MapToSign for $unsigned {
            type Signed = $signed;
            type Unsigned = $unsigned;

            #[allow(clippy::cast_possible_wrap)]
            #[allow(trivial_numeric_casts)]
            fn map_non_sign_to_sign(self) -> Self::Signed {
                self.wrapping_sub(
                    <$unsigned>::MAX / (<$unsigned>::ONE + <$unsigned>::ONE)
                        + <$unsigned>::ONE,
                ) as Self::Signed
            }
        }
    };
}

// Implement for all standard integer pairs
impl_sign_mapping!(i8, u8);
impl_sign_mapping!(i16, u16);
impl_sign_mapping!(i32, u32);
impl_sign_mapping!(i64, u64);
impl_sign_mapping!(i128, u128);
impl_sign_mapping!(isize, usize);
impl_sign_mapping!(f16, f16);
impl_sign_mapping!(f32, f32);
impl_sign_mapping!(f64, f64);
impl_sign_mapping!(f128, f128);
/// A unified trait providing wrapping arithmetic operations for all numeric types
pub const trait WrapOps {
    /// Wrapping addition. Computes `self + other`, wrapping around at the boundary of the type.
    #[must_use]
    fn wrapping_add(self, other: Self) -> Self;

    /// Wrapping subtraction. Computes `self - other`, wrapping around at the boundary of the type.
    #[must_use]
    fn wrapping_sub(self, other: Self) -> Self;

    /// Wrapping multiplication. Computes `self * other`, wrapping around at the boundary of the type.
    #[must_use]
    fn wrapping_mul(self, other: Self) -> Self;
}

macro_rules! impl_wrap_ops_int {
    ($($t:ty)*) => ($(
        impl const WrapOps for $t {
            #[inline]
            fn wrapping_add(self, other: Self) -> Self {
                <$t>::wrapping_add(self, other)
            }

            #[inline]
            fn wrapping_sub(self, other: Self) -> Self {
                <$t>::wrapping_sub(self, other)
            }

            #[inline]
            fn wrapping_mul(self, other: Self) -> Self {
                <$t>::wrapping_mul(self, other)
            }
        }
    )*)
}

macro_rules! impl_wrap_ops_float {
    ($($t:ty)*) => ($(
        impl const WrapOps for $t {
            #[inline]
            fn wrapping_add(self, other: Self) -> Self {
                self + other
            }

            #[inline]
            fn wrapping_sub(self, other: Self) -> Self {
                self - other
            }

            #[inline]
            fn wrapping_mul(self, other: Self) -> Self {
                self * other
            }
        }
    )*)
}

impl_wrap_ops_int! { i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }
impl_wrap_ops_float! {f16 f32 f64 f128}

// pub const trait Modular<Rhs = Self> {
//     type Output;
//     fn modular(&self, modulus: Rhs) -> Self::Output;
// }

// macro_rules! impl_modular_unsigned {
//     ($($t:ty),*) => {
//         $(
//             impl Modular for $t {
//                 type Output = $t;

//                 fn modular(&self, modulus: $t) -> $t {
//                     if modulus == 0 {
//                         panic!("Division by zero: modulus cannot be zero");
//                     }
//                     self % modulus
//                 }
//             }
//         )*
//     };
// }

// macro_rules! impl_modular_signed {
//     ($($t:ty),*) => {
//         $(
//             impl Modular for $t {
//                 type Output = $t;

//                 fn modular(&self, modulus: $t) -> $t {
//                     if modulus == 0 {
//                         panic!("Division by zero: modulus cannot be zero");
//                     }
//                     // Use rem_euclid to ensure positive remainder for negative numbers
//                     self.rem_euclid(modulus.abs())
//                 }
//             }
//         )*
//     };
// }

// macro_rules! impl_modular_float {
//     ($($t:ty),*) => {
//         $(
//             impl Modular for $t {
//                 type Output = $t;

//                 fn modular(&self, modulus: $t) -> $t {
//                     if modulus == 0.0 || modulus.is_nan() || self.is_nan() {
//                         panic!("Invalid modular operation: NaN or zero modulus");
//                     }
//                     // Use rem_euclid for consistent positive results
//                     self.rem_euclid(modulus.abs())
//                 }
//             }
//         )*
//     };
// }

// impl_modular_unsigned!(u8, u16, u32, u64, u128, usize, U4);
// impl_modular_signed!(i8, i16, i32, i64, i128, isize);
// impl_modular_float!(f32, f64);

/// Round a value to its nearest neighbor
pub const trait Round {
    #[must_use]
    /// Round the current value to its nearest neighbor and return the result
    fn round(self) -> Self;
}
impl const Round for f16 {
    fn round(self) -> Self {
        core::intrinsics::roundf16(self)
    }
}
impl const Round for f32 {
    fn round(self) -> Self {
        core::f32::math::round(self)
    }
}
impl const Round for f64 {
    fn round(self) -> Self {
        core::f64::math::round(self)
    }
}
impl const Round for f128 {
    fn round(self) -> Self {
        core::intrinsics::roundf128(self)
    }
}
impl const Round for u8 {
    fn round(self) -> Self {
        self
    }
}
impl const Round for u16 {
    fn round(self) -> Self {
        self
    }
}
impl const Round for u64 {
    fn round(self) -> Self {
        self
    }
}
impl const Round for u32 {
    fn round(self) -> Self {
        self
    }
}
impl const Round for u128 {
    fn round(self) -> Self {
        self
    }
}
impl const Round for usize {
    fn round(self) -> Self {
        self
    }
}

impl const Round for i8 {
    fn round(self) -> Self {
        self
    }
}
impl const Round for i16 {
    fn round(self) -> Self {
        self
    }
}
impl const Round for i64 {
    fn round(self) -> Self {
        self
    }
}
impl const Round for i32 {
    fn round(self) -> Self {
        self
    }
}
impl const Round for i128 {
    fn round(self) -> Self {
        self
    }
}
impl const Round for isize {
    fn round(self) -> Self {
        self
    }
}
/// Take the sqrt of the given value
pub const trait Sqrt {
    #[must_use]
    /// Take the sqrt of the given value and return the result
    fn sqrt(self) -> Self;
}
impl Sqrt for f16 {
    fn sqrt(self) -> Self {
        core::intrinsics::sqrtf16(self)
    }
}
impl Sqrt for f32 {
    fn sqrt(self) -> Self {
        core::f32::math::sqrt(self)
    }
}
impl Sqrt for f64 {
    fn sqrt(self) -> Self {
        core::f64::math::sqrt(self)
    }
}
impl Sqrt for f128 {
    fn sqrt(self) -> Self {
        core::intrinsics::sqrtf128(self)
    }
}

/// Take the abs of the given value
pub const trait Abs {
    #[must_use]
    /// Take the sqrt of the given value and return the result
    fn abs(self) -> Self;
}
impl const Abs for f16 {
    // #[cfg(all(not(target_env = "gnu")))]
    // fn abs(self) -> Self {
    //     Self::from_bits(self.to_bits() & !(1 << 15))
    // }
    fn abs(self) -> Self {
        core::intrinsics::fabs(self)
    }
}
impl const Abs for f32 {
    // #[cfg(all(not(target_env = "gnu"), not(target_env = "msvc")))]
    // fn abs(self) -> Self {
    //     Self::from_bits(self.to_bits() & !(1 << 31))
    // }
    fn abs(self) -> Self {
        core::intrinsics::fabs(self)
    }
}
impl const Abs for f64 {
    // #[cfg(all(not(target_env = "gnu"), not(target_env = "msvc")))]
    // fn abs(self) -> Self {
    //     Self::from_bits(self.to_bits() & !(1 << 63))
    // }
    fn abs(self) -> Self {
        core::intrinsics::fabs(self)
    }
}
impl const Abs for f128 {
    // #[cfg(all(not(target_env = "gnu")))]
    // fn abs(self) -> Self {
    //     Self::from_bits(self.to_bits() & !(1 << 127))
    // }
    fn abs(self) -> Self {
        core::intrinsics::fabs(self)
    }
}
impl const Abs for i8 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl const Abs for i16 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl const Abs for i32 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl const Abs for i64 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl const Abs for i128 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl const Abs for isize {
    fn abs(self) -> Self {
        self.abs()
    }
}
/// Find the next number that is divisible by two
pub const trait NextPowerOfTwo {
    /// Find the next number that is divisible by two
    #[must_use]
    fn next_power_of_two(self) -> Self;
}

impl const NextPowerOfTwo for u8 {
    fn next_power_of_two(self) -> Self {
        Self::next_power_of_two(self)
    }
}

impl const NextPowerOfTwo for u16 {
    fn next_power_of_two(self) -> Self {
        Self::next_power_of_two(self)
    }
}

impl const NextPowerOfTwo for u32 {
    fn next_power_of_two(self) -> Self {
        Self::next_power_of_two(self)
    }
}

impl const NextPowerOfTwo for u64 {
    fn next_power_of_two(self) -> Self {
        Self::next_power_of_two(self)
    }
}

impl const NextPowerOfTwo for u128 {
    fn next_power_of_two(self) -> Self {
        Self::next_power_of_two(self)
    }
}

impl const NextPowerOfTwo for usize {
    fn next_power_of_two(self) -> Self {
        Self::next_power_of_two(self)
    }
}

impl const NextPowerOfTwo for i8 {
    fn next_power_of_two(self) -> Self {
        self.unsigned_abs().next_power_of_two() as Self
    }
}

impl const NextPowerOfTwo for i16 {
    fn next_power_of_two(self) -> Self {
        self.unsigned_abs().next_power_of_two() as Self
    }
}

impl const NextPowerOfTwo for i32 {
    fn next_power_of_two(self) -> Self {
        self.unsigned_abs().next_power_of_two() as Self
    }
}

impl const NextPowerOfTwo for i64 {
    fn next_power_of_two(self) -> Self {
        self.unsigned_abs().next_power_of_two() as Self
    }
}

impl const NextPowerOfTwo for i128 {
    fn next_power_of_two(self) -> Self {
        self.unsigned_abs().next_power_of_two() as Self
    }
}

impl const NextPowerOfTwo for isize {
    fn next_power_of_two(self) -> Self {
        self.unsigned_abs().next_power_of_two() as Self
    }
}

impl NextPowerOfTwo for f16 {
    fn next_power_of_two(self) -> Self {
        if self <= 1.0 {
            1.0
        } else {
            self.log2().ceil().exp2()
        }
    }
}
impl NextPowerOfTwo for f32 {
    fn next_power_of_two(self) -> Self {
        if self <= 1.0 {
            1.0
        } else {
            self.log2().ceil().exp2()
        }
    }
}

impl NextPowerOfTwo for f64 {
    fn next_power_of_two(self) -> Self {
        if self <= 1.0 {
            1.0
        } else {
            self.log2().ceil().exp2()
        }
    }
}
impl NextPowerOfTwo for f128 {
    fn next_power_of_two(self) -> Self {
        if self <= 1.0 {
            1.0
        } else {
            self.log2().ceil().exp2()
        }
    }
}

/// Find the next number that is a power of two and return both the value and the exponent
pub const trait NextPowerOfTwoWithExponent {
    /// Find the next power of two and return (2^x, x)
    #[must_use]
    fn next_power_of_two_with_exponent(self) -> (Self, Self)
    where
        Self: core::marker::Sized;
}

impl const NextPowerOfTwoWithExponent for u8 {
    fn next_power_of_two_with_exponent(self) -> (Self, Self) {
        let power = Self::next_power_of_two(self);
        let exponent = power.trailing_zeros() as Self;
        (power, exponent)
    }
}

impl const NextPowerOfTwoWithExponent for u16 {
    fn next_power_of_two_with_exponent(self) -> (Self, Self) {
        let power = Self::next_power_of_two(self);
        let exponent = power.trailing_zeros() as Self;
        (power, exponent)
    }
}

impl const NextPowerOfTwoWithExponent for u32 {
    fn next_power_of_two_with_exponent(self) -> (Self, Self) {
        let power = Self::next_power_of_two(self);
        let exponent = power.trailing_zeros();
        (power, exponent)
    }
}

impl const NextPowerOfTwoWithExponent for u64 {
    fn next_power_of_two_with_exponent(self) -> (Self, Self) {
        let power = Self::next_power_of_two(self);
        let exponent = power.trailing_zeros() as Self;
        (power, exponent)
    }
}

impl const NextPowerOfTwoWithExponent for u128 {
    fn next_power_of_two_with_exponent(self) -> (Self, Self) {
        let power = Self::next_power_of_two(self);
        let exponent = power.trailing_zeros() as Self;
        (power, exponent)
    }
}

impl const NextPowerOfTwoWithExponent for usize {
    fn next_power_of_two_with_exponent(self) -> (Self, Self) {
        let power = Self::next_power_of_two(self);
        let exponent = power.trailing_zeros() as Self;
        (power, exponent)
    }
}

impl const NextPowerOfTwoWithExponent for i8 {
    fn next_power_of_two_with_exponent(self) -> (Self, Self) {
        let power = self.unsigned_abs().next_power_of_two() as Self;
        let exponent = power.unsigned_abs().trailing_zeros() as Self;
        (power, exponent)
    }
}

impl const NextPowerOfTwoWithExponent for i16 {
    fn next_power_of_two_with_exponent(self) -> (Self, Self) {
        let power = self.unsigned_abs().next_power_of_two() as Self;
        let exponent = power.unsigned_abs().trailing_zeros() as Self;
        (power, exponent)
    }
}

impl const NextPowerOfTwoWithExponent for i32 {
    fn next_power_of_two_with_exponent(self) -> (Self, Self) {
        let power = self.unsigned_abs().next_power_of_two() as Self;
        let exponent = power.unsigned_abs().trailing_zeros() as Self;
        (power, exponent)
    }
}

impl const NextPowerOfTwoWithExponent for i64 {
    fn next_power_of_two_with_exponent(self) -> (Self, Self) {
        let power = self.unsigned_abs().next_power_of_two() as Self;
        let exponent = power.unsigned_abs().trailing_zeros() as Self;
        (power, exponent)
    }
}

impl const NextPowerOfTwoWithExponent for i128 {
    fn next_power_of_two_with_exponent(self) -> (Self, Self) {
        let power = self.unsigned_abs().next_power_of_two() as Self;
        let exponent = power.unsigned_abs().trailing_zeros() as Self;
        (power, exponent)
    }
}

impl const NextPowerOfTwoWithExponent for isize {
    fn next_power_of_two_with_exponent(self) -> (Self, Self) {
        let power = self.unsigned_abs().next_power_of_two() as Self;
        let exponent = power.unsigned_abs().trailing_zeros() as Self;
        (power, exponent)
    }
}

impl NextPowerOfTwoWithExponent for f16 {
    fn next_power_of_two_with_exponent(self) -> (Self, Self) {
        let power = if self <= 1.0 {
            1.0
        } else {
            self.log2().ceil().exp2()
        };
        let exponent = power.log2();
        (power, exponent)
    }
}

impl NextPowerOfTwoWithExponent for f32 {
    fn next_power_of_two_with_exponent(self) -> (Self, Self) {
        let power = if self <= 1.0 {
            1.0
        } else {
            self.log2().ceil().exp2()
        };
        let exponent = power.log2();
        (power, exponent)
    }
}

impl NextPowerOfTwoWithExponent for f64 {
    fn next_power_of_two_with_exponent(self) -> (Self, Self) {
        let power = if self <= 1.0 {
            1.0
        } else {
            self.log2().ceil().exp2()
        };
        let exponent = power.log2();
        (power, exponent)
    }
}

impl NextPowerOfTwoWithExponent for f128 {
    fn next_power_of_two_with_exponent(self) -> (Self, Self) {
        let power = if self <= 1.0 {
            1.0
        } else {
            self.log2().ceil().exp2()
        };
        let exponent = power.log2();
        (power, exponent)
    }
}
/// Find the exponent x where 2^x is the next power of two
pub const trait NextPowerOfTwoExponent {
    /// Find the exponent x where 2^x is the next power of two
    #[must_use]
    fn next_power_of_two_exponent(self) -> Self;
}

impl const NextPowerOfTwoExponent for u8 {
    fn next_power_of_two_exponent(self) -> Self {
        Self::next_power_of_two(self).trailing_zeros() as Self
    }
}

impl const NextPowerOfTwoExponent for u16 {
    fn next_power_of_two_exponent(self) -> Self {
        Self::next_power_of_two(self).trailing_zeros() as Self
    }
}

impl const NextPowerOfTwoExponent for u32 {
    fn next_power_of_two_exponent(self) -> Self {
        Self::next_power_of_two(self).trailing_zeros()
    }
}

impl const NextPowerOfTwoExponent for u64 {
    fn next_power_of_two_exponent(self) -> Self {
        Self::next_power_of_two(self).trailing_zeros() as Self
    }
}

impl const NextPowerOfTwoExponent for u128 {
    fn next_power_of_two_exponent(self) -> Self {
        Self::next_power_of_two(self).trailing_zeros() as Self
    }
}

impl const NextPowerOfTwoExponent for usize {
    fn next_power_of_two_exponent(self) -> Self {
        Self::next_power_of_two(self).trailing_zeros() as Self
    }
}

impl const NextPowerOfTwoExponent for i8 {
    fn next_power_of_two_exponent(self) -> Self {
        self.unsigned_abs().next_power_of_two().trailing_zeros() as Self
    }
}

impl const NextPowerOfTwoExponent for i16 {
    fn next_power_of_two_exponent(self) -> Self {
        self.unsigned_abs().next_power_of_two().trailing_zeros() as Self
    }
}

impl const NextPowerOfTwoExponent for i32 {
    fn next_power_of_two_exponent(self) -> Self {
        self.unsigned_abs().next_power_of_two().trailing_zeros() as Self
    }
}

impl const NextPowerOfTwoExponent for i64 {
    fn next_power_of_two_exponent(self) -> Self {
        self.unsigned_abs().next_power_of_two().trailing_zeros() as Self
    }
}

impl const NextPowerOfTwoExponent for i128 {
    fn next_power_of_two_exponent(self) -> Self {
        self.unsigned_abs().next_power_of_two().trailing_zeros() as Self
    }
}

impl const NextPowerOfTwoExponent for isize {
    fn next_power_of_two_exponent(self) -> Self {
        self.unsigned_abs().next_power_of_two().trailing_zeros() as Self
    }
}

impl NextPowerOfTwoExponent for f16 {
    fn next_power_of_two_exponent(self) -> Self {
        let power = if self <= 1.0 {
            1.0
        } else {
            self.log2().ceil().exp2()
        };
        power.log2()
    }
}

impl NextPowerOfTwoExponent for f32 {
    fn next_power_of_two_exponent(self) -> Self {
        let power = if self <= 1.0 {
            1.0
        } else {
            self.log2().ceil().exp2()
        };
        power.log2()
    }
}

impl NextPowerOfTwoExponent for f64 {
    fn next_power_of_two_exponent(self) -> Self {
        let power = if self <= 1.0 {
            1.0
        } else {
            self.log2().ceil().exp2()
        };
        power.log2()
    }
}

impl NextPowerOfTwoExponent for f128 {
    fn next_power_of_two_exponent(self) -> Self {
        let power = if self <= 1.0 {
            1.0
        } else {
            self.log2().ceil().exp2()
        };
        power.log2()
    }
}

/// Calculate what 2^x will result in the given value
pub const trait Log2 {
    /// Calculate what 2^x will result in the given value
    #[must_use]
    fn log2(self) -> Self;
}

// TODO: Implement Log2 for all U and I types

impl Log2 for f16 {
    fn log2(self) -> Self {
        core::intrinsics::log2f16(self)
    }
}
impl Log2 for f32 {
    fn log2(self) -> Self {
        core::intrinsics::log2f32(self)
    }
}

impl Log2 for f64 {
    fn log2(self) -> Self {
        core::intrinsics::log2f64(self)
    }
}
impl Log2 for f128 {
    fn log2(self) -> Self {
        core::intrinsics::log2f128(self)
    }
}

/// Calculate what 10^x will result in the given value
pub const trait Log10 {
    /// Calculate what 10^x will result in the given value
    #[must_use]
    fn log10(self) -> Self;
}

// TODO: Implement Log10 for all U and I types

impl Log10 for f16 {
    fn log10(self) -> Self {
        core::intrinsics::log10f16(self)
    }
}
impl Log10 for f32 {
    fn log10(self) -> Self {
        core::intrinsics::log10f32(self)
    }
}

impl Log10 for f64 {
    fn log10(self) -> Self {
        core::intrinsics::log10f64(self)
    }
}
impl Log10 for f128 {
    fn log10(self) -> Self {
        core::intrinsics::log10f128(self)
    }
}

/// Calculate what e^x will result in the given value
pub const trait Log {
    /// Calculate what e^x will result in the given value
    #[must_use]
    fn log(self) -> Self;
}

// TODO: Implement Log for all U and I types

impl Log for f16 {
    fn log(self) -> Self {
        core::intrinsics::logf16(self)
    }
}
impl Log for f32 {
    fn log(self) -> Self {
        core::intrinsics::logf32(self)
    }
}

impl Log for f64 {
    fn log(self) -> Self {
        core::intrinsics::logf64(self)
    }
}
impl Log for f128 {
    fn log(self) -> Self {
        core::intrinsics::logf128(self)
    }
}

/// Calculate 2^x
pub const trait Exp2 {
    ///  Calculate 2^x
    #[must_use]
    fn exp2(self) -> Self;
}

// TODO: Implement Exp for all U and I types

impl Exp2 for f16 {
    fn exp2(self) -> Self {
        core::intrinsics::exp2f16(self)
    }
}
impl Exp2 for f32 {
    fn exp2(self) -> Self {
        core::intrinsics::exp2f32(self)
    }
}

impl Exp2 for f64 {
    fn exp2(self) -> Self {
        core::intrinsics::exp2f64(self)
    }
}
impl Exp2 for f128 {
    fn exp2(self) -> Self {
        core::intrinsics::exp2f128(self)
    }
}

/// ¯\_(ツ)_/¯
pub const trait Fract {
    /// ¯\_(ツ)_/¯
    #[must_use]
    fn fract(self) -> Self;
}

// TODO: Implement Fract for all U and I types

impl Fract for f16 {
    fn fract(self) -> Self {
        self - self.trunc()
    }
}
impl Fract for f32 {
    fn fract(self) -> Self {
        core::f32::math::fract(self)
    }
}

impl Fract for f64 {
    fn fract(self) -> Self {
        core::f64::math::fract(self)
    }
}
impl Fract for f128 {
    fn fract(self) -> Self {
        self - self.trunc()
    }
}

/// ¯\_(ツ)_/¯
pub const trait Trunc {
    /// ¯\_(ツ)_/¯
    #[must_use]
    fn trunc(self) -> Self;
}

// TODO: Implement Fract for all U and I types

impl Trunc for f16 {
    fn trunc(self) -> Self {
        core::intrinsics::truncf16(self)
    }
}
impl Trunc for f32 {
    fn trunc(self) -> Self {
        core::intrinsics::truncf32(self)
    }
}

impl Trunc for f64 {
    fn trunc(self) -> Self {
        core::intrinsics::truncf64(self)
    }
}
impl Trunc for f128 {
    fn trunc(self) -> Self {
        core::intrinsics::truncf128(self)
    }
}

/// Saturating addition
pub const trait SaturatingAdd<Rhs = Self> {
    /// The resulting type after applying saturating addition
    type Output;

    #[must_use]
    /// Add two values, saturating at the numeric bounds instead of overflowing
    fn saturating_add(self, rhs: Rhs) -> Self::Output;
}

impl const SaturatingAdd for i8 {
    type Output = Self;
    fn saturating_add(self, rhs: Self) -> Self::Output {
        self.saturating_add(rhs)
    }
}

impl const SaturatingAdd for i16 {
    type Output = Self;
    fn saturating_add(self, rhs: Self) -> Self::Output {
        self.saturating_add(rhs)
    }
}

impl const SaturatingAdd for i32 {
    type Output = Self;
    fn saturating_add(self, rhs: Self) -> Self::Output {
        self.saturating_add(rhs)
    }
}

impl const SaturatingAdd for i64 {
    type Output = Self;
    fn saturating_add(self, rhs: Self) -> Self::Output {
        self.saturating_add(rhs)
    }
}

impl const SaturatingAdd for i128 {
    type Output = Self;
    fn saturating_add(self, rhs: Self) -> Self::Output {
        self.saturating_add(rhs)
    }
}

impl const SaturatingAdd for isize {
    type Output = Self;
    fn saturating_add(self, rhs: Self) -> Self::Output {
        self.saturating_add(rhs)
    }
}

impl const SaturatingAdd for u8 {
    type Output = Self;
    fn saturating_add(self, rhs: Self) -> Self::Output {
        self.saturating_add(rhs)
    }
}

impl const SaturatingAdd for u16 {
    type Output = Self;
    fn saturating_add(self, rhs: Self) -> Self::Output {
        self.saturating_add(rhs)
    }
}

impl const SaturatingAdd for u32 {
    type Output = Self;
    fn saturating_add(self, rhs: Self) -> Self::Output {
        self.saturating_add(rhs)
    }
}

impl const SaturatingAdd for u64 {
    type Output = Self;
    fn saturating_add(self, rhs: Self) -> Self::Output {
        self.saturating_add(rhs)
    }
}

impl const SaturatingAdd for u128 {
    type Output = Self;
    fn saturating_add(self, rhs: Self) -> Self::Output {
        self.saturating_add(rhs)
    }
}

impl const SaturatingAdd for usize {
    type Output = Self;
    fn saturating_add(self, rhs: Self) -> Self::Output {
        self.saturating_add(rhs)
    }
}

/// Saturating subtraction
pub const trait SaturatingSub<Rhs = Self> {
    /// The resulting type after applying saturating subtraction
    type Output;

    #[must_use]
    /// Subtract two values, saturating at the numeric bounds instead of overflowing
    fn saturating_sub(self, rhs: Rhs) -> Self::Output;
}

impl const SaturatingSub for i8 {
    type Output = Self;
    fn saturating_sub(self, rhs: Self) -> Self::Output {
        self.saturating_sub(rhs)
    }
}

impl const SaturatingSub for i16 {
    type Output = Self;
    fn saturating_sub(self, rhs: Self) -> Self::Output {
        self.saturating_sub(rhs)
    }
}

impl const SaturatingSub for i32 {
    type Output = Self;
    fn saturating_sub(self, rhs: Self) -> Self::Output {
        self.saturating_sub(rhs)
    }
}

impl const SaturatingSub for i64 {
    type Output = Self;
    fn saturating_sub(self, rhs: Self) -> Self::Output {
        self.saturating_sub(rhs)
    }
}

impl const SaturatingSub for i128 {
    type Output = Self;
    fn saturating_sub(self, rhs: Self) -> Self::Output {
        self.saturating_sub(rhs)
    }
}

impl const SaturatingSub for isize {
    type Output = Self;
    fn saturating_sub(self, rhs: Self) -> Self::Output {
        self.saturating_sub(rhs)
    }
}

impl const SaturatingSub for u8 {
    type Output = Self;
    fn saturating_sub(self, rhs: Self) -> Self::Output {
        self.saturating_sub(rhs)
    }
}

impl const SaturatingSub for u16 {
    type Output = Self;
    fn saturating_sub(self, rhs: Self) -> Self::Output {
        self.saturating_sub(rhs)
    }
}

impl const SaturatingSub for u32 {
    type Output = Self;
    fn saturating_sub(self, rhs: Self) -> Self::Output {
        self.saturating_sub(rhs)
    }
}

impl const SaturatingSub for u64 {
    type Output = Self;
    fn saturating_sub(self, rhs: Self) -> Self::Output {
        self.saturating_sub(rhs)
    }
}

impl const SaturatingSub for u128 {
    type Output = Self;
    fn saturating_sub(self, rhs: Self) -> Self::Output {
        self.saturating_sub(rhs)
    }
}

impl const SaturatingSub for usize {
    type Output = Self;
    fn saturating_sub(self, rhs: Self) -> Self::Output {
        self.saturating_sub(rhs)
    }
}

/// Saturating multiplication
pub const trait SaturatingMul<Rhs = Self> {
    /// The resulting type after applying saturating multiplication
    type Output;

    #[must_use]
    /// Multiply two values, saturating at the numeric bounds instead of overflowing
    fn saturating_mul(self, rhs: Rhs) -> Self::Output;
}

impl const SaturatingMul for i8 {
    type Output = Self;
    fn saturating_mul(self, rhs: Self) -> Self::Output {
        self.saturating_mul(rhs)
    }
}

impl const SaturatingMul for i16 {
    type Output = Self;
    fn saturating_mul(self, rhs: Self) -> Self::Output {
        self.saturating_mul(rhs)
    }
}

impl const SaturatingMul for i32 {
    type Output = Self;
    fn saturating_mul(self, rhs: Self) -> Self::Output {
        self.saturating_mul(rhs)
    }
}

impl const SaturatingMul for i64 {
    type Output = Self;
    fn saturating_mul(self, rhs: Self) -> Self::Output {
        self.saturating_mul(rhs)
    }
}

impl const SaturatingMul for i128 {
    type Output = Self;
    fn saturating_mul(self, rhs: Self) -> Self::Output {
        self.saturating_mul(rhs)
    }
}

impl const SaturatingMul for isize {
    type Output = Self;
    fn saturating_mul(self, rhs: Self) -> Self::Output {
        self.saturating_mul(rhs)
    }
}

impl const SaturatingMul for u8 {
    type Output = Self;
    fn saturating_mul(self, rhs: Self) -> Self::Output {
        self.saturating_mul(rhs)
    }
}

impl const SaturatingMul for u16 {
    type Output = Self;
    fn saturating_mul(self, rhs: Self) -> Self::Output {
        self.saturating_mul(rhs)
    }
}

impl const SaturatingMul for u32 {
    type Output = Self;
    fn saturating_mul(self, rhs: Self) -> Self::Output {
        self.saturating_mul(rhs)
    }
}

impl const SaturatingMul for u64 {
    type Output = Self;
    fn saturating_mul(self, rhs: Self) -> Self::Output {
        self.saturating_mul(rhs)
    }
}

impl const SaturatingMul for u128 {
    type Output = Self;
    fn saturating_mul(self, rhs: Self) -> Self::Output {
        self.saturating_mul(rhs)
    }
}

impl const SaturatingMul for usize {
    type Output = Self;
    fn saturating_mul(self, rhs: Self) -> Self::Output {
        self.saturating_mul(rhs)
    }
}

/// Saturating absolute value
pub const trait SaturatingAbs {
    /// The resulting type after applying saturating absolute value
    type Output;

    #[must_use]
    /// Take the absolute value, saturating at the numeric bounds instead of overflowing
    fn saturating_abs(self) -> Self::Output;
}

impl const SaturatingAbs for i8 {
    type Output = Self;
    fn saturating_abs(self) -> Self::Output {
        self.saturating_abs()
    }
}

impl const SaturatingAbs for i16 {
    type Output = Self;
    fn saturating_abs(self) -> Self::Output {
        self.saturating_abs()
    }
}

impl const SaturatingAbs for i32 {
    type Output = Self;
    fn saturating_abs(self) -> Self::Output {
        self.saturating_abs()
    }
}

impl const SaturatingAbs for i64 {
    type Output = Self;
    fn saturating_abs(self) -> Self::Output {
        self.saturating_abs()
    }
}

impl const SaturatingAbs for i128 {
    type Output = Self;
    fn saturating_abs(self) -> Self::Output {
        self.saturating_abs()
    }
}

impl const SaturatingAbs for isize {
    type Output = Self;
    fn saturating_abs(self) -> Self::Output {
        self.saturating_abs()
    }
}

/// Saturating negation
pub const trait SaturatingNeg {
    /// The resulting type after applying saturating negation
    type Output;

    #[must_use]
    /// Negate the value, saturating at the numeric bounds instead of overflowing
    fn saturating_neg(self) -> Self::Output;
}

impl const SaturatingNeg for i8 {
    type Output = Self;
    fn saturating_neg(self) -> Self::Output {
        self.saturating_neg()
    }
}

impl const SaturatingNeg for i16 {
    type Output = Self;
    fn saturating_neg(self) -> Self::Output {
        self.saturating_neg()
    }
}

impl const SaturatingNeg for i32 {
    type Output = Self;
    fn saturating_neg(self) -> Self::Output {
        self.saturating_neg()
    }
}

impl const SaturatingNeg for i64 {
    type Output = Self;
    fn saturating_neg(self) -> Self::Output {
        self.saturating_neg()
    }
}

impl const SaturatingNeg for i128 {
    type Output = Self;
    fn saturating_neg(self) -> Self::Output {
        self.saturating_neg()
    }
}

impl const SaturatingNeg for isize {
    type Output = Self;
    fn saturating_neg(self) -> Self::Output {
        self.saturating_neg()
    }
}
/// Check if a number is a power of two
pub const trait IsPowerOfTwo {
    /// Returns true if the number is a power of two
    #[must_use]
    #[allow(clippy::wrong_self_convention)]
    fn is_power_of_two(self) -> bool;
}

impl const IsPowerOfTwo for u8 {
    fn is_power_of_two(self) -> bool {
        Self::is_power_of_two(self)
    }
}

impl const IsPowerOfTwo for u16 {
    fn is_power_of_two(self) -> bool {
        Self::is_power_of_two(self)
    }
}

impl const IsPowerOfTwo for u32 {
    fn is_power_of_two(self) -> bool {
        Self::is_power_of_two(self)
    }
}

impl const IsPowerOfTwo for u64 {
    fn is_power_of_two(self) -> bool {
        Self::is_power_of_two(self)
    }
}

impl const IsPowerOfTwo for u128 {
    fn is_power_of_two(self) -> bool {
        Self::is_power_of_two(self)
    }
}

impl const IsPowerOfTwo for usize {
    fn is_power_of_two(self) -> bool {
        Self::is_power_of_two(self)
    }
}

impl const IsPowerOfTwo for i8 {
    fn is_power_of_two(self) -> bool {
        self > 0 && self.unsigned_abs().is_power_of_two()
    }
}

impl const IsPowerOfTwo for i16 {
    fn is_power_of_two(self) -> bool {
        self > 0 && self.unsigned_abs().is_power_of_two()
    }
}

impl const IsPowerOfTwo for i32 {
    fn is_power_of_two(self) -> bool {
        self > 0 && self.unsigned_abs().is_power_of_two()
    }
}

impl const IsPowerOfTwo for i64 {
    fn is_power_of_two(self) -> bool {
        self > 0 && self.unsigned_abs().is_power_of_two()
    }
}

impl const IsPowerOfTwo for i128 {
    fn is_power_of_two(self) -> bool {
        self > 0 && self.unsigned_abs().is_power_of_two()
    }
}

impl const IsPowerOfTwo for isize {
    fn is_power_of_two(self) -> bool {
        self > 0 && self.unsigned_abs().is_power_of_two()
    }
}

impl IsPowerOfTwo for f16 {
    #[allow(clippy::float_cmp)]
    fn is_power_of_two(self) -> bool {
        self > 0.0 && self.is_finite() && self.log2().fract() == 0.0
    }
}

impl IsPowerOfTwo for f32 {
    fn is_power_of_two(self) -> bool {
        self > 0.0 && self.is_finite() && self.log2().fract() == 0.0
    }
}

impl IsPowerOfTwo for f64 {
    fn is_power_of_two(self) -> bool {
        self > 0.0 && self.is_finite() && self.log2().fract() == 0.0
    }
}

impl IsPowerOfTwo for f128 {
    #[allow(clippy::float_cmp)]
    fn is_power_of_two(self) -> bool {
        self > 0.0 && self.is_finite() && self.log2().fract() == 0.0
    }
}

/// Get the previous power of two (rounding down)
pub const trait PrevPowerOfTwo {
    /// Find the previous power of two (the largest power of two less than or equal to self)
    #[must_use]
    fn prev_power_of_two(self) -> Self;
}

impl const PrevPowerOfTwo for u8 {
    fn prev_power_of_two(self) -> Self {
        if self == 0 {
            0
        } else {
            1 << (Self::BITS - 1 - self.leading_zeros())
        }
    }
}

impl const PrevPowerOfTwo for u16 {
    fn prev_power_of_two(self) -> Self {
        if self == 0 {
            0
        } else {
            1 << (Self::BITS - 1 - self.leading_zeros())
        }
    }
}

impl const PrevPowerOfTwo for u32 {
    fn prev_power_of_two(self) -> Self {
        if self == 0 {
            0
        } else {
            1 << (Self::BITS - 1 - self.leading_zeros())
        }
    }
}

impl const PrevPowerOfTwo for u64 {
    fn prev_power_of_two(self) -> Self {
        if self == 0 {
            0
        } else {
            1 << (Self::BITS - 1 - self.leading_zeros())
        }
    }
}

impl const PrevPowerOfTwo for u128 {
    fn prev_power_of_two(self) -> Self {
        if self == 0 {
            0
        } else {
            1 << (Self::BITS - 1 - self.leading_zeros())
        }
    }
}

impl const PrevPowerOfTwo for usize {
    fn prev_power_of_two(self) -> Self {
        if self == 0 {
            0
        } else {
            1 << (Self::BITS - 1 - self.leading_zeros())
        }
    }
}

impl const PrevPowerOfTwo for i8 {
    fn prev_power_of_two(self) -> Self {
        if self <= 0 {
            0
        } else {
            self.unsigned_abs().prev_power_of_two() as Self
        }
    }
}

impl const PrevPowerOfTwo for i16 {
    fn prev_power_of_two(self) -> Self {
        if self <= 0 {
            0
        } else {
            self.unsigned_abs().prev_power_of_two() as Self
        }
    }
}

impl const PrevPowerOfTwo for i32 {
    fn prev_power_of_two(self) -> Self {
        if self <= 0 {
            0
        } else {
            self.unsigned_abs().prev_power_of_two() as Self
        }
    }
}

impl const PrevPowerOfTwo for i64 {
    fn prev_power_of_two(self) -> Self {
        if self <= 0 {
            0
        } else {
            self.unsigned_abs().prev_power_of_two() as Self
        }
    }
}

impl const PrevPowerOfTwo for i128 {
    fn prev_power_of_two(self) -> Self {
        if self <= 0 {
            0
        } else {
            self.unsigned_abs().prev_power_of_two() as Self
        }
    }
}

impl const PrevPowerOfTwo for isize {
    fn prev_power_of_two(self) -> Self {
        if self <= 0 {
            0
        } else {
            self.unsigned_abs().prev_power_of_two() as Self
        }
    }
}

impl PrevPowerOfTwo for f16 {
    fn prev_power_of_two(self) -> Self {
        if self <= 1.0 {
            1.0
        } else {
            self.log2().floor().exp2()
        }
    }
}

impl PrevPowerOfTwo for f32 {
    fn prev_power_of_two(self) -> Self {
        if self <= 1.0 {
            1.0
        } else {
            self.log2().floor().exp2()
        }
    }
}

impl PrevPowerOfTwo for f64 {
    fn prev_power_of_two(self) -> Self {
        if self <= 1.0 {
            1.0
        } else {
            self.log2().floor().exp2()
        }
    }
}

impl PrevPowerOfTwo for f128 {
    fn prev_power_of_two(self) -> Self {
        if self <= 1.0 {
            1.0
        } else {
            self.log2().floor().exp2()
        }
    }
}
/// Round down the current number
pub const trait Floor {
    #[must_use]
    /// Round down the current number
    fn floor(self) -> Self;
}

/// Round up the current number
pub const trait Ceil {
    #[must_use]
    /// Round up the current number
    fn ceil(self) -> Self;
}

impl const Floor for f16 {
    fn floor(self) -> Self {
        core::intrinsics::floorf16(self)
    }
}
impl const Ceil for f16 {
    fn ceil(self) -> Self {
        core::intrinsics::ceilf16(self)
    }
}

impl const Floor for f32 {
    fn floor(self) -> Self {
        core::f32::math::floor(self)
    }
}
impl const Ceil for f32 {
    fn ceil(self) -> Self {
        core::f32::math::ceil(self)
    }
}

impl const Floor for f64 {
    fn floor(self) -> Self {
        core::f64::math::floor(self)
    }
}
impl const Ceil for f64 {
    fn ceil(self) -> Self {
        core::f64::math::ceil(self)
    }
}

impl const Floor for f128 {
    fn floor(self) -> Self {
        core::intrinsics::floorf128(self)
    }
}
impl const Ceil for f128 {
    fn ceil(self) -> Self {
        core::intrinsics::ceilf128(self)
    }
}

macro_rules! impl_int {
    ($($t:ty),*) => {
        $(
            impl const Floor for $t {
                fn floor(self) -> Self {
                    self
                }
            }
            impl const Ceil for $t {
                fn ceil(self) -> Self {
                    self
                }
            }
        )*
    };
}

impl_int!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

/// Trait providing trigonometric functions
pub const trait MathRotation {
    /// Computes the sine of self (in radians)
    #[must_use]
    fn sin(self) -> Self;

    /// Computes the cosine of self (in radians)
    #[must_use]
    fn cos(self) -> Self;
}

impl MathRotation for f16 {
    fn sin(self) -> Self {
        core::intrinsics::sinf16(self)
    }
    fn cos(self) -> Self {
        core::intrinsics::cosf16(self)
    }
}

impl MathRotation for f32 {
    fn sin(self) -> Self {
        core::intrinsics::sinf32(self)
    }
    fn cos(self) -> Self {
        core::intrinsics::cosf32(self)
    }
}

impl MathRotation for f64 {
    fn sin(self) -> Self {
        core::intrinsics::sinf64(self)
    }
    fn cos(self) -> Self {
        core::intrinsics::cosf64(self)
    }
}

impl MathRotation for f128 {
    fn sin(self) -> Self {
        core::intrinsics::sinf128(self)
    }
    fn cos(self) -> Self {
        core::intrinsics::cosf128(self)
    }
}

/// Trait providing fused multiply-add operation
pub const trait MulAdd {
    /// Computes `(self * a) + b` with only one rounding error if possible
    #[must_use]
    fn mul_add(self, a: Self, b: Self) -> Self;
}

impl const MulAdd for f16 {
    fn mul_add(self, a: Self, b: Self) -> Self {
        core::intrinsics::fmaf16(self, a, b)
    }
}

impl const MulAdd for f32 {
    fn mul_add(self, a: Self, b: Self) -> Self {
        core::f32::math::mul_add(self, a, b)
    }
}

impl const MulAdd for f64 {
    fn mul_add(self, a: Self, b: Self) -> Self {
        core::f64::math::mul_add(self, a, b)
    }
}

impl const MulAdd for f128 {
    fn mul_add(self, a: Self, b: Self) -> Self {
        core::intrinsics::fmaf128(self, a, b)
    }
}
macro_rules! impl_mul_add_plain {
    ($($t:ty),* $(,)?) => {
        $(
            impl const MulAdd for $t {
                #[inline(always)]
                fn mul_add(self, a: Self, b: Self) -> Self {
                    self * a + b
                }
            }
        )*
    };
}

impl_mul_add_plain!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize,
);

/// Clamp a value between a minimum and maximum bound
///
/// The function isn't called `clamp` for compatibility with `core::ops::Ord` which numbers like `core::f64` do not implement
pub const trait Clamp {
    /// Restricts a value to be within a specified range [min, max]
    #[must_use]
    fn clamped(self, min: Self, max: Self) -> Self;
}

macro_rules! impl_clamp_int {
    ($($t:ty),*) => {
        $(
            impl const Clamp for $t {
                fn clamped(self, min: Self, max: Self) -> Self {
                    if self < min {
                        min
                    } else if self > max {
                        max
                    } else {
                        self
                    }
                }
            }
        )*
    };
}

macro_rules! impl_clamp_float {
    ($($t:ty),*) => {
        $(
            impl const Clamp for $t {
                fn clamped(self, min: Self, max: Self) -> Self {
                    if self.is_nan() || min.is_nan() || max.is_nan() {
                        return self;
                    }

                    if self < min {
                        min
                    } else if self > max {
                        max
                    } else {
                        self
                    }
                }
            }
        )*
    };
}

impl_clamp_int!(i8, i16, i32, i64, i128, isize);

impl_clamp_int!(u8, u16, u32, u64, u128, usize);

impl_clamp_float!(f16, f32, f64, f128);

/// Get the hypotenuse of a value
pub const trait Hypot {
    #[must_use]
    /// Get the hypotenuse of a value
    fn hypot(self, other: Self) -> Self;
}
impl<
    T: Copy
        + [const] Sqrt
        + [const] core::ops::Add<Output = T>
        + [const] core::ops::Mul<Output = T>,
> const Hypot for T
{
    fn hypot(self, other: Self) -> Self {
        (self * self + other * other).sqrt()
    }
}

// impl Hypot for f16 {
//     fn hypot(self, other: Self) -> Self {
//         cmath::hypotf(self as f32, other as f32) as f16
//     }
// }

// impl Hypot for f32 {
//     fn hypot(self, other: Self) -> Self {
//         #[cfg(feature = "std")]
//         return self.hypot(other);
//         #[cfg(not(feature = "std"))]
//         (self * self + other * other).sqrt()
//     }
// }

// impl Hypot for f16 {
//     fn hypot(self, other: Self) -> Self {
//         #[cfg(feature = "std")]
//         return self.hypot(other);
//         #[cfg(not(feature = "std"))]
//         (self * self + other * other).sqrt()
//     }
// }

// impl Hypot for f64 {
//     fn hypot(self, other: Self) -> Self {
//         #[cfg(feature = "std")]
//         return self.hypot(other);
//         #[cfg(not(feature = "std"))]
//         (self * self + other * other).sqrt()
//     }
// }

// impl Hypot for f128 {
//     fn hypot(self, other: Self) -> Self {
//         #[cfg(feature = "std")]
//         return self.hypot(other);
//         #[cfg(not(feature = "std"))]
//         (self * self + other * other).sqrt()
//     }
// }

/// Returns the exp or exp2 of a value
pub const trait Exp {
    #[must_use]
    /// Returns e^self.
    fn exp(self) -> Self;
}
impl Exp for f16 {
    fn exp(self) -> Self {
        core::intrinsics::expf16(self)
    }
}

impl Exp for f32 {
    fn exp(self) -> Self {
        core::intrinsics::expf32(self)
    }
}

impl Exp for f64 {
    fn exp(self) -> Self {
        core::intrinsics::expf64(self)
    }
}

impl Exp for f128 {
    fn exp(self) -> Self {
        core::intrinsics::expf128(self)
    }
}
/// If the given type supports values between 0 and 1
pub const trait SupportsRange0To1 {}

/// Numbers that support the number range 0-127 (2^8)/2
pub const trait SupportsRange128 {}
/// Numbers that support the number range 0-255 (2^8)
pub const trait SupportsRange256 {}
/// Numbers that support the number range 0-32767 (2^16)/2
pub const trait SupportsRange32768 {}
/// Numbers that support the number range 0-65535 (2^16)
pub const trait SupportsRange65536 {}

mirl_core::impl_empty_trait!(SupportsRange0To1 for f16 ,f32 ,f64 ,f128);

mirl_core::impl_empty_trait!(SupportsRange128 for u8, u16, u32, u64 ,u128 ,usize ,i8 ,i16 ,i32, i64, i128, isize ,f16 ,f32 ,f64 ,f128);

mirl_core::impl_empty_trait!(SupportsRange256 for u8, u16, u32 ,u64 ,u128 ,usize, i16, i32 ,i64 ,i128, isize, f16, f32, f64 ,f128);

mirl_core::impl_empty_trait!(SupportsRange32768 for u16, u32 ,u64 ,u128 ,usize ,i16 ,i32 ,i64 ,i128, isize, f16, f32, f64 ,f128);

mirl_core::impl_empty_trait!(SupportsRange65536 for u16, u32 ,u64 ,u128 ,usize, i32 ,i64 ,i128, isize, f16, f32, f64 ,f128);
#[allow(missing_docs)]
/// An extended version of the [`ConstOne`](crate::math::ConstOne)/[`ConstZero`](crate::math::ConstZero) traits covering all numbers from 0 to 127
pub const trait ConstNumbers128: SupportsRange128 {
    const CONST_0: Self;
    const CONST_1: Self;
    const CONST_2: Self;
    const CONST_3: Self;
    const CONST_4: Self;
    const CONST_5: Self;
    const CONST_6: Self;
    const CONST_7: Self;
    const CONST_8: Self;
    const CONST_9: Self;
    const CONST_10: Self;
    const CONST_11: Self;
    const CONST_12: Self;
    const CONST_13: Self;
    const CONST_14: Self;
    const CONST_15: Self;
    const CONST_16: Self;
    const CONST_17: Self;
    const CONST_18: Self;
    const CONST_19: Self;
    const CONST_20: Self;
    const CONST_21: Self;
    const CONST_22: Self;
    const CONST_23: Self;
    const CONST_24: Self;
    const CONST_25: Self;
    const CONST_26: Self;
    const CONST_27: Self;
    const CONST_28: Self;
    const CONST_29: Self;
    const CONST_30: Self;
    const CONST_31: Self;
    const CONST_32: Self;
    const CONST_33: Self;
    const CONST_34: Self;
    const CONST_35: Self;
    const CONST_36: Self;
    const CONST_37: Self;
    const CONST_38: Self;
    const CONST_39: Self;
    const CONST_40: Self;
    const CONST_41: Self;
    const CONST_42: Self;
    const CONST_43: Self;
    const CONST_44: Self;
    const CONST_45: Self;
    const CONST_46: Self;
    const CONST_47: Self;
    const CONST_48: Self;
    const CONST_49: Self;
    const CONST_50: Self;
    const CONST_51: Self;
    const CONST_52: Self;
    const CONST_53: Self;
    const CONST_54: Self;
    const CONST_55: Self;
    const CONST_56: Self;
    const CONST_57: Self;
    const CONST_58: Self;
    const CONST_59: Self;
    const CONST_60: Self;
    const CONST_61: Self;
    const CONST_62: Self;
    const CONST_63: Self;
    const CONST_64: Self;
    const CONST_65: Self;
    const CONST_66: Self;
    const CONST_67: Self;
    const CONST_68: Self;
    const CONST_69: Self;
    const CONST_70: Self;
    const CONST_71: Self;
    const CONST_72: Self;
    const CONST_73: Self;
    const CONST_74: Self;
    const CONST_75: Self;
    const CONST_76: Self;
    const CONST_77: Self;
    const CONST_78: Self;
    const CONST_79: Self;
    const CONST_80: Self;
    const CONST_81: Self;
    const CONST_82: Self;
    const CONST_83: Self;
    const CONST_84: Self;
    const CONST_85: Self;
    const CONST_86: Self;
    const CONST_87: Self;
    const CONST_88: Self;
    const CONST_89: Self;
    const CONST_90: Self;
    const CONST_91: Self;
    const CONST_92: Self;
    const CONST_93: Self;
    const CONST_94: Self;
    const CONST_95: Self;
    const CONST_96: Self;
    const CONST_97: Self;
    const CONST_98: Self;
    const CONST_99: Self;
    const CONST_100: Self;
    const CONST_101: Self;
    const CONST_102: Self;
    const CONST_103: Self;
    const CONST_104: Self;
    const CONST_105: Self;
    const CONST_106: Self;
    const CONST_107: Self;
    const CONST_108: Self;
    const CONST_109: Self;
    const CONST_110: Self;
    const CONST_111: Self;
    const CONST_112: Self;
    const CONST_113: Self;
    const CONST_114: Self;
    const CONST_115: Self;
    const CONST_116: Self;
    const CONST_117: Self;
    const CONST_118: Self;
    const CONST_119: Self;
    const CONST_120: Self;
    const CONST_121: Self;
    const CONST_122: Self;
    const CONST_123: Self;
    const CONST_124: Self;
    const CONST_125: Self;
    const CONST_126: Self;
    const CONST_127: Self;
}

impl<
    T: crate::math::ConstOne
        + crate::math::ConstZero
        + SupportsRange128
        + const core::ops::Add<Output = T>,
> const ConstNumbers128 for T
{
    const CONST_0: Self = T::ZERO;
    const CONST_1: Self = T::ONE;
    const CONST_2: Self = T::CONST_1 + T::CONST_1;
    const CONST_3: Self = T::CONST_2 + T::CONST_1;
    const CONST_4: Self = T::CONST_3 + T::CONST_1;
    const CONST_5: Self = T::CONST_4 + T::CONST_1;
    const CONST_6: Self = T::CONST_5 + T::CONST_1;
    const CONST_7: Self = T::CONST_6 + T::CONST_1;
    const CONST_8: Self = T::CONST_7 + T::CONST_1;
    const CONST_9: Self = T::CONST_8 + T::CONST_1;
    const CONST_10: Self = T::CONST_9 + T::CONST_1;
    const CONST_11: Self = T::CONST_10 + T::CONST_1;
    const CONST_12: Self = T::CONST_11 + T::CONST_1;
    const CONST_13: Self = T::CONST_12 + T::CONST_1;
    const CONST_14: Self = T::CONST_13 + T::CONST_1;
    const CONST_15: Self = T::CONST_14 + T::CONST_1;
    const CONST_16: Self = T::CONST_15 + T::CONST_1;
    const CONST_17: Self = T::CONST_16 + T::CONST_1;
    const CONST_18: Self = T::CONST_17 + T::CONST_1;
    const CONST_19: Self = T::CONST_18 + T::CONST_1;
    const CONST_20: Self = T::CONST_19 + T::CONST_1;
    const CONST_21: Self = T::CONST_20 + T::CONST_1;
    const CONST_22: Self = T::CONST_21 + T::CONST_1;
    const CONST_23: Self = T::CONST_22 + T::CONST_1;
    const CONST_24: Self = T::CONST_23 + T::CONST_1;
    const CONST_25: Self = T::CONST_24 + T::CONST_1;
    const CONST_26: Self = T::CONST_25 + T::CONST_1;
    const CONST_27: Self = T::CONST_26 + T::CONST_1;
    const CONST_28: Self = T::CONST_27 + T::CONST_1;
    const CONST_29: Self = T::CONST_28 + T::CONST_1;
    const CONST_30: Self = T::CONST_29 + T::CONST_1;
    const CONST_31: Self = T::CONST_30 + T::CONST_1;
    const CONST_32: Self = T::CONST_31 + T::CONST_1;
    const CONST_33: Self = T::CONST_32 + T::CONST_1;
    const CONST_34: Self = T::CONST_33 + T::CONST_1;
    const CONST_35: Self = T::CONST_34 + T::CONST_1;
    const CONST_36: Self = T::CONST_35 + T::CONST_1;
    const CONST_37: Self = T::CONST_36 + T::CONST_1;
    const CONST_38: Self = T::CONST_37 + T::CONST_1;
    const CONST_39: Self = T::CONST_38 + T::CONST_1;
    const CONST_40: Self = T::CONST_39 + T::CONST_1;
    const CONST_41: Self = T::CONST_40 + T::CONST_1;
    const CONST_42: Self = T::CONST_41 + T::CONST_1;
    const CONST_43: Self = T::CONST_42 + T::CONST_1;
    const CONST_44: Self = T::CONST_43 + T::CONST_1;
    const CONST_45: Self = T::CONST_44 + T::CONST_1;
    const CONST_46: Self = T::CONST_45 + T::CONST_1;
    const CONST_47: Self = T::CONST_46 + T::CONST_1;
    const CONST_48: Self = T::CONST_47 + T::CONST_1;
    const CONST_49: Self = T::CONST_48 + T::CONST_1;
    const CONST_50: Self = T::CONST_49 + T::CONST_1;
    const CONST_51: Self = T::CONST_50 + T::CONST_1;
    const CONST_52: Self = T::CONST_51 + T::CONST_1;
    const CONST_53: Self = T::CONST_52 + T::CONST_1;
    const CONST_54: Self = T::CONST_53 + T::CONST_1;
    const CONST_55: Self = T::CONST_54 + T::CONST_1;
    const CONST_56: Self = T::CONST_55 + T::CONST_1;
    const CONST_57: Self = T::CONST_56 + T::CONST_1;
    const CONST_58: Self = T::CONST_57 + T::CONST_1;
    const CONST_59: Self = T::CONST_58 + T::CONST_1;
    const CONST_60: Self = T::CONST_59 + T::CONST_1;
    const CONST_61: Self = T::CONST_60 + T::CONST_1;
    const CONST_62: Self = T::CONST_61 + T::CONST_1;
    const CONST_63: Self = T::CONST_62 + T::CONST_1;
    const CONST_64: Self = T::CONST_63 + T::CONST_1;
    const CONST_65: Self = T::CONST_64 + T::CONST_1;
    const CONST_66: Self = T::CONST_65 + T::CONST_1;
    const CONST_67: Self = T::CONST_66 + T::CONST_1;
    const CONST_68: Self = T::CONST_67 + T::CONST_1;
    const CONST_69: Self = T::CONST_68 + T::CONST_1;
    const CONST_70: Self = T::CONST_69 + T::CONST_1;
    const CONST_71: Self = T::CONST_70 + T::CONST_1;
    const CONST_72: Self = T::CONST_71 + T::CONST_1;
    const CONST_73: Self = T::CONST_72 + T::CONST_1;
    const CONST_74: Self = T::CONST_73 + T::CONST_1;
    const CONST_75: Self = T::CONST_74 + T::CONST_1;
    const CONST_76: Self = T::CONST_75 + T::CONST_1;
    const CONST_77: Self = T::CONST_76 + T::CONST_1;
    const CONST_78: Self = T::CONST_77 + T::CONST_1;
    const CONST_79: Self = T::CONST_78 + T::CONST_1;
    const CONST_80: Self = T::CONST_79 + T::CONST_1;
    const CONST_81: Self = T::CONST_80 + T::CONST_1;
    const CONST_82: Self = T::CONST_81 + T::CONST_1;
    const CONST_83: Self = T::CONST_82 + T::CONST_1;
    const CONST_84: Self = T::CONST_83 + T::CONST_1;
    const CONST_85: Self = T::CONST_84 + T::CONST_1;
    const CONST_86: Self = T::CONST_85 + T::CONST_1;
    const CONST_87: Self = T::CONST_86 + T::CONST_1;
    const CONST_88: Self = T::CONST_87 + T::CONST_1;
    const CONST_89: Self = T::CONST_88 + T::CONST_1;
    const CONST_90: Self = T::CONST_89 + T::CONST_1;
    const CONST_91: Self = T::CONST_90 + T::CONST_1;
    const CONST_92: Self = T::CONST_91 + T::CONST_1;
    const CONST_93: Self = T::CONST_92 + T::CONST_1;
    const CONST_94: Self = T::CONST_93 + T::CONST_1;
    const CONST_95: Self = T::CONST_94 + T::CONST_1;
    const CONST_96: Self = T::CONST_95 + T::CONST_1;
    const CONST_97: Self = T::CONST_96 + T::CONST_1;
    const CONST_98: Self = T::CONST_97 + T::CONST_1;
    const CONST_99: Self = T::CONST_98 + T::CONST_1;
    const CONST_100: Self = T::CONST_99 + T::CONST_1;
    const CONST_101: Self = T::CONST_100 + T::CONST_1;
    const CONST_102: Self = T::CONST_101 + T::CONST_1;
    const CONST_103: Self = T::CONST_102 + T::CONST_1;
    const CONST_104: Self = T::CONST_103 + T::CONST_1;
    const CONST_105: Self = T::CONST_104 + T::CONST_1;
    const CONST_106: Self = T::CONST_105 + T::CONST_1;
    const CONST_107: Self = T::CONST_106 + T::CONST_1;
    const CONST_108: Self = T::CONST_107 + T::CONST_1;
    const CONST_109: Self = T::CONST_108 + T::CONST_1;
    const CONST_110: Self = T::CONST_109 + T::CONST_1;
    const CONST_111: Self = T::CONST_110 + T::CONST_1;
    const CONST_112: Self = T::CONST_111 + T::CONST_1;
    const CONST_113: Self = T::CONST_112 + T::CONST_1;
    const CONST_114: Self = T::CONST_113 + T::CONST_1;
    const CONST_115: Self = T::CONST_114 + T::CONST_1;
    const CONST_116: Self = T::CONST_115 + T::CONST_1;
    const CONST_117: Self = T::CONST_116 + T::CONST_1;
    const CONST_118: Self = T::CONST_117 + T::CONST_1;
    const CONST_119: Self = T::CONST_118 + T::CONST_1;
    const CONST_120: Self = T::CONST_119 + T::CONST_1;
    const CONST_121: Self = T::CONST_120 + T::CONST_1;
    const CONST_122: Self = T::CONST_121 + T::CONST_1;
    const CONST_123: Self = T::CONST_122 + T::CONST_1;
    const CONST_124: Self = T::CONST_123 + T::CONST_1;
    const CONST_125: Self = T::CONST_124 + T::CONST_1;
    const CONST_126: Self = T::CONST_125 + T::CONST_1;
    const CONST_127: Self = T::CONST_126 + T::CONST_1;
}
#[allow(missing_docs)]
/// An extended version of [`ConstNumbers128`] covering all numbers from 0 to 255
pub const trait ConstNumbers256:
    ConstNumbers128 + SupportsRange256
{
    const CONST_128: Self;
    const CONST_129: Self;
    const CONST_130: Self;
    const CONST_131: Self;
    const CONST_132: Self;
    const CONST_133: Self;
    const CONST_134: Self;
    const CONST_135: Self;
    const CONST_136: Self;
    const CONST_137: Self;
    const CONST_138: Self;
    const CONST_139: Self;
    const CONST_140: Self;
    const CONST_141: Self;
    const CONST_142: Self;
    const CONST_143: Self;
    const CONST_144: Self;
    const CONST_145: Self;
    const CONST_146: Self;
    const CONST_147: Self;
    const CONST_148: Self;
    const CONST_149: Self;
    const CONST_150: Self;
    const CONST_151: Self;
    const CONST_152: Self;
    const CONST_153: Self;
    const CONST_154: Self;
    const CONST_155: Self;
    const CONST_156: Self;
    const CONST_157: Self;
    const CONST_158: Self;
    const CONST_159: Self;
    const CONST_160: Self;
    const CONST_161: Self;
    const CONST_162: Self;
    const CONST_163: Self;
    const CONST_164: Self;
    const CONST_165: Self;
    const CONST_166: Self;
    const CONST_167: Self;
    const CONST_168: Self;
    const CONST_169: Self;
    const CONST_170: Self;
    const CONST_171: Self;
    const CONST_172: Self;
    const CONST_173: Self;
    const CONST_174: Self;
    const CONST_175: Self;
    const CONST_176: Self;
    const CONST_177: Self;
    const CONST_178: Self;
    const CONST_179: Self;
    const CONST_180: Self;
    const CONST_181: Self;
    const CONST_182: Self;
    const CONST_183: Self;
    const CONST_184: Self;
    const CONST_185: Self;
    const CONST_186: Self;
    const CONST_187: Self;
    const CONST_188: Self;
    const CONST_189: Self;
    const CONST_190: Self;
    const CONST_191: Self;
    const CONST_192: Self;
    const CONST_193: Self;
    const CONST_194: Self;
    const CONST_195: Self;
    const CONST_196: Self;
    const CONST_197: Self;
    const CONST_198: Self;
    const CONST_199: Self;
    const CONST_200: Self;
    const CONST_201: Self;
    const CONST_202: Self;
    const CONST_203: Self;
    const CONST_204: Self;
    const CONST_205: Self;
    const CONST_206: Self;
    const CONST_207: Self;
    const CONST_208: Self;
    const CONST_209: Self;
    const CONST_210: Self;
    const CONST_211: Self;
    const CONST_212: Self;
    const CONST_213: Self;
    const CONST_214: Self;
    const CONST_215: Self;
    const CONST_216: Self;
    const CONST_217: Self;
    const CONST_218: Self;
    const CONST_219: Self;
    const CONST_220: Self;
    const CONST_221: Self;
    const CONST_222: Self;
    const CONST_223: Self;
    const CONST_224: Self;
    const CONST_225: Self;
    const CONST_226: Self;
    const CONST_227: Self;
    const CONST_228: Self;
    const CONST_229: Self;
    const CONST_230: Self;
    const CONST_231: Self;
    const CONST_232: Self;
    const CONST_233: Self;
    const CONST_234: Self;
    const CONST_235: Self;
    const CONST_236: Self;
    const CONST_237: Self;
    const CONST_238: Self;
    const CONST_239: Self;
    const CONST_240: Self;
    const CONST_241: Self;
    const CONST_242: Self;
    const CONST_243: Self;
    const CONST_244: Self;
    const CONST_245: Self;
    const CONST_246: Self;
    const CONST_247: Self;
    const CONST_248: Self;
    const CONST_249: Self;
    const CONST_250: Self;
    const CONST_251: Self;
    const CONST_252: Self;
    const CONST_253: Self;
    const CONST_254: Self;
    const CONST_255: Self;
}

impl<
    T: crate::math::ConstOne
        + crate::math::ConstZero
        + SupportsRange256
        + const ConstNumbers128
        + const core::ops::Add<Output = T>,
> const ConstNumbers256 for T
{
    const CONST_128: Self = T::CONST_127 + T::CONST_1;
    const CONST_129: Self = T::CONST_128 + T::CONST_1;
    const CONST_130: Self = T::CONST_129 + T::CONST_1;
    const CONST_131: Self = T::CONST_130 + T::CONST_1;
    const CONST_132: Self = T::CONST_131 + T::CONST_1;
    const CONST_133: Self = T::CONST_132 + T::CONST_1;
    const CONST_134: Self = T::CONST_133 + T::CONST_1;
    const CONST_135: Self = T::CONST_134 + T::CONST_1;
    const CONST_136: Self = T::CONST_135 + T::CONST_1;
    const CONST_137: Self = T::CONST_136 + T::CONST_1;
    const CONST_138: Self = T::CONST_137 + T::CONST_1;
    const CONST_139: Self = T::CONST_138 + T::CONST_1;
    const CONST_140: Self = T::CONST_139 + T::CONST_1;
    const CONST_141: Self = T::CONST_140 + T::CONST_1;
    const CONST_142: Self = T::CONST_141 + T::CONST_1;
    const CONST_143: Self = T::CONST_142 + T::CONST_1;
    const CONST_144: Self = T::CONST_143 + T::CONST_1;
    const CONST_145: Self = T::CONST_144 + T::CONST_1;
    const CONST_146: Self = T::CONST_145 + T::CONST_1;
    const CONST_147: Self = T::CONST_146 + T::CONST_1;
    const CONST_148: Self = T::CONST_147 + T::CONST_1;
    const CONST_149: Self = T::CONST_148 + T::CONST_1;
    const CONST_150: Self = T::CONST_149 + T::CONST_1;
    const CONST_151: Self = T::CONST_150 + T::CONST_1;
    const CONST_152: Self = T::CONST_151 + T::CONST_1;
    const CONST_153: Self = T::CONST_152 + T::CONST_1;
    const CONST_154: Self = T::CONST_153 + T::CONST_1;
    const CONST_155: Self = T::CONST_154 + T::CONST_1;
    const CONST_156: Self = T::CONST_155 + T::CONST_1;
    const CONST_157: Self = T::CONST_156 + T::CONST_1;
    const CONST_158: Self = T::CONST_157 + T::CONST_1;
    const CONST_159: Self = T::CONST_158 + T::CONST_1;
    const CONST_160: Self = T::CONST_159 + T::CONST_1;
    const CONST_161: Self = T::CONST_160 + T::CONST_1;
    const CONST_162: Self = T::CONST_161 + T::CONST_1;
    const CONST_163: Self = T::CONST_162 + T::CONST_1;
    const CONST_164: Self = T::CONST_163 + T::CONST_1;
    const CONST_165: Self = T::CONST_164 + T::CONST_1;
    const CONST_166: Self = T::CONST_165 + T::CONST_1;
    const CONST_167: Self = T::CONST_166 + T::CONST_1;
    const CONST_168: Self = T::CONST_167 + T::CONST_1;
    const CONST_169: Self = T::CONST_168 + T::CONST_1;
    const CONST_170: Self = T::CONST_169 + T::CONST_1;
    const CONST_171: Self = T::CONST_170 + T::CONST_1;
    const CONST_172: Self = T::CONST_171 + T::CONST_1;
    const CONST_173: Self = T::CONST_172 + T::CONST_1;
    const CONST_174: Self = T::CONST_173 + T::CONST_1;
    const CONST_175: Self = T::CONST_174 + T::CONST_1;
    const CONST_176: Self = T::CONST_175 + T::CONST_1;
    const CONST_177: Self = T::CONST_176 + T::CONST_1;
    const CONST_178: Self = T::CONST_177 + T::CONST_1;
    const CONST_179: Self = T::CONST_178 + T::CONST_1;
    const CONST_180: Self = T::CONST_179 + T::CONST_1;
    const CONST_181: Self = T::CONST_180 + T::CONST_1;
    const CONST_182: Self = T::CONST_181 + T::CONST_1;
    const CONST_183: Self = T::CONST_182 + T::CONST_1;
    const CONST_184: Self = T::CONST_183 + T::CONST_1;
    const CONST_185: Self = T::CONST_184 + T::CONST_1;
    const CONST_186: Self = T::CONST_185 + T::CONST_1;
    const CONST_187: Self = T::CONST_186 + T::CONST_1;
    const CONST_188: Self = T::CONST_187 + T::CONST_1;
    const CONST_189: Self = T::CONST_188 + T::CONST_1;
    const CONST_190: Self = T::CONST_189 + T::CONST_1;
    const CONST_191: Self = T::CONST_190 + T::CONST_1;
    const CONST_192: Self = T::CONST_191 + T::CONST_1;
    const CONST_193: Self = T::CONST_192 + T::CONST_1;
    const CONST_194: Self = T::CONST_193 + T::CONST_1;
    const CONST_195: Self = T::CONST_194 + T::CONST_1;
    const CONST_196: Self = T::CONST_195 + T::CONST_1;
    const CONST_197: Self = T::CONST_196 + T::CONST_1;
    const CONST_198: Self = T::CONST_197 + T::CONST_1;
    const CONST_199: Self = T::CONST_198 + T::CONST_1;
    const CONST_200: Self = T::CONST_199 + T::CONST_1;
    const CONST_201: Self = T::CONST_200 + T::CONST_1;
    const CONST_202: Self = T::CONST_201 + T::CONST_1;
    const CONST_203: Self = T::CONST_202 + T::CONST_1;
    const CONST_204: Self = T::CONST_203 + T::CONST_1;
    const CONST_205: Self = T::CONST_204 + T::CONST_1;
    const CONST_206: Self = T::CONST_205 + T::CONST_1;
    const CONST_207: Self = T::CONST_206 + T::CONST_1;
    const CONST_208: Self = T::CONST_207 + T::CONST_1;
    const CONST_209: Self = T::CONST_208 + T::CONST_1;
    const CONST_210: Self = T::CONST_209 + T::CONST_1;
    const CONST_211: Self = T::CONST_210 + T::CONST_1;
    const CONST_212: Self = T::CONST_211 + T::CONST_1;
    const CONST_213: Self = T::CONST_212 + T::CONST_1;
    const CONST_214: Self = T::CONST_213 + T::CONST_1;
    const CONST_215: Self = T::CONST_214 + T::CONST_1;
    const CONST_216: Self = T::CONST_215 + T::CONST_1;
    const CONST_217: Self = T::CONST_216 + T::CONST_1;
    const CONST_218: Self = T::CONST_217 + T::CONST_1;
    const CONST_219: Self = T::CONST_218 + T::CONST_1;
    const CONST_220: Self = T::CONST_219 + T::CONST_1;
    const CONST_221: Self = T::CONST_220 + T::CONST_1;
    const CONST_222: Self = T::CONST_221 + T::CONST_1;
    const CONST_223: Self = T::CONST_222 + T::CONST_1;
    const CONST_224: Self = T::CONST_223 + T::CONST_1;
    const CONST_225: Self = T::CONST_224 + T::CONST_1;
    const CONST_226: Self = T::CONST_225 + T::CONST_1;
    const CONST_227: Self = T::CONST_226 + T::CONST_1;
    const CONST_228: Self = T::CONST_227 + T::CONST_1;
    const CONST_229: Self = T::CONST_228 + T::CONST_1;
    const CONST_230: Self = T::CONST_229 + T::CONST_1;
    const CONST_231: Self = T::CONST_230 + T::CONST_1;
    const CONST_232: Self = T::CONST_231 + T::CONST_1;
    const CONST_233: Self = T::CONST_232 + T::CONST_1;
    const CONST_234: Self = T::CONST_233 + T::CONST_1;
    const CONST_235: Self = T::CONST_234 + T::CONST_1;
    const CONST_236: Self = T::CONST_235 + T::CONST_1;
    const CONST_237: Self = T::CONST_236 + T::CONST_1;
    const CONST_238: Self = T::CONST_237 + T::CONST_1;
    const CONST_239: Self = T::CONST_238 + T::CONST_1;
    const CONST_240: Self = T::CONST_239 + T::CONST_1;
    const CONST_241: Self = T::CONST_240 + T::CONST_1;
    const CONST_242: Self = T::CONST_241 + T::CONST_1;
    const CONST_243: Self = T::CONST_242 + T::CONST_1;
    const CONST_244: Self = T::CONST_243 + T::CONST_1;
    const CONST_245: Self = T::CONST_244 + T::CONST_1;
    const CONST_246: Self = T::CONST_245 + T::CONST_1;
    const CONST_247: Self = T::CONST_246 + T::CONST_1;
    const CONST_248: Self = T::CONST_247 + T::CONST_1;
    const CONST_249: Self = T::CONST_248 + T::CONST_1;
    const CONST_250: Self = T::CONST_249 + T::CONST_1;
    const CONST_251: Self = T::CONST_250 + T::CONST_1;
    const CONST_252: Self = T::CONST_251 + T::CONST_1;
    const CONST_253: Self = T::CONST_252 + T::CONST_1;
    const CONST_254: Self = T::CONST_253 + T::CONST_1;
    const CONST_255: Self = T::CONST_254 + T::CONST_1;
}

/// Convert the current value from degrees to radians and from radians to degrees
/// T: Self
/// A: Accuracy, usually: f16, f32, f64, or f128
pub const trait AngularConversion<T, A = f64> {
    #[must_use]
    /// Convert angle degrees into angle radians
    fn to_radians(self) -> Self;
    #[must_use]
    /// Convert angle degrees into angle radians
    fn to_degrees(self) -> Self;
}
impl<
    T: [const] IntoPatch<A> + [const] core::marker::Destruct,
    A: [const] IntoPatch<T>
        + [const] core::marker::Destruct
        + ConstRotationRatio
        + [const] std::ops::Mul<Output = A>,
> const AngularConversion<T, A> for T
{
    fn to_radians(self) -> Self {
        (self.into_value() * A::DEGREES_TO_RADIAN_RATIO).into_value()
    }
    fn to_degrees(self) -> Self {
        (self.into_value() * A::RADIANS_TO_DEGREES_RATIO).into_value()
    }
}
/// Try to convert the current value from degrees to radians and from radians to degrees
/// T: Self
/// A: Accuracy, usually: f16, f32, f64, or f128
pub const trait TryAngularConversion<T, A = f64>:
    std::marker::Sized
{
    #[must_use]
    /// Try to convert angle degrees into angle radians
    fn try_to_radians(self) -> Option<Self>;
    #[must_use]
    /// Try to convert angle degrees into angle radians
    fn try_to_degrees(self) -> Option<Self>;
}
impl<
    T: [const] TryIntoPatch<A> + [const] core::marker::Destruct,
    A: [const] TryIntoPatch<T>
        + [const] core::marker::Destruct
        + ConstRotationRatio
        + [const] std::ops::Mul<Output = A>,
> const TryAngularConversion<T, A> for T
{
    fn try_to_radians(self) -> Option<Self> {
        (self.try_into_value()? * A::DEGREES_TO_RADIAN_RATIO).try_into_value()
    }
    fn try_to_degrees(self) -> Option<Self> {
        (self.try_into_value()? * A::RADIANS_TO_DEGREES_RATIO).try_into_value()
    }
}
/// Get the next bigger/smaller value that can be represented
pub const trait NextUpDown {
    #[must_use]
    /// The next bigger value
    fn next_up(self) -> Self;
    #[must_use]
    /// The next smaller value
    fn next_down(self) -> Self;
}

impl_trait!(NextUpDown {fn next_down(self)->Self{self.next_down()}fn next_up(self)->Self{self.next_up()}} for  f16,f32,f64,f128 );

impl_trait!(NextUpDown {
    /// The next bigger value, stops at the maximum value the type can represent
    fn next_down(self)->Self{self.saturating_sub(1)}
    /// The next smaller value, stops at the minimum value the type can represent
    fn next_up(self)->Self{self.saturating_add(1)}
} for  u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize );

/// A trait for getting the smallest value >0 and the biggest value <1
pub const trait UniformPreviousNext {
    /// Get the smallest value >0
    #[must_use]
    fn smallest_bigger_than_zero() -> Self;
    /// Get the biggest value <0
    #[must_use]
    fn biggest_smaller_than_one() -> Self;
}

impl<T: ConstZero + ConstOne + NextUpDown> UniformPreviousNext for T {
    fn biggest_smaller_than_one() -> Self {
        Self::ZERO.next_up()
    }
    fn smallest_bigger_than_zero() -> Self {
        Self::ONE.next_down()
    }
}

/// Interpolate between 2 values with Self being the progress
pub const trait InterpolateAsInterpolator<V1, V2 = V1, Output = V1> {
    /// Interpolate between 2 values with Self being the progress
    fn interpolate_values(self, val1: V1, val2: V2) -> Output;
}
impl<
    S: ConstZero
        + ConstOne
        + [const] core::ops::Sub<Output = S>
        + [const] core::ops::Add<Output = S>
        + [const] core::ops::Mul<Output = S>
        + Copy
        + SupportsRange0To1,
    V1: [const] IntoPatch<S>,
    V2: [const] IntoPatch<S>,
> const InterpolateAsInterpolator<V1, V2, S> for S
{
    fn interpolate_values(self, val1: V1, val2: V2) -> Self {
        (S::ONE - self) * val1.into_value() + self * val2.into_value()
    }
}

/// Interpolate between this and another value
///
/// To get this implemented for a type:
/// Pick a progress type, f64 for example
///
/// (Simple: Interpolate with self with accuracy of f64)
/// Implement `IntoPatch<f64>` and `IntoPatch<f64>` for your type
///
/// (Complex: Interpolate with T with accuracy of f64 and return O)
/// Pick a progress type, f64 for example
/// Implement `IntoPatch<f64>` for your type
/// Implement `IntoPatch<f64>` for `T`
/// Implement `FromPatch<f64>` for `O`
pub const trait InterpolateBetween<P, V2 = Self, Output = Self> {
    /// Interpolate between this and another value
    fn interpolate_with(self, other: V2, progress: P) -> Output;
}
impl<Progress: [const] InterpolateAsInterpolator<T, V2, Output>, T, V2, Output> const
    InterpolateBetween<Progress, V2, Output> for T
{
    fn interpolate_with(self, other: V2, progress: Progress) -> Output {
        progress.interpolate_values(self, other)
    }
}

/// Interpolate between 2 values with Self being the progress
pub const trait InterpolateColorAsInterpolator {
    /// Interpolate between 2 values with Self being the progress
    fn interpolate_colors(self, val1: u32, val2: u32) -> u32;
    /// Interpolate between 2 values with Self being the progress
    fn interpolate_colors_and_alpha(
        self,
        val1: u32,
        val2: u32,
        alpha: u32,
    ) -> u32;
}
impl<
    S: [const] InterpolateAsInterpolator<u32, u32, S>
        + Copy
        + [const] TryIntoPatch<u32>,
> const InterpolateColorAsInterpolator for S
{
    fn interpolate_colors(self, val1: u32, val2: u32) -> u32 {
        rgba_to_u32(
            self.interpolate_values(val1.red(), val2.red())
                .try_into_value()
                .unwrap_or_default(),
            self.interpolate_values(val1.green(), val2.green())
                .try_into_value()
                .unwrap_or_default(),
            self.interpolate_values(val1.blue(), val2.blue())
                .try_into_value()
                .unwrap_or_default(),
            self.interpolate_values(val1.alpha(), val2.alpha())
                .try_into_value()
                .unwrap_or_default(),
        )
    }
    fn interpolate_colors_and_alpha(
        self,
        val1: u32,
        val2: u32,
        alpha: u32,
    ) -> u32 {
        rgba_to_u32(
            self.interpolate_values(val1.red(), val2.red())
                .try_into_value()
                .unwrap_or_default(),
            self.interpolate_values(val1.green(), val2.green())
                .try_into_value()
                .unwrap_or_default(),
            self.interpolate_values(val1.blue(), val2.blue())
                .try_into_value()
                .unwrap_or_default(),
            alpha,
        )
    }
}
/// Interpolate between this color and another color
///
/// Analogous to [`InterpolateBetween`] but specialized for RGBA colors packed as `u32`.
pub const trait InterpolateColorBetween<P> {
    /// Interpolate between this color and another, including alpha channel
    fn interpolate_color_with(self, other: u32, progress: P) -> u32;
    /// Interpolate between this color and another, overriding the alpha channel with a fixed value
    fn interpolate_color_with_alpha(
        self,
        other: u32,
        progress: P,
        alpha: u32,
    ) -> u32;
}

impl<Progress: [const] InterpolateColorAsInterpolator> const
    InterpolateColorBetween<Progress> for u32
{
    fn interpolate_color_with(self, other: u32, progress: Progress) -> u32 {
        progress.interpolate_colors(self, other)
    }

    fn interpolate_color_with_alpha(
        self,
        other: u32,
        progress: Progress,
        alpha: u32,
    ) -> u32 {
        progress.interpolate_colors_and_alpha(self, other, alpha)
    }
}
/// Normalize a vector
pub const trait NormalizeVector<T> {
    #[must_use]
    /// Normalize the internal vector and return it
    fn normalized_vector(self) -> Self;
    /// Normalize the internal vector
    fn normalize_vector(&mut self);
}
impl<
    T: [const] std::ops::Mul<Output = T>
        + [const] std::ops::Add<Output = T>
        + [const] std::ops::Div<Output = T>
        + [const] crate::Abs
        + [const] crate::Sqrt
        + Copy,
> const NormalizeVector<T> for (T, T, T)
{
    default fn normalized_vector(self) -> Self {
        let v =
            (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).abs().sqrt();
        (self.0 / v, self.1 / v, self.2 / v)
    }
    default fn normalize_vector(&mut self) {
        let v =
            (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).abs().sqrt();
        self.0 = self.0 / v;
        self.1 = self.1 / v;
        self.2 = self.2 / v;
    }
}
/// Check if the upper and lower bounds fit inside the bounds of another value
pub const trait BoundsFitInBounds {
    #[must_use]
    /// Check if the upper and lower bounds fit inside the bounds of another value
    ///
    /// This assumes that the other value is the same size or smaller
    /// If this isn't true this function may crash
    fn bounds_fit_in_bounds<Other: Bounded + [const] IntoPatch<Self>>() -> bool
    where
        Self: Bounded + [const] core::cmp::PartialOrd + Copy;

    /// Check if the value can fit inside the upper and lower bounds of another value
    ///
    /// This assumes that the other value is the same size or smaller
    /// If this isn't true this function may crash
    fn value_fits_in_bounds<Other: Bounded + [const] IntoPatch<Self>>(
        &self,
    ) -> bool
    where
        Self: Bounded + [const] core::cmp::PartialOrd + Copy;
}

impl<S: Bounded + [const] core::cmp::PartialOrd + Copy> const BoundsFitInBounds
    for S
{
    fn bounds_fit_in_bounds<Other: Bounded + [const] IntoPatch<Self>>() -> bool
    {
        Self::MIN >= Other::MIN.into_value()
            && Self::MAX <= Other::MAX.into_value()
    }

    fn value_fits_in_bounds<Other: Bounded + [const] IntoPatch<Self>>(
        &self,
    ) -> bool {
        *self >= Other::MIN.into_value() && *self <= Other::MAX.into_value()
    }
}

/// Convert the current value into [`core::cmp::Ordering`] if self is -1, 0, or 1
pub const trait SignToOrdering {
    /// Convert the current value into [`core::cmp::Ordering`] if self is -1, 0, or 1
    fn sign_to_ordering(&self) -> Option<core::cmp::Ordering>;
}

impl<T: ConstOne + ConstZero + ConstNegativeOne + core::cmp::PartialEq>
    SignToOrdering for T
{
    fn sign_to_ordering(&self) -> Option<core::cmp::Ordering> {
        use core::cmp::Ordering;
        Some(if Self::NEGATIVE_ONE.eq(self) {
            Ordering::Greater
        } else if Self::ZERO.eq(self) {
            Ordering::Equal
        } else if Self::ONE.eq(self) {
            Ordering::Less
        } else {
            return None;
        })
        // Some(match self {
        //     Self::NEGATIVE_ONE => Ordering::Less,
        //     Self::ZERO => Ordering::Equal,
        //     Self::ONE => Ordering::Greater,
        //     _ => return None,
        // })
    }
}
