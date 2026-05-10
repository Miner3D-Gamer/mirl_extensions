use crate::Sqrt;

/// Sqrt all values of a tuple
pub const trait TupleSqrt {
    #[must_use]
    /// Sqrt the all the values in the current tuple
    fn tuplesqrt(self) -> Self;
}

impl<T: Sqrt> TupleSqrt for (T,) {
    fn tuplesqrt(self) -> Self {
        (self.0.sqrt(),)
    }
}

impl<T: Sqrt> TupleSqrt for (T, T) {
    fn tuplesqrt(self) -> Self {
        (self.0.sqrt(), self.1.sqrt())
    }
}

impl<T: Sqrt> TupleSqrt for (T, T, T) {
    fn tuplesqrt(self) -> Self {
        (self.0.sqrt(), self.1.sqrt(), self.2.sqrt())
    }
}

impl<T: Sqrt> TupleSqrt for (T, T, T, T) {
    fn tuplesqrt(self) -> Self {
        (self.0.sqrt(), self.1.sqrt(), self.2.sqrt(), self.3.sqrt())
    }
}

impl<T: Sqrt> TupleSqrt for (T, T, T, T, T) {
    fn tuplesqrt(self) -> Self {
        (
            self.0.sqrt(),
            self.1.sqrt(),
            self.2.sqrt(),
            self.3.sqrt(),
            self.4.sqrt(),
        )
    }
}

impl<T: Sqrt> TupleSqrt for (T, T, T, T, T, T) {
    fn tuplesqrt(self) -> Self {
        (
            self.0.sqrt(),
            self.1.sqrt(),
            self.2.sqrt(),
            self.3.sqrt(),
            self.4.sqrt(),
            self.5.sqrt(),
        )
    }
}

impl<T: Sqrt> TupleSqrt for (T, T, T, T, T, T, T) {
    fn tuplesqrt(self) -> Self {
        (
            self.0.sqrt(),
            self.1.sqrt(),
            self.2.sqrt(),
            self.3.sqrt(),
            self.4.sqrt(),
            self.5.sqrt(),
            self.6.sqrt(),
        )
    }
}

impl<T: Sqrt> TupleSqrt for (T, T, T, T, T, T, T, T) {
    fn tuplesqrt(self) -> Self {
        (
            self.0.sqrt(),
            self.1.sqrt(),
            self.2.sqrt(),
            self.3.sqrt(),
            self.4.sqrt(),
            self.5.sqrt(),
            self.6.sqrt(),
            self.7.sqrt(),
        )
    }
}

impl<T: Sqrt> TupleSqrt for (T, T, T, T, T, T, T, T, T) {
    fn tuplesqrt(self) -> Self {
        (
            self.0.sqrt(),
            self.1.sqrt(),
            self.2.sqrt(),
            self.3.sqrt(),
            self.4.sqrt(),
            self.5.sqrt(),
            self.6.sqrt(),
            self.7.sqrt(),
            self.8.sqrt(),
        )
    }
}

impl<T: Sqrt> TupleSqrt for (T, T, T, T, T, T, T, T, T, T) {
    fn tuplesqrt(self) -> Self {
        (
            self.0.sqrt(),
            self.1.sqrt(),
            self.2.sqrt(),
            self.3.sqrt(),
            self.4.sqrt(),
            self.5.sqrt(),
            self.6.sqrt(),
            self.7.sqrt(),
            self.8.sqrt(),
            self.9.sqrt(),
        )
    }
}

impl<T: Sqrt> TupleSqrt for (T, T, T, T, T, T, T, T, T, T, T) {
    fn tuplesqrt(self) -> Self {
        (
            self.0.sqrt(),
            self.1.sqrt(),
            self.2.sqrt(),
            self.3.sqrt(),
            self.4.sqrt(),
            self.5.sqrt(),
            self.6.sqrt(),
            self.7.sqrt(),
            self.8.sqrt(),
            self.9.sqrt(),
            self.10.sqrt(),
        )
    }
}

impl<T: Sqrt> TupleSqrt for (T, T, T, T, T, T, T, T, T, T, T, T) {
    fn tuplesqrt(self) -> Self {
        (
            self.0.sqrt(),
            self.1.sqrt(),
            self.2.sqrt(),
            self.3.sqrt(),
            self.4.sqrt(),
            self.5.sqrt(),
            self.6.sqrt(),
            self.7.sqrt(),
            self.8.sqrt(),
            self.9.sqrt(),
            self.10.sqrt(),
            self.11.sqrt(),
        )
    }
}
