use num_bigint::BigInt;

use crate::FromPatch;

impl crate::AutoImplTryFromPatch for BigInt {}

macro_rules! impl_into_bigint {
    ($($t:ty),* $(,)?) => {
        $(
            impl FromPatch<$t> for BigInt {
                fn from_value(value: $t) -> Self {
                    Self::from(value)
                }
            }
        )*
    };
}
impl_into_bigint!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128);
