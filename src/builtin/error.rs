#[allow(clippy::missing_errors_doc)]
/// Converts from one error type to another
pub fn result_error_into<T, I, O: core::convert::From<I>>(
    result: core::result::Result<T, I>,
) -> core::result::Result<T, O> {
    match result {
        Ok(v) => Ok(v),
        Err(e) => Err(e.into()),
    }
}

#[allow(clippy::missing_errors_doc)]
/// Converts from one value type to another
pub fn result_value_into<T, V: core::convert::From<T>, I>(
    result: core::result::Result<T, I>,
) -> core::result::Result<V, I> {
    match result {
        Ok(v) => Ok(v.into()),
        Err(e) => Err(e),
    }
}

#[allow(clippy::missing_errors_doc)]
/// Converts from one value and error type to another
pub fn result_into<
    T,
    V: core::convert::From<T>,
    I,
    O: core::convert::From<I>,
>(
    result: core::result::Result<T, I>,
) -> core::result::Result<V, O> {
    match result {
        Ok(v) => Ok(v.into()),
        Err(e) => Err(e.into()),
    }
}

/// Converts from one error type to another
pub const trait ExtendedErrorErrorInto<T, I, O: core::convert::From<I>> {
    #[allow(clippy::missing_errors_doc)]
    /// Converts from one error type to another
    fn result_error_into(&self) -> core::result::Result<T, O>;
}

impl<T: Clone, I: Clone, O: core::convert::From<I>>
    ExtendedErrorErrorInto<T, I, O> for core::result::Result<T, I>
{
    #[allow(clippy::missing_errors_doc)]
    /// Converts from one error type to another
    fn result_error_into(&self) -> core::result::Result<T, O> {
        match self {
            Ok(v) => Ok(v.clone()),
            Err(e) => Err(e.clone().into()),
        }
    }
}

/// Converts from one value type inside an error to another
pub const trait ExtendedErrorValueInto<T, V: core::convert::From<T>, I> {
    #[allow(clippy::missing_errors_doc)]
    /// Converts from one value type inside an error to another
    fn result_value_into(&self) -> core::result::Result<V, I>;
}

impl<T: Clone, V: core::convert::From<T>, I: Clone>
    ExtendedErrorValueInto<T, V, I> for core::result::Result<T, I>
{
    fn result_value_into(&self) -> core::result::Result<V, I> {
        match self {
            Ok(v) => Ok(v.clone().into()),
            Err(e) => Err(e.clone()),
        }
    }
}

/// Converts from one value and error type to another
pub const trait ExtendedErrorFullInto<
    T,
    V: core::convert::From<T>,
    I,
    O: core::convert::From<I>,
>
{
    #[allow(clippy::missing_errors_doc)]
    /// Converts from one value and error type to another
    fn result_into(&self) -> core::result::Result<V, O>;
}

impl<T: Clone, V: core::convert::From<T>, I: Clone, O: core::convert::From<I>>
    ExtendedErrorFullInto<T, V, I, O> for core::result::Result<T, I>
{
    fn result_into(&self) -> core::result::Result<V, O> {
        match self {
            Ok(v) => Ok(v.clone().into()),
            Err(e) => Err(e.clone().into()),
        }
    }
}
