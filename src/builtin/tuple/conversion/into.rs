#![allow(clippy::type_complexity)]
use core::marker::Destruct;

use crate::IntoPatch;

/// Converts a tuple into another tuple of the same size with converted element types
pub const trait TupleInto<T> {
    /// Convert all given values into a target value tuple
    ///
    /// To convert each value into an individual type, use [`into_value`](IntoPatch::into_value) instead
    fn tuple_into(self) -> T;
}

impl<T, S: [const] IntoPatch<T> + [const] Destruct> const TupleInto<(T,)>
    for (S,)
{
    fn tuple_into(self) -> (T,) {
        (self.0.into_value(),)
    }
}

impl<T, S0, S1> const TupleInto<(T, T)> for (S0, S1)
where
    S0: [const] IntoPatch<T> + [const] Destruct,
    S1: [const] IntoPatch<T> + [const] Destruct,
{
    fn tuple_into(self) -> (T, T) {
        (self.0.into_value(), self.1.into_value())
    }
}

impl<T, S0, S1, S2> const TupleInto<(T, T, T)> for (S0, S1, S2)
where
    S0: [const] IntoPatch<T> + [const] Destruct,
    S1: [const] IntoPatch<T> + [const] Destruct,
    S2: [const] IntoPatch<T> + [const] Destruct,
{
    fn tuple_into(self) -> (T, T, T) {
        (self.0.into_value(), self.1.into_value(), self.2.into_value())
    }
}

impl<T, S0, S1, S2, S3> const TupleInto<(T, T, T, T)> for (S0, S1, S2, S3)
where
    S0: [const] IntoPatch<T> + [const] Destruct,
    S1: [const] IntoPatch<T> + [const] Destruct,
    S2: [const] IntoPatch<T> + [const] Destruct,
    S3: [const] IntoPatch<T> + [const] Destruct,
{
    fn tuple_into(self) -> (T, T, T, T) {
        (
            self.0.into_value(),
            self.1.into_value(),
            self.2.into_value(),
            self.3.into_value(),
        )
    }
}

impl<T, S0, S1, S2, S3, S4> const TupleInto<(T, T, T, T, T)>
    for (S0, S1, S2, S3, S4)
where
    S0: [const] IntoPatch<T> + [const] Destruct,
    S1: [const] IntoPatch<T> + [const] Destruct,
    S2: [const] IntoPatch<T> + [const] Destruct,
    S3: [const] IntoPatch<T> + [const] Destruct,
    S4: [const] IntoPatch<T> + [const] Destruct,
{
    fn tuple_into(self) -> (T, T, T, T, T) {
        (
            self.0.into_value(),
            self.1.into_value(),
            self.2.into_value(),
            self.3.into_value(),
            self.4.into_value(),
        )
    }
}

impl<T, S0, S1, S2, S3, S4, S5> const TupleInto<(T, T, T, T, T, T)>
    for (S0, S1, S2, S3, S4, S5)
where
    S0: [const] IntoPatch<T> + [const] Destruct,
    S1: [const] IntoPatch<T> + [const] Destruct,
    S2: [const] IntoPatch<T> + [const] Destruct,
    S3: [const] IntoPatch<T> + [const] Destruct,
    S4: [const] IntoPatch<T> + [const] Destruct,
    S5: [const] IntoPatch<T> + [const] Destruct,
{
    fn tuple_into(self) -> (T, T, T, T, T, T) {
        (
            self.0.into_value(),
            self.1.into_value(),
            self.2.into_value(),
            self.3.into_value(),
            self.4.into_value(),
            self.5.into_value(),
        )
    }
}

impl<T, S0, S1, S2, S3, S4, S5, S6> const TupleInto<(T, T, T, T, T, T, T)>
    for (S0, S1, S2, S3, S4, S5, S6)
where
    S0: [const] IntoPatch<T> + [const] Destruct,
    S1: [const] IntoPatch<T> + [const] Destruct,
    S2: [const] IntoPatch<T> + [const] Destruct,
    S3: [const] IntoPatch<T> + [const] Destruct,
    S4: [const] IntoPatch<T> + [const] Destruct,
    S5: [const] IntoPatch<T> + [const] Destruct,
    S6: [const] IntoPatch<T> + [const] Destruct,
{
    fn tuple_into(self) -> (T, T, T, T, T, T, T) {
        (
            self.0.into_value(),
            self.1.into_value(),
            self.2.into_value(),
            self.3.into_value(),
            self.4.into_value(),
            self.5.into_value(),
            self.6.into_value(),
        )
    }
}

impl<T, S0, S1, S2, S3, S4, S5, S6, S7> const
    TupleInto<(T, T, T, T, T, T, T, T)> for (S0, S1, S2, S3, S4, S5, S6, S7)
where
    S0: [const] IntoPatch<T> + [const] Destruct,
    S1: [const] IntoPatch<T> + [const] Destruct,
    S2: [const] IntoPatch<T> + [const] Destruct,
    S3: [const] IntoPatch<T> + [const] Destruct,
    S4: [const] IntoPatch<T> + [const] Destruct,
    S5: [const] IntoPatch<T> + [const] Destruct,
    S6: [const] IntoPatch<T> + [const] Destruct,
    S7: [const] IntoPatch<T> + [const] Destruct,
{
    fn tuple_into(self) -> (T, T, T, T, T, T, T, T) {
        (
            self.0.into_value(),
            self.1.into_value(),
            self.2.into_value(),
            self.3.into_value(),
            self.4.into_value(),
            self.5.into_value(),
            self.6.into_value(),
            self.7.into_value(),
        )
    }
}

impl<T, S0, S1, S2, S3, S4, S5, S6, S7, S8> const
    TupleInto<(T, T, T, T, T, T, T, T, T)>
    for (S0, S1, S2, S3, S4, S5, S6, S7, S8)
where
    S0: [const] IntoPatch<T> + [const] Destruct,
    S1: [const] IntoPatch<T> + [const] Destruct,
    S2: [const] IntoPatch<T> + [const] Destruct,
    S3: [const] IntoPatch<T> + [const] Destruct,
    S4: [const] IntoPatch<T> + [const] Destruct,
    S5: [const] IntoPatch<T> + [const] Destruct,
    S6: [const] IntoPatch<T> + [const] Destruct,
    S7: [const] IntoPatch<T> + [const] Destruct,
    S8: [const] IntoPatch<T> + [const] Destruct,
{
    fn tuple_into(self) -> (T, T, T, T, T, T, T, T, T) {
        (
            self.0.into_value(),
            self.1.into_value(),
            self.2.into_value(),
            self.3.into_value(),
            self.4.into_value(),
            self.5.into_value(),
            self.6.into_value(),
            self.7.into_value(),
            self.8.into_value(),
        )
    }
}

impl<T, S0, S1, S2, S3, S4, S5, S6, S7, S8, S9> const
    TupleInto<(T, T, T, T, T, T, T, T, T, T)>
    for (S0, S1, S2, S3, S4, S5, S6, S7, S8, S9)
where
    S0: [const] IntoPatch<T> + [const] Destruct,
    S1: [const] IntoPatch<T> + [const] Destruct,
    S2: [const] IntoPatch<T> + [const] Destruct,
    S3: [const] IntoPatch<T> + [const] Destruct,
    S4: [const] IntoPatch<T> + [const] Destruct,
    S5: [const] IntoPatch<T> + [const] Destruct,
    S6: [const] IntoPatch<T> + [const] Destruct,
    S7: [const] IntoPatch<T> + [const] Destruct,
    S8: [const] IntoPatch<T> + [const] Destruct,
    S9: [const] IntoPatch<T> + [const] Destruct,
{
    fn tuple_into(self) -> (T, T, T, T, T, T, T, T, T, T) {
        (
            self.0.into_value(),
            self.1.into_value(),
            self.2.into_value(),
            self.3.into_value(),
            self.4.into_value(),
            self.5.into_value(),
            self.6.into_value(),
            self.7.into_value(),
            self.8.into_value(),
            self.9.into_value(),
        )
    }
}

impl<T, S0, S1, S2, S3, S4, S5, S6, S7, S8, S9, S10> const
    TupleInto<(T, T, T, T, T, T, T, T, T, T, T)>
    for (S0, S1, S2, S3, S4, S5, S6, S7, S8, S9, S10)
where
    S0: [const] IntoPatch<T> + [const] Destruct,
    S1: [const] IntoPatch<T> + [const] Destruct,
    S2: [const] IntoPatch<T> + [const] Destruct,
    S3: [const] IntoPatch<T> + [const] Destruct,
    S4: [const] IntoPatch<T> + [const] Destruct,
    S5: [const] IntoPatch<T> + [const] Destruct,
    S6: [const] IntoPatch<T> + [const] Destruct,
    S7: [const] IntoPatch<T> + [const] Destruct,
    S8: [const] IntoPatch<T> + [const] Destruct,
    S9: [const] IntoPatch<T> + [const] Destruct,
    S10: [const] IntoPatch<T> + [const] Destruct,
{
    fn tuple_into(self) -> (T, T, T, T, T, T, T, T, T, T, T) {
        (
            self.0.into_value(),
            self.1.into_value(),
            self.2.into_value(),
            self.3.into_value(),
            self.4.into_value(),
            self.5.into_value(),
            self.6.into_value(),
            self.7.into_value(),
            self.8.into_value(),
            self.9.into_value(),
            self.10.into_value(),
        )
    }
}

impl<T, S0, S1, S2, S3, S4, S5, S6, S7, S8, S9, S10, S11> const
    TupleInto<(T, T, T, T, T, T, T, T, T, T, T, T)>
    for (S0, S1, S2, S3, S4, S5, S6, S7, S8, S9, S10, S11)
where
    S0: [const] IntoPatch<T> + [const] Destruct,
    S1: [const] IntoPatch<T> + [const] Destruct,
    S2: [const] IntoPatch<T> + [const] Destruct,
    S3: [const] IntoPatch<T> + [const] Destruct,
    S4: [const] IntoPatch<T> + [const] Destruct,
    S5: [const] IntoPatch<T> + [const] Destruct,
    S6: [const] IntoPatch<T> + [const] Destruct,
    S7: [const] IntoPatch<T> + [const] Destruct,
    S8: [const] IntoPatch<T> + [const] Destruct,
    S9: [const] IntoPatch<T> + [const] Destruct,
    S10: [const] IntoPatch<T> + [const] Destruct,
    S11: [const] IntoPatch<T> + [const] Destruct,
{
    fn tuple_into(self) -> (T, T, T, T, T, T, T, T, T, T, T, T) {
        (
            self.0.into_value(),
            self.1.into_value(),
            self.2.into_value(),
            self.3.into_value(),
            self.4.into_value(),
            self.5.into_value(),
            self.6.into_value(),
            self.7.into_value(),
            self.8.into_value(),
            self.9.into_value(),
            self.10.into_value(),
            self.11.into_value(),
        )
    }
}
