use crate::Round;

/// Round all values of a tuple
pub const trait TupleRound {
    #[must_use]
    /// Round the all the values in the current tuple
    fn tuple_round(self) -> Self;
}

impl<T: Round> TupleRound for (T,) {
    fn tuple_round(self) -> Self {
        (self.0.round(),)
    }
}

impl<T: Round> TupleRound for (T, T) {
    fn tuple_round(self) -> Self {
        (self.0.round(), self.1.round())
    }
}

impl<T: Round> TupleRound for (T, T, T) {
    fn tuple_round(self) -> Self {
        (self.0.round(), self.1.round(), self.2.round())
    }
}

impl<T: Round> TupleRound for (T, T, T, T) {
    fn tuple_round(self) -> Self {
        (self.0.round(), self.1.round(), self.2.round(), self.3.round())
    }
}

impl<T: Round> TupleRound for (T, T, T, T, T) {
    fn tuple_round(self) -> Self {
        (
            self.0.round(),
            self.1.round(),
            self.2.round(),
            self.3.round(),
            self.4.round(),
        )
    }
}

impl<T: Round> TupleRound for (T, T, T, T, T, T) {
    fn tuple_round(self) -> Self {
        (
            self.0.round(),
            self.1.round(),
            self.2.round(),
            self.3.round(),
            self.4.round(),
            self.5.round(),
        )
    }
}

impl<T: Round> TupleRound for (T, T, T, T, T, T, T) {
    fn tuple_round(self) -> Self {
        (
            self.0.round(),
            self.1.round(),
            self.2.round(),
            self.3.round(),
            self.4.round(),
            self.5.round(),
            self.6.round(),
        )
    }
}

impl<T: Round> TupleRound for (T, T, T, T, T, T, T, T) {
    fn tuple_round(self) -> Self {
        (
            self.0.round(),
            self.1.round(),
            self.2.round(),
            self.3.round(),
            self.4.round(),
            self.5.round(),
            self.6.round(),
            self.7.round(),
        )
    }
}

impl<T: Round> TupleRound for (T, T, T, T, T, T, T, T, T) {
    fn tuple_round(self) -> Self {
        (
            self.0.round(),
            self.1.round(),
            self.2.round(),
            self.3.round(),
            self.4.round(),
            self.5.round(),
            self.6.round(),
            self.7.round(),
            self.8.round(),
        )
    }
}

impl<T: Round> TupleRound for (T, T, T, T, T, T, T, T, T, T) {
    fn tuple_round(self) -> Self {
        (
            self.0.round(),
            self.1.round(),
            self.2.round(),
            self.3.round(),
            self.4.round(),
            self.5.round(),
            self.6.round(),
            self.7.round(),
            self.8.round(),
            self.9.round(),
        )
    }
}

impl<T: Round> TupleRound for (T, T, T, T, T, T, T, T, T, T, T) {
    fn tuple_round(self) -> Self {
        (
            self.0.round(),
            self.1.round(),
            self.2.round(),
            self.3.round(),
            self.4.round(),
            self.5.round(),
            self.6.round(),
            self.7.round(),
            self.8.round(),
            self.9.round(),
            self.10.round(),
        )
    }
}

impl<T: Round> TupleRound for (T, T, T, T, T, T, T, T, T, T, T, T) {
    fn tuple_round(self) -> Self {
        (
            self.0.round(),
            self.1.round(),
            self.2.round(),
            self.3.round(),
            self.4.round(),
            self.5.round(),
            self.6.round(),
            self.7.round(),
            self.8.round(),
            self.9.round(),
            self.10.round(),
            self.11.round(),
        )
    }
}
