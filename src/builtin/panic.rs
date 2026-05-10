/// Allows for using `.unwrap()`
pub const trait Unwrap<O> {
    #[track_caller]
    /// Unwrap the given value, extracting them from their container or panicking
    fn unwrap(self) -> O;
    /// Unwrap the given value, extracting them from their container or get a defined default
    fn unwrap_or(self, default: O) -> O;

    /// Unwrap the given value without checking if the value is unwrappable.
    ///
    /// # Safety
    /// You are responsible for assuring the value is available. If it isn't UB will occur
    unsafe fn unwrap_unchecked(self) -> O;
}

/// Allows for using `.unwrap_or_default()`
pub const trait UnwrapDefault<O> {
    /// Unwrap the given value, extracting them from their container or panicking
    fn unwrap_or_default(self) -> O;
}

impl<O: [const] core::marker::Destruct> const Unwrap<O> for Option<O> {
    #[inline(always)]
    fn unwrap(self) -> O {
        Self::unwrap(self)
    }
    #[inline(always)]
    fn unwrap_or(self, default: O) -> O {
        Self::unwrap_or(self, default)
    }
    #[inline(always)]
    unsafe fn unwrap_unchecked(self) -> O {
        unsafe { Self::unwrap_unchecked(self) }
    }
}
impl<O: [const] core::default::Default> const UnwrapDefault<O> for Option<O> {
    #[inline(always)]
    fn unwrap_or_default(self) -> O {
        Self::unwrap_or_default(self)
    }
}
// #[cfg_attr(not(panic = "immediate-abort"), inline(never))]
// #[cfg_attr(panic = "immediate-abort", inline)]
// #[cold]
// #[track_caller]
// pub const fn unwrap_failed() -> ! {
//     core::panicking::panic("called `Option::unwrap()` on a `None` value")
// }
