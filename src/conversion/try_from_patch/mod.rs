use crate::{AutoImplTryFromPatch, FromPatch};

/// The inverse of [`TryFromPatch`]
pub const trait TryIntoPatch<T>: Sized {
    #[must_use]
    /// A custom from to bypass rusts orphan rule
    fn try_into_value(self) -> Option<T>;
}
impl<T: Sized + [const] TryFromPatch<O>, O> const TryIntoPatch<T> for O {
    default fn try_into_value(self) -> Option<T> {
        T::try_from_value(self)
    }
}

/// Lets you convert from one value to another.
///
/// What's the difference between this and [`core::convert::TryFrom`]?
/// [`core::convert::TryFrom`] has many holes covered by [`core::convert::From`], this inconveniences things.
/// This trait "patches" the [`core::convert::TryFrom`] by combining both traits and adding even more
pub const trait TryFromPatch<T>: Sized {
    #[must_use]
    /// A custom from to bypass rusts orphan rule
    fn try_from_value(value: T) -> Option<Self>;
}

impl<T: FromPatch<V> + AutoImplTryFromPatch, V> TryFromPatch<V> for T {
    default fn try_from_value(value: V) -> Option<Self> {
        Some(T::from_value(value))
    }
}

#[cfg(feature = "std")]
mod map;
mod numbers;
#[cfg(feature = "std")]
mod string;
#[cfg(feature = "std")]
mod vec;

#[cfg(feature = "bigfloat")]
mod bigfloat;
#[cfg(feature = "bigint")]
mod bigint;

#[cfg(feature = "ascii")]
mod ascii;