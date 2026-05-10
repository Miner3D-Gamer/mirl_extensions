use crate::Ceil;

/// Ceil all values of a tuple
pub const trait TupleCeil {
    #[must_use]
    /// Ceil the all the values in the current tuple
    fn tuple_ceil(self) -> Self;
}

impl<T: Ceil> TupleCeil for (T,) {
    fn tuple_ceil(self) -> Self {
        (self.0.ceil(),)
    }
}

impl<T: Ceil> TupleCeil for (T, T) {
    fn tuple_ceil(self) -> Self {
        (self.0.ceil(), self.1.ceil())
    }
}

impl<T: Ceil> TupleCeil for (T, T, T) {
    fn tuple_ceil(self) -> Self {
        (self.0.ceil(), self.1.ceil(), self.2.ceil())
    }
}

impl<T: Ceil> TupleCeil for (T, T, T, T) {
    fn tuple_ceil(self) -> Self {
        (self.0.ceil(), self.1.ceil(), self.2.ceil(), self.3.ceil())
    }
}

impl<T: Ceil> TupleCeil for (T, T, T, T, T) {
    fn tuple_ceil(self) -> Self {
        (
            self.0.ceil(),
            self.1.ceil(),
            self.2.ceil(),
            self.3.ceil(),
            self.4.ceil(),
        )
    }
}

impl<T: Ceil> TupleCeil for (T, T, T, T, T, T) {
    fn tuple_ceil(self) -> Self {
        (
            self.0.ceil(),
            self.1.ceil(),
            self.2.ceil(),
            self.3.ceil(),
            self.4.ceil(),
            self.5.ceil(),
        )
    }
}

impl<T: Ceil> TupleCeil for (T, T, T, T, T, T, T) {
    fn tuple_ceil(self) -> Self {
        (
            self.0.ceil(),
            self.1.ceil(),
            self.2.ceil(),
            self.3.ceil(),
            self.4.ceil(),
            self.5.ceil(),
            self.6.ceil(),
        )
    }
}

impl<T: Ceil> TupleCeil for (T, T, T, T, T, T, T, T) {
    fn tuple_ceil(self) -> Self {
        (
            self.0.ceil(),
            self.1.ceil(),
            self.2.ceil(),
            self.3.ceil(),
            self.4.ceil(),
            self.5.ceil(),
            self.6.ceil(),
            self.7.ceil(),
        )
    }
}

impl<T: Ceil> TupleCeil for (T, T, T, T, T, T, T, T, T) {
    fn tuple_ceil(self) -> Self {
        (
            self.0.ceil(),
            self.1.ceil(),
            self.2.ceil(),
            self.3.ceil(),
            self.4.ceil(),
            self.5.ceil(),
            self.6.ceil(),
            self.7.ceil(),
            self.8.ceil(),
        )
    }
}

impl<T: Ceil> TupleCeil for (T, T, T, T, T, T, T, T, T, T) {
    fn tuple_ceil(self) -> Self {
        (
            self.0.ceil(),
            self.1.ceil(),
            self.2.ceil(),
            self.3.ceil(),
            self.4.ceil(),
            self.5.ceil(),
            self.6.ceil(),
            self.7.ceil(),
            self.8.ceil(),
            self.9.ceil(),
        )
    }
}

impl<T: Ceil> TupleCeil for (T, T, T, T, T, T, T, T, T, T, T) {
    fn tuple_ceil(self) -> Self {
        (
            self.0.ceil(),
            self.1.ceil(),
            self.2.ceil(),
            self.3.ceil(),
            self.4.ceil(),
            self.5.ceil(),
            self.6.ceil(),
            self.7.ceil(),
            self.8.ceil(),
            self.9.ceil(),
            self.10.ceil(),
        )
    }
}

impl<T: Ceil> TupleCeil for (T, T, T, T, T, T, T, T, T, T, T, T) {
    fn tuple_ceil(self) -> Self {
        (
            self.0.ceil(),
            self.1.ceil(),
            self.2.ceil(),
            self.3.ceil(),
            self.4.ceil(),
            self.5.ceil(),
            self.6.ceil(),
            self.7.ceil(),
            self.8.ceil(),
            self.9.ceil(),
            self.10.ceil(),
            self.11.ceil(),
        )
    }
}
