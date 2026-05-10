use crate::FromPatch;

impl crate::AutoImplTryFromPatch for num_bigfloat::BigFloat {}

macro_rules! impl_into_bigint {
    ($($t:ty),* $(,)?) => {
        $(
            impl FromPatch<$t> for num_bigfloat::BigFloat {
                fn from_value(value: $t) -> Self {
                    Self::from(value)
                }
            }
        )*
    };
}
impl_into_bigint!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

impl FromPatch<usize> for num_bigfloat::BigFloat {
    fn from_value(value: usize) -> Self {
        Self::from(value as u64)
    }
}
