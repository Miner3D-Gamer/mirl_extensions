use num_traits::ToPrimitive;

use crate::TryFromPatch;

impl TryFromPatch<&'_ str> for num_bigint::BigInt {
    fn try_from_value(value: &'_ str) -> Option<Self> {
        use std::str::FromStr;
        Self::from_str(value).ok()
    }
}
impl TryFromPatch<String> for num_bigint::BigInt {
    fn try_from_value(value: String) -> Option<Self> {
        Self::try_from_value(value.as_str())
    }
}

impl TryFromPatch<num_bigint::BigInt> for u8 {
    fn try_from_value(value: num_bigint::BigInt) -> Option<Self> {
        value.to_u8()
    }
}

impl TryFromPatch<num_bigint::BigInt> for u16 {
    fn try_from_value(value: num_bigint::BigInt) -> Option<Self> {
        value.to_u16()
    }
}

impl TryFromPatch<num_bigint::BigInt> for u32 {
    fn try_from_value(value: num_bigint::BigInt) -> Option<Self> {
        value.to_u32()
    }
}

impl TryFromPatch<num_bigint::BigInt> for u64 {
    fn try_from_value(value: num_bigint::BigInt) -> Option<Self> {
        value.to_u64()
    }
}

impl TryFromPatch<num_bigint::BigInt> for u128 {
    fn try_from_value(value: num_bigint::BigInt) -> Option<Self> {
        value.to_u128()
    }
}

impl TryFromPatch<num_bigint::BigInt> for usize {
    fn try_from_value(value: num_bigint::BigInt) -> Option<Self> {
        value.to_usize()
    }
}

impl TryFromPatch<num_bigint::BigInt> for i8 {
    fn try_from_value(value: num_bigint::BigInt) -> Option<Self> {
        value.to_i8()
    }
}

impl TryFromPatch<num_bigint::BigInt> for i16 {
    fn try_from_value(value: num_bigint::BigInt) -> Option<Self> {
        value.to_i16()
    }
}

impl TryFromPatch<num_bigint::BigInt> for i32 {
    fn try_from_value(value: num_bigint::BigInt) -> Option<Self> {
        value.to_i32()
    }
}

impl TryFromPatch<num_bigint::BigInt> for i64 {
    fn try_from_value(value: num_bigint::BigInt) -> Option<Self> {
        value.to_i64()
    }
}

impl TryFromPatch<num_bigint::BigInt> for i128 {
    fn try_from_value(value: num_bigint::BigInt) -> Option<Self> {
        value.to_i128()
    }
}

impl TryFromPatch<num_bigint::BigInt> for isize {
    fn try_from_value(value: num_bigint::BigInt) -> Option<Self> {
        value.to_isize()
    }
}

impl TryFromPatch<num_bigint::BigInt> for f32 {
    fn try_from_value(value: num_bigint::BigInt) -> Option<Self> {
        value.to_f32()
    }
}

impl TryFromPatch<num_bigint::BigInt> for f64 {
    fn try_from_value(value: num_bigint::BigInt) -> Option<Self> {
        value.to_f64()
    }
}
