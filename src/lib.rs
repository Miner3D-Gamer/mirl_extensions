//! A crate extending almost all rust types with a bunch of new functionality
// Actual additions
// #![feature(f128)]
// #![feature(f16)]
#![feature(iter_advance_by)]
// Other
#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(min_specialization)]
// This has no effect yet but will work as indented once it's implemented by the rust team
// Accessing core
#![allow(internal_features)]
#![feature(core_intrinsics)]
// Const features
#![feature(const_ops)]
#![feature(const_destruct)]
#![feature(const_default)]
#![feature(const_trait_impl)]
#![feature(const_option_ops)]
#![feature(const_try)]
#![cfg_attr(feature = "mirl_geometry", feature(const_cmp))]
// #![feature(const_ref_cell)]

// TODO: Implement a `.map(|x| x)` function for tuples

pub use mirl_core::{impl_empty_trait, impl_trait};
pub use mirl_extensions_conversion::*;
pub use mirl_extensions_core::*;
pub use mirl_extensions_math::*;

// mod_and_pub_import!(conversion);
mod_and_pub_import!(consts);
mod_and_pub_import!(keycodes);
mod_and_pub_import!(builtin);
#[cfg(feature = "mirl_geometry")]
mod_and_pub_import!(geometry);
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
