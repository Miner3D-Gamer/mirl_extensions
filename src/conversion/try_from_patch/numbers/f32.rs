// use crate::TryFromPatch;

// f32
crate::impl_try_from_patch_float!(
    f32, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, usize, isize
);
// impl const TryFromPatch<f32> for i8 {
//     fn try_from_value(v: f32) -> Option<Self> {
//         Some(v as Self)
//     }
// }
// impl const TryFromPatch<f32> for i16 {
//     fn try_from_value(v: f32) -> Option<Self> {
//         Some(v as Self)
//     }
// }
// impl const TryFromPatch<f32> for i32 {
//     fn try_from_value(v: f32) -> Option<Self> {
//         Some(v as Self)
//     }
// }
// impl const TryFromPatch<f32> for i64 {
//     fn try_from_value(v: f32) -> Option<Self> {
//         Some(v as Self)
//     }
// }
// impl const TryFromPatch<f32> for i128 {
//     fn try_from_value(v: f32) -> Option<Self> {
//         Some(v as Self)
//     }
// }
// impl const TryFromPatch<f32> for isize {
//     fn try_from_value(v: f32) -> Option<Self> {
//         Some(v as Self)
//     }
// }
// impl const TryFromPatch<f32> for u8 {
//     fn try_from_value(v: f32) -> Option<Self> {
//         Some(v as Self)
//     }
// }
// impl const TryFromPatch<f32> for u16 {
//     fn try_from_value(v: f32) -> Option<Self> {
//         Some(v as Self)
//     }
// }
// impl const TryFromPatch<f32> for u32 {
//     fn try_from_value(v: f32) -> Option<Self> {
//         Some(v as Self)
//     }
// }
// impl const TryFromPatch<f32> for u64 {
//     fn try_from_value(v: f32) -> Option<Self> {
//         Some(v as Self)
//     }
// }
// impl const TryFromPatch<f32> for u128 {
//     fn try_from_value(v: f32) -> Option<Self> {
//         Some(v as Self)
//     }
// }
// impl const TryFromPatch<f32> for usize {
//     fn try_from_value(v: f32) -> Option<Self> {
//         Some(v as Self)
//     }
// }
