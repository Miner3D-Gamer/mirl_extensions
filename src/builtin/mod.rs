mod cell;
pub use cell::*;

mod refcell;

mod error;
pub use error::*;

#[cfg(feature = "std")]
mod repeat;
#[cfg(feature = "std")]
pub use repeat::*;

/// An extension to `core::ops::Range`
mod range;
pub use range::*;

mod char;
pub use char::*;

// #[cfg(feature = "std")]
mod panic;

// #[cfg(feature = "std")]
pub use panic::*;

/// More tuple functions
mod tuple;

pub use tuple::*;

#[cfg(feature = "std")]
mod string;
#[cfg(feature = "std")]
pub use string::*;

/// List operations
#[cfg(feature = "std")]
mod lists;
#[cfg(feature = "std")]
pub use lists::*;

/// Writing out {variable} = `core::default::Default::default()`; is annoying, if only there was a function you could call from the variable itself.
pub const trait SetToDefault {
    /// Set the value to its default form
    fn restore_default(&mut self);
}
impl<T: Default> SetToDefault for T {
    fn restore_default(&mut self) {
        *self = core::default::Default::default();
    }
}
/// A trait for quickly printing to console by just calling [`.println_self()`](LogToConsole::println_self) instead of [`println!("{:?}", self)`](std::println)
pub const trait LogToConsole: LogToConsoleHelper {
    /// Equivalent to println!("{self:?}")
    fn println_self(&self);
    /// Equivalent to print!("{self:?}")
    fn print_self(&self);
}
/// A helper trait for [`LogToConsole`] which automatically implements prefix and suffix functions
pub const trait LogToConsoleHelper {
    /// Print the given value with a prefix
    fn println_self_prefixed(&self, prefix: &str);
    /// Print the given value with a suffix
    fn println_self_suffixed(&self, suffix: &str);
}
#[cfg(feature = "std")]
impl<T: LogToConsole + core::fmt::Debug> LogToConsoleHelper for T {
    fn println_self_prefixed(&self, prefix: &str) {
        String::println_self(&format!("{prefix}{self:?}"));
    }
    fn println_self_suffixed(&self, suffix: &str) {
        String::println_self(&format!("{self:?}{suffix}"));
    }
}
#[cfg(feature = "std")]
impl<T: core::fmt::Debug> LogToConsole for T {
    fn println_self(&self) {
        println!("{self:?}");
    }
    fn print_self(&self) {
        print!("{self:?}");
    }
}
/// Extra functionality for [`FileData`](mirl_core::platform::file_system::FileData)
pub const trait FileDataStringListSupport {
    #[must_use]
    /// Constructor to load data from a [`Vec<String>`]
    fn from_list_of_strings(
        value: &[String],
    ) -> mirl_core::platform::file_system::FileData;
    /// Get the list of strings/file paths
    #[must_use]
    fn to_list_of_strings(&self) -> Option<Vec<String>>;
}
impl FileDataStringListSupport for mirl_core::platform::file_system::FileData {
    fn to_list_of_strings(&self) -> Option<Vec<String>> {
        self.raw_data.bytes_to_strings()
    }
    fn from_list_of_strings(
        value: &[String],
    ) -> mirl_core::platform::file_system::FileData {
        Self::from_bytes(
            value.strings_to_bytes(),
            mirl_core::platform::file_system::DataType::ListOfText,
        )
    }
}
