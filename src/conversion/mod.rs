mod try_from_patch;
pub use try_from_patch::*;
mod from_patch;
pub use from_patch::*;

/// If a type is primitive, we can automatically derive [`FromPatch`] for any [`FromPatch`] implementations
pub trait AutoImplTryFromPatch {}

/// Using this macro, every given object will automatically implement [`FromPatch`] for itself
///
/// `impl_from_patch_self!({struct1}, {struct2}, {enum})`
#[macro_export]
macro_rules! impl_from_patch_self {
    ($($t:ty),* $(,)?) => {
        $(
            // impl crate::extension::ImplFromPatchForSelf for $t {}

            impl const $crate::FromPatch<$t> for $t {
                 fn from_value(value: $t) -> Self {
                    value
                }
            }
        )*
    };
}
impl_from_patch_self!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f16, f32,
    f64, f128, bool, char
);
mirl_core::impl_empty_trait!(AutoImplTryFromPatch for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize,f16, f32, f64,f128, bool, char);

// impl TryFromPatch<i128> for i128 {
//     fn try_from_value(value: i128) -> Option<Self> {
//         Some(value)
//     }
// }

/// Implement a safe float to Option<Self> conversion
///
/// Usage:
/// `impl_try_from_patch_float(float, int1, int2, int3)`
// TODO:
// the `FloatToInt` trait is not yet const
// Make these impl const again as soon as rust nightly allows for const `FloatToInt`
#[macro_export]
macro_rules! impl_try_from_patch_float {
    ($float:ty, $($int:ty),*) => {
        $(
            impl $crate::TryFromPatch<$float> for $int {
                fn try_from_value(v: $float) -> Option<Self> {
                    use $crate::math::BoundsFitInBounds;
                    if v.value_fits_in_bounds::<Self>() {
                        Some(unsafe { v.to_int_unchecked::<Self>() })
                    } else {
                        None
                    }
                }
            }
        )*
    };
}
// impl<T1: IsPrimitive> IsPrimitive for (T1,) {}
// impl<T1: IsPrimitive, T2: IsPrimitive> IsPrimitive for (T1, T2) {}
// impl<T1: IsPrimitive, T2: IsPrimitive, T3: IsPrimitive> IsPrimitive
//     for (T1, T2, T3)
// {
// }

// impl<T1: IsPrimitive, T2: IsPrimitive, T3: IsPrimitive, T4: IsPrimitive>
//     IsPrimitive for (T1, T2, T3, T4)
// {
// }

// impl<
//     T1: IsPrimitive,
//     T2: IsPrimitive,
//     T3: IsPrimitive,
//     T4: IsPrimitive,
//     T5: IsPrimitive,
// > IsPrimitive for (T1, T2, T3, T4, T5)
// {
// }

// impl<
//     T1: IsPrimitive,
//     T2: IsPrimitive,
//     T3: IsPrimitive,
//     T4: IsPrimitive,
//     T5: IsPrimitive,
//     T6: IsPrimitive,
// > IsPrimitive for (T1, T2, T3, T4, T5, T6)
// {
// }

// impl<
//     T1: IsPrimitive,
//     T2: IsPrimitive,
//     T3: IsPrimitive,
//     T4: IsPrimitive,
//     T5: IsPrimitive,
//     T6: IsPrimitive,
//     T7: IsPrimitive,
// > IsPrimitive for (T1, T2, T3, T4, T5, T6, T7)
// {
// }

// impl<
//     T1: IsPrimitive,
//     T2: IsPrimitive,
//     T3: IsPrimitive,
//     T4: IsPrimitive,
//     T5: IsPrimitive,
//     T6: IsPrimitive,
//     T7: IsPrimitive,
//     T8: IsPrimitive,
// > IsPrimitive for (T1, T2, T3, T4, T5, T6, T7, T8)
// {
// }

// impl<
//     T1: IsPrimitive,
//     T2: IsPrimitive,
//     T3: IsPrimitive,
//     T4: IsPrimitive,
//     T5: IsPrimitive,
//     T6: IsPrimitive,
//     T7: IsPrimitive,
//     T8: IsPrimitive,
//     T9: IsPrimitive,
// > IsPrimitive for (T1, T2, T3, T4, T5, T6, T7, T8, T9)
// {
// }

// impl<
//     T1: IsPrimitive,
//     T2: IsPrimitive,
//     T3: IsPrimitive,
//     T4: IsPrimitive,
//     T5: IsPrimitive,
//     T6: IsPrimitive,
//     T7: IsPrimitive,
//     T8: IsPrimitive,
//     T9: IsPrimitive,
//     T10: IsPrimitive,
// > IsPrimitive for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
// {
// }

// impl<
//     T1: IsPrimitive,
//     T2: IsPrimitive,
//     T3: IsPrimitive,
//     T4: IsPrimitive,
//     T5: IsPrimitive,
//     T6: IsPrimitive,
//     T7: IsPrimitive,
//     T8: IsPrimitive,
//     T9: IsPrimitive,
//     T10: IsPrimitive,
//     T11: IsPrimitive,
// > IsPrimitive for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
// {
// }

// impl<
//     T1: IsPrimitive,
//     T2: IsPrimitive,
//     T3: IsPrimitive,
//     T4: IsPrimitive,
//     T5: IsPrimitive,
//     T6: IsPrimitive,
//     T7: IsPrimitive,
//     T8: IsPrimitive,
//     T9: IsPrimitive,
//     T10: IsPrimitive,
//     T11: IsPrimitive,
//     T12: IsPrimitive,
// > IsPrimitive for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)
// {
// }
