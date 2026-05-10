// #[cfg(feature = "std")]
// use crate::conversion::impl_from_patch_self;

/// The inverse of [`FromPatch`]
pub const trait IntoPatch<T>: Sized {
    #[must_use]
    /// A custom from to bypass rusts orphan rule
    fn into_value(self) -> T;
}
impl<T: Sized + [const] FromPatch<O>, O> const IntoPatch<T> for O {
    fn into_value(self) -> T {
        T::from_value(self)
    }
}

/// Lets you convert from one value to another.
///
/// What's the difference between this and `core::convert::From`?
/// `core::convert::From` has many holes like being unable to convert a i32 to f32.
/// This trait "patches" the `core::convert::From` by recreating all safe existing conversions and extending on the original concept
pub const trait FromPatch<T>: Sized {
    #[must_use]
    /// A custom from to bypass rusts orphan rule
    fn from_value(value: T) -> Self;
}
mod numbers;

#[cfg(feature = "std")]
mod hashmap;
#[cfg(feature = "std")]
mod hashset;
#[cfg(feature = "std")]
mod string;
#[cfg(feature = "std")]
mod vec;

#[cfg(feature = "ascii")]
mod ascii;

mod tuple;

#[cfg(feature = "tokio")]
mod asynchronous;

#[cfg(feature = "std")]
crate::impl_from_patch_self!(std::time::SystemTime);

impl<T1, T2: FromPatch<T1>> FromPatch<Option<T1>> for Option<T2> {
    fn from_value(value: Option<T1>) -> Self {
        value.map(T2::from_value)
    }
}

impl<T1, T2: FromPatch<T1>, E1, E2: FromPatch<E1>> FromPatch<Result<T1, E1>>
    for Result<T2, E2>
{
    fn from_value(value: Result<T1, E1>) -> Self {
        match value {
            Err(e) => Err(E2::from_value(e)),
            Ok(v) => Ok(T2::from_value(v)),
        }
    }
}
// impl<T1, T2: FromPatch<T1>, E> FromPatch<Result<T1, E>> for Result<T2, E> {
//     fn from_value(value: Result<T1, E>) -> Self {
//         match value {
//             Err(e) => Err(e),
//             Ok(v) => Ok(T2::from_value(v)),
//         }
//     }
// }
// #[cfg(feature = "std")]
// impl<T1: IntoPatch<T2>, T2> FromPatch<Box<T1>> for Box<T2> {
//     fn from_value(value: Box<T1>) -> Self {
//         Self::new(value.into_value())
//     }
// }

impl<T: Clone> FromPatch<&T> for T {
    default fn from_value(value: &T) -> Self {
        value.clone()
    }
}
impl<T> FromPatch<core::cell::Cell<T>> for T {
    default fn from_value(value: core::cell::Cell<T>) -> Self {
        value.into_inner()
    }
}

impl<T> FromPatch<core::cell::RefCell<T>> for T {
    default fn from_value(value: core::cell::RefCell<T>) -> Self {
        value.into_inner()
    }
}
#[cfg(feature = "std")]
impl<T> FromPatch<Box<T>> for T {
    default fn from_value(value: Box<T>) -> Self {
        *value
    }
}

// Owned T2 directly (no IntoPatch indirection needed here)
impl<T> FromPatch<T> for core::cell::Cell<T> {
    fn from_value(value: T) -> Self {
        Self::new(value)
    }
}

impl<T> FromPatch<T> for core::cell::RefCell<T> {
    fn from_value(value: T) -> Self {
        Self::new(value)
    }
}

// impl<T1, T2: FromPatch<T1>> FromPatch<T1>
//     for core::cell::Cell<T2>
// {
//     fn from_value(value: T1) -> Self {
//         Self::new(T2::from_value(value))
//     }
// }

// impl FromPatch<&'static str> for &[u8] {
//     fn from_value(value: &'static str) -> Self {
//         value.as_bytes()
//     }
// }

impl<'a> FromPatch<&'a str> for &'a [u8] {
    fn from_value(value: &'a str) -> Self {
        value.as_bytes()
    }
}

#[cfg(feature = "bigfloat")]
mod bigfloat;
#[cfg(feature = "bigint")]
mod bigint;
