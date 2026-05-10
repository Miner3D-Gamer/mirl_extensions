use num_traits::ToPrimitive;

use crate::TryFromPatch;

impl TryFromPatch<&'_ str> for num_bigfloat::BigFloat {
    fn try_from_value(value: &'_ str) -> Option<Self> {
        use std::str::FromStr;
        Self::from_str(value).ok()
    }
}
impl TryFromPatch<String> for num_bigfloat::BigFloat {
    fn try_from_value(value: String) -> Option<Self> {
        Self::try_from_value(value.as_str())
    }
}
impl TryFromPatch<num_bigfloat::BigFloat> for u8 {
    fn try_from_value(value: num_bigfloat::BigFloat) -> Option<Self> {
        value.to_u8()
    }
}

impl TryFromPatch<num_bigfloat::BigFloat> for u16 {
    fn try_from_value(value: num_bigfloat::BigFloat) -> Option<Self> {
        value.to_u16()
    }
}

impl TryFromPatch<num_bigfloat::BigFloat> for u32 {
    fn try_from_value(value: num_bigfloat::BigFloat) -> Option<Self> {
        value.to_u32()
    }
}

impl TryFromPatch<num_bigfloat::BigFloat> for u64 {
    fn try_from_value(value: num_bigfloat::BigFloat) -> Option<Self> {
        value.to_u64()
    }
}

impl TryFromPatch<num_bigfloat::BigFloat> for u128 {
    fn try_from_value(value: num_bigfloat::BigFloat) -> Option<Self> {
        value.to_u128()
    }
}

impl TryFromPatch<num_bigfloat::BigFloat> for usize {
    fn try_from_value(value: num_bigfloat::BigFloat) -> Option<Self> {
        value.to_usize()
    }
}

impl TryFromPatch<num_bigfloat::BigFloat> for i8 {
    fn try_from_value(value: num_bigfloat::BigFloat) -> Option<Self> {
        value.to_i8()
    }
}

impl TryFromPatch<num_bigfloat::BigFloat> for i16 {
    fn try_from_value(value: num_bigfloat::BigFloat) -> Option<Self> {
        value.to_i16()
    }
}

impl TryFromPatch<num_bigfloat::BigFloat> for i32 {
    fn try_from_value(value: num_bigfloat::BigFloat) -> Option<Self> {
        value.to_i32()
    }
}

impl TryFromPatch<num_bigfloat::BigFloat> for i64 {
    fn try_from_value(value: num_bigfloat::BigFloat) -> Option<Self> {
        value.to_i64()
    }
}

impl TryFromPatch<num_bigfloat::BigFloat> for i128 {
    fn try_from_value(value: num_bigfloat::BigFloat) -> Option<Self> {
        value.to_i128()
    }
}

impl TryFromPatch<num_bigfloat::BigFloat> for isize {
    fn try_from_value(value: num_bigfloat::BigFloat) -> Option<Self> {
        value.to_isize()
    }
}

impl TryFromPatch<num_bigfloat::BigFloat> for f32 {
    fn try_from_value(value: num_bigfloat::BigFloat) -> Option<Self> {
        Some(value.to_f32())
    }
}

impl TryFromPatch<num_bigfloat::BigFloat> for f64 {
    fn try_from_value(value: num_bigfloat::BigFloat) -> Option<Self> {
        Some(value.to_f64())
    }
}
