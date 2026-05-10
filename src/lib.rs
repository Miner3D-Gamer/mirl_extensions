//! A crate extending almost all rust types with a bunch of new functionality
// Actual additions
#![feature(f128)]
#![feature(f16)]
#![feature(iter_advance_by)]
// Other
#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(min_specialization)]
// This has no effect yet but will work as indented once it's implemented by the rust team
// Accessing core
#![allow(internal_features)]
#![feature(core_float_math)]
#![feature(core_intrinsics)]
// Const features
#![feature(const_ops)]
#![feature(const_convert)]
#![feature(const_destruct)]
#![feature(const_default)]
#![feature(const_trait_impl)]
#![feature(const_option_ops)]
#![feature(const_try)]
#![feature(const_cmp)]
#![feature(const_result_trait_fn)]
// #![feature(const_ref_cell)]

pub use mirl_core::{impl_empty_trait, impl_trait};
pub use mirl_extensions_core::*;

mod_and_pub_import!(math);
mod_and_pub_import!(conversion);
mod_and_pub_import!(consts);
mod_and_pub_import!(keycodes);
mod_and_pub_import!(builtin);
mod_and_pub_import!(geometry);
mod_and_pub_import!(direction);
mod_and_pub_import!(supported_crates);
#[cfg(feature = "mirl_values")]
mod_and_pub_import!(mirl_values);

// mod u4;
// pub use u4::*;

// mod u2;
// pub use u2::*;

// mod u1;
// pub use u1::*;

// mod small_u_support;

/// Get the size of the given cursor resolution
pub const trait GetCursorResolution {
    #[must_use]
    /// Get the size of the cursor in pixels
    fn get_cursor_resolution<T: [const] crate::FromPatch<u8>>(&self) -> T;
    #[must_use]
    /// Try to get the size of the cursor in pixels
    fn try_get_cursor_resolution<T: [const] crate::TryFromPatch<u8>>(
        &self,
    ) -> Option<T>;
}
impl const GetCursorResolution
    for mirl_core::platform::mouse::CursorResolution
{
    fn get_cursor_resolution<T: [const] crate::FromPatch<u8>>(&self) -> T {
        T::from_value(match self {
            Self::X16 => 15,
            Self::X32 => 31,
            Self::X64 => 63,
            Self::X128 => 127,
            Self::X256 => 255,
        })
    }
    fn try_get_cursor_resolution<T: [const] crate::TryFromPatch<u8>>(
        &self,
    ) -> Option<T> {
        T::try_from_value(match self {
            Self::X16 => 15,
            Self::X32 => 31,
            Self::X64 => 63,
            Self::X128 => 127,
            Self::X256 => 255,
        })
    }
}
