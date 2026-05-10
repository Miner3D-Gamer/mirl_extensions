use crate::Abs;

/// Abs all values of a tuple
pub const trait TupleAbs {
    #[must_use]
    /// Abs the all the values in the current tuple
    fn tuple_abs(self) -> Self;
}

impl<T: Abs> TupleAbs for (T,) {
    fn tuple_abs(self) -> Self {
        (self.0.abs(),)
    }
}

impl<T: Abs> TupleAbs for (T, T) {
    fn tuple_abs(self) -> Self {
        (self.0.abs(), self.1.abs())
    }
}

impl<T: Abs> TupleAbs for (T, T, T) {
    fn tuple_abs(self) -> Self {
        (self.0.abs(), self.1.abs(), self.2.abs())
    }
}

impl<T: Abs> TupleAbs for (T, T, T, T) {
    fn tuple_abs(self) -> Self {
        (self.0.abs(), self.1.abs(), self.2.abs(), self.3.abs())
    }
}

impl<T: Abs> TupleAbs for (T, T, T, T, T) {
    fn tuple_abs(self) -> Self {
        (self.0.abs(), self.1.abs(), self.2.abs(), self.3.abs(), self.4.abs())
    }
}

impl<T: Abs> TupleAbs for (T, T, T, T, T, T) {
    fn tuple_abs(self) -> Self {
        (
            self.0.abs(),
            self.1.abs(),
            self.2.abs(),
            self.3.abs(),
            self.4.abs(),
            self.5.abs(),
        )
    }
}

impl<T: Abs> TupleAbs for (T, T, T, T, T, T, T) {
    fn tuple_abs(self) -> Self {
        (
            self.0.abs(),
            self.1.abs(),
            self.2.abs(),
            self.3.abs(),
            self.4.abs(),
            self.5.abs(),
            self.6.abs(),
        )
    }
}

impl<T: Abs> TupleAbs for (T, T, T, T, T, T, T, T) {
    fn tuple_abs(self) -> Self {
        (
            self.0.abs(),
            self.1.abs(),
            self.2.abs(),
            self.3.abs(),
            self.4.abs(),
            self.5.abs(),
            self.6.abs(),
            self.7.abs(),
        )
    }
}

impl<T: Abs> TupleAbs for (T, T, T, T, T, T, T, T, T) {
    fn tuple_abs(self) -> Self {
        (
            self.0.abs(),
            self.1.abs(),
            self.2.abs(),
            self.3.abs(),
            self.4.abs(),
            self.5.abs(),
            self.6.abs(),
            self.7.abs(),
            self.8.abs(),
        )
    }
}

impl<T: Abs> TupleAbs for (T, T, T, T, T, T, T, T, T, T) {
    fn tuple_abs(self) -> Self {
        (
            self.0.abs(),
            self.1.abs(),
            self.2.abs(),
            self.3.abs(),
            self.4.abs(),
            self.5.abs(),
            self.6.abs(),
            self.7.abs(),
            self.8.abs(),
            self.9.abs(),
        )
    }
}

impl<T: Abs> TupleAbs for (T, T, T, T, T, T, T, T, T, T, T) {
    fn tuple_abs(self) -> Self {
        (
            self.0.abs(),
            self.1.abs(),
            self.2.abs(),
            self.3.abs(),
            self.4.abs(),
            self.5.abs(),
            self.6.abs(),
            self.7.abs(),
            self.8.abs(),
            self.9.abs(),
            self.10.abs(),
        )
    }
}

impl<T: Abs> TupleAbs for (T, T, T, T, T, T, T, T, T, T, T, T) {
    fn tuple_abs(self) -> Self {
        (
            self.0.abs(),
            self.1.abs(),
            self.2.abs(),
            self.3.abs(),
            self.4.abs(),
            self.5.abs(),
            self.6.abs(),
            self.7.abs(),
            self.8.abs(),
            self.9.abs(),
            self.10.abs(),
            self.11.abs(),
        )
    }
}
