use crate::Floor;

/// Floor all values of a tuple
pub const trait TupleFloor {
    #[must_use]
    /// Floor the all the values in the current tuple
    fn tuple_floor(self) -> Self;
}

impl<T: Floor> TupleFloor for (T,) {
    fn tuple_floor(self) -> Self {
        (self.0.floor(),)
    }
}

impl<T: Floor> TupleFloor for (T, T) {
    fn tuple_floor(self) -> Self {
        (self.0.floor(), self.1.floor())
    }
}

impl<T: Floor> TupleFloor for (T, T, T) {
    fn tuple_floor(self) -> Self {
        (self.0.floor(), self.1.floor(), self.2.floor())
    }
}

impl<T: Floor> TupleFloor for (T, T, T, T) {
    fn tuple_floor(self) -> Self {
        (self.0.floor(), self.1.floor(), self.2.floor(), self.3.floor())
    }
}

impl<T: Floor> TupleFloor for (T, T, T, T, T) {
    fn tuple_floor(self) -> Self {
        (
            self.0.floor(),
            self.1.floor(),
            self.2.floor(),
            self.3.floor(),
            self.4.floor(),
        )
    }
}

impl<T: Floor> TupleFloor for (T, T, T, T, T, T) {
    fn tuple_floor(self) -> Self {
        (
            self.0.floor(),
            self.1.floor(),
            self.2.floor(),
            self.3.floor(),
            self.4.floor(),
            self.5.floor(),
        )
    }
}

impl<T: Floor> TupleFloor for (T, T, T, T, T, T, T) {
    fn tuple_floor(self) -> Self {
        (
            self.0.floor(),
            self.1.floor(),
            self.2.floor(),
            self.3.floor(),
            self.4.floor(),
            self.5.floor(),
            self.6.floor(),
        )
    }
}

impl<T: Floor> TupleFloor for (T, T, T, T, T, T, T, T) {
    fn tuple_floor(self) -> Self {
        (
            self.0.floor(),
            self.1.floor(),
            self.2.floor(),
            self.3.floor(),
            self.4.floor(),
            self.5.floor(),
            self.6.floor(),
            self.7.floor(),
        )
    }
}

impl<T: Floor> TupleFloor for (T, T, T, T, T, T, T, T, T) {
    fn tuple_floor(self) -> Self {
        (
            self.0.floor(),
            self.1.floor(),
            self.2.floor(),
            self.3.floor(),
            self.4.floor(),
            self.5.floor(),
            self.6.floor(),
            self.7.floor(),
            self.8.floor(),
        )
    }
}

impl<T: Floor> TupleFloor for (T, T, T, T, T, T, T, T, T, T) {
    fn tuple_floor(self) -> Self {
        (
            self.0.floor(),
            self.1.floor(),
            self.2.floor(),
            self.3.floor(),
            self.4.floor(),
            self.5.floor(),
            self.6.floor(),
            self.7.floor(),
            self.8.floor(),
            self.9.floor(),
        )
    }
}

impl<T: Floor> TupleFloor for (T, T, T, T, T, T, T, T, T, T, T) {
    fn tuple_floor(self) -> Self {
        (
            self.0.floor(),
            self.1.floor(),
            self.2.floor(),
            self.3.floor(),
            self.4.floor(),
            self.5.floor(),
            self.6.floor(),
            self.7.floor(),
            self.8.floor(),
            self.9.floor(),
            self.10.floor(),
        )
    }
}

impl<T: Floor> TupleFloor for (T, T, T, T, T, T, T, T, T, T, T, T) {
    fn tuple_floor(self) -> Self {
        (
            self.0.floor(),
            self.1.floor(),
            self.2.floor(),
            self.3.floor(),
            self.4.floor(),
            self.5.floor(),
            self.6.floor(),
            self.7.floor(),
            self.8.floor(),
            self.9.floor(),
            self.10.floor(),
            self.11.floor(),
        )
    }
}
