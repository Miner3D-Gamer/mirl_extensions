// #[allow(clippy::missing_errors_doc)]
// /// Converts from one error type to another
// pub fn result_error_into<T, I, O: core::convert::From<I>>(
//     result: core::result::Result<T, I>,
// ) -> core::result::Result<T, O> {
//     match result {
//         Ok(v) => Ok(v),
//         Err(e) => Err(e.into()),
//     }
// }

// #[allow(clippy::missing_errors_doc)]
// /// Converts from one value type to another
// pub fn result_value_into<T, V: core::convert::From<T>, I>(
//     result: core::result::Result<T, I>,
// ) -> core::result::Result<V, I> {
//     match result {
//         Ok(v) => Ok(v.into()),
//         Err(e) => Err(e),
//     }
// }

// #[allow(clippy::missing_errors_doc)]
// /// Converts from one value and error type to another
// pub fn result_into<
//     T,
//     V: core::convert::From<T>,
//     I,
//     O: core::convert::From<I>,
// >(
//     result: core::result::Result<T, I>,
// ) -> core::result::Result<V, O> {
//     match result {
//         Ok(v) => Ok(v.into()),
//         Err(e) => Err(e.into()),
//     }
// }

use crate::IntoPatch;

/// Converts from one error type to another
pub const trait ExtendedErrorErrorInto<T, I: Into<O>, O> {
    #[allow(clippy::missing_errors_doc)]
    /// Converts from one error type to another
    fn result_error_into(self) -> core::result::Result<T, O>;
}

impl<T, I: Into<O>, O> ExtendedErrorErrorInto<T, I, O>
    for core::result::Result<T, I>
{
    #[allow(clippy::missing_errors_doc)]
    /// Converts from one error type to another
    fn result_error_into(self) -> core::result::Result<T, O> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }
}

/// Converts from one value type inside an error to another
pub const trait ExtendedErrorValueInto<T: Into<V>, V, I> {
    #[allow(clippy::missing_errors_doc)]
    /// Converts from one value type inside an error to another
    fn result_value_into(self) -> core::result::Result<V, I>;
}

impl<T: Into<V>, V, I> ExtendedErrorValueInto<T, V, I>
    for core::result::Result<T, I>
{
    fn result_value_into(self) -> core::result::Result<V, I> {
        match self {
            Ok(v) => Ok(v.into()),
            Err(e) => Err(e),
        }
    }
}

/// Converts from one value and error type to another
pub const trait ExtendedErrorFullInto<T: Into<V>, V, I: Into<O>, O> {
    #[allow(clippy::missing_errors_doc)]
    /// Converts from one value and error type to another
    fn result_into(self) -> core::result::Result<V, O>;
}

impl<T: Into<V>, V, I: Into<O>, O> ExtendedErrorFullInto<T, V, I, O>
    for core::result::Result<T, I>
{
    fn result_into(self) -> core::result::Result<V, O> {
        match self {
            Ok(v) => Ok(v.into()),
            Err(e) => Err(e.into()),
        }
    }
}

/// Converts from one error type to another
pub const trait ExtendedErrorErrorIntoPatch<T, I: Into<O>, O> {
    #[allow(clippy::missing_errors_doc)]
    /// Converts from one error type to another
    fn result_error_into_value(self) -> core::result::Result<T, O>;
}

impl<T, I: Into<O>, O> ExtendedErrorErrorIntoPatch<T, I, O>
    for core::result::Result<T, I>
{
    #[allow(clippy::missing_errors_doc)]
    /// Converts from one error type to another
    fn result_error_into_value(self) -> core::result::Result<T, O> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }
}

/// Converts from one value type inside an error to another
pub const trait ExtendedErrorValueIntoPatch<T: IntoPatch<V>, V, I> {
    #[allow(clippy::missing_errors_doc)]
    /// Converts from one value type inside an error to another
    fn result_value_into_value(self) -> core::result::Result<V, I>;
}

impl<T: IntoPatch<V>, V, I> ExtendedErrorValueIntoPatch<T, V, I>
    for core::result::Result<T, I>
{
    fn result_value_into_value(self) -> core::result::Result<V, I> {
        match self {
            Ok(v) => Ok(v.into_value()),
            Err(e) => Err(e),
        }
    }
}

/// Converts from one value and error type to another
pub const trait ExtendedErrorFullIntoPatch<T: Into<V>, V, I: Into<O>, O> {
    #[allow(clippy::missing_errors_doc)]
    /// Converts from one value and error type to another
    fn result_into_value(self) -> core::result::Result<V, O>;
}

impl<T: Into<V>, V, I: Into<O>, O> ExtendedErrorFullIntoPatch<T, V, I, O>
    for core::result::Result<T, I>
{
    fn result_into_value(self) -> core::result::Result<V, O> {
        match self {
            Ok(v) => Ok(v.into()),
            Err(e) => Err(e.into()),
        }
    }
}
