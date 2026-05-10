// use crate::BoundsFitInBounds;

crate::impl_try_from_patch_float!(
    f64, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, usize, isize
);

// // f64
// impl TryFromPatch<f64> for i8 {
//     fn try_from_value(v: f64) -> Option<Self> {
//         if v.value_fits_in_bounds::<Self>() {
//             Some(unsafe { v.to_int_unchecked::<Self>() })
//         } else {
//             None
//         }
//     }
// }
// impl TryFromPatch<f64> for i16 {
//     fn try_from_value(v: f64) -> Option<Self> {
//         Some(v as Self)
//     }
// }
// impl TryFromPatch<f64> for i32 {
//     fn try_from_value(v: f64) -> Option<Self> {
//         Some(v as Self)
//     }
// }
// impl TryFromPatch<f64> for i64 {
//     fn try_from_value(v: f64) -> Option<Self> {
//         Some(v as Self)
//     }
// }
// impl TryFromPatch<f64> for i128 {
//     fn try_from_value(v: f64) -> Option<Self> {
//         Some(v as Self)
//     }
// }
// impl TryFromPatch<f64> for isize {
//     fn try_from_value(v: f64) -> Option<Self> {
//         Some(v as Self)
//     }
// }
// impl TryFromPatch<f64> for u8 {
//     fn try_from_value(v: f64) -> Option<Self> {
//         Some(v as Self)
//     }
// }
// impl TryFromPatch<f64> for u16 {
//     fn try_from_value(v: f64) -> Option<Self> {
//         Some(v as Self)
//     }
// }
// impl TryFromPatch<f64> for u32 {
//     fn try_from_value(v: f64) -> Option<Self> {
//         Some(v as Self)
//     }
// }
// impl TryFromPatch<f64> for u64 {
//     fn try_from_value(v: f64) -> Option<Self> {
//         Some(v as Self)
//     }
// }
// impl TryFromPatch<f64> for u128 {
//     fn try_from_value(v: f64) -> Option<Self> {
//         Some(v as Self)
//     }
// }
// impl TryFromPatch<f64> for usize {
//     fn try_from_value(v: f64) -> Option<Self> {
//         Some(v as Self)
//     }
// }
// impl TryFromPatch<f64> for f32 {
//     fn try_from_value(v: Self) -> Option<Self> {
//         if v.value_fits_in_bounds::<Self>() {
//             Some(unsafe { v.to_ })
//         } else {
//             None
//         }
//     }
// }
// impl TryFromPatch<f64> for f16 {
//     fn try_from_value(v: Self) -> Option<Self> {
//         Some(v)
//     }
// }
// impl  TryFromPatch<Self> for f128 {
//     fn try_from_value(v: Self) -> Option<Self> {
//         Some(v)
//     }
// }
// use crate::TryFromPatch;
