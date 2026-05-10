use crate::{TryFromPatch, TryIntoPatch};

impl<T: TryIntoPatch<O>, O> TryFromPatch<(T,)> for (O,) {
    fn try_from_value(value: (T,)) -> Option<Self> {
        Some((value.0.try_into_value()?,))
    }
}

impl<T1: TryIntoPatch<O1>, T2: TryIntoPatch<O2>, O1, O2> TryFromPatch<(T1, T2)>
    for (O1, O2)
{
    fn try_from_value(value: (T1, T2)) -> Option<Self> {
        Some((value.0.try_into_value()?, value.1.try_into_value()?))
    }
}

impl<
    T1: TryIntoPatch<O1>,
    T2: TryIntoPatch<O2>,
    T3: TryIntoPatch<O3>,
    O1,
    O2,
    O3,
> TryFromPatch<(T1, T2, T3)> for (O1, O2, O3)
{
    fn try_from_value(value: (T1, T2, T3)) -> Option<Self> {
        Some((
            value.0.try_into_value()?,
            value.1.try_into_value()?,
            value.2.try_into_value()?,
        ))
    }
}

impl<
    T1: TryIntoPatch<O1>,
    T2: TryIntoPatch<O2>,
    T3: TryIntoPatch<O3>,
    T4: TryIntoPatch<O4>,
    O1,
    O2,
    O3,
    O4,
> TryFromPatch<(T1, T2, T3, T4)> for (O1, O2, O3, O4)
{
    fn try_from_value(value: (T1, T2, T3, T4)) -> Option<Self> {
        Some((
            value.0.try_into_value()?,
            value.1.try_into_value()?,
            value.2.try_into_value()?,
            value.3.try_into_value()?,
        ))
    }
}

impl<
    T1: TryIntoPatch<O1>,
    T2: TryIntoPatch<O2>,
    T3: TryIntoPatch<O3>,
    T4: TryIntoPatch<O4>,
    T5: TryIntoPatch<O5>,
    O1,
    O2,
    O3,
    O4,
    O5,
> TryFromPatch<(T1, T2, T3, T4, T5)> for (O1, O2, O3, O4, O5)
{
    fn try_from_value(value: (T1, T2, T3, T4, T5)) -> Option<Self> {
        Some((
            value.0.try_into_value()?,
            value.1.try_into_value()?,
            value.2.try_into_value()?,
            value.3.try_into_value()?,
            value.4.try_into_value()?,
        ))
    }
}

impl<
    T1: TryIntoPatch<O1>,
    T2: TryIntoPatch<O2>,
    T3: TryIntoPatch<O3>,
    T4: TryIntoPatch<O4>,
    T5: TryIntoPatch<O5>,
    T6: TryIntoPatch<O6>,
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
> TryFromPatch<(T1, T2, T3, T4, T5, T6)> for (O1, O2, O3, O4, O5, O6)
{
    fn try_from_value(value: (T1, T2, T3, T4, T5, T6)) -> Option<Self> {
        Some((
            value.0.try_into_value()?,
            value.1.try_into_value()?,
            value.2.try_into_value()?,
            value.3.try_into_value()?,
            value.4.try_into_value()?,
            value.5.try_into_value()?,
        ))
    }
}

impl<
    T1: TryIntoPatch<O1>,
    T2: TryIntoPatch<O2>,
    T3: TryIntoPatch<O3>,
    T4: TryIntoPatch<O4>,
    T5: TryIntoPatch<O5>,
    T6: TryIntoPatch<O6>,
    T7: TryIntoPatch<O7>,
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
    O7,
> TryFromPatch<(T1, T2, T3, T4, T5, T6, T7)> for (O1, O2, O3, O4, O5, O6, O7)
{
    fn try_from_value(value: (T1, T2, T3, T4, T5, T6, T7)) -> Option<Self> {
        Some((
            value.0.try_into_value()?,
            value.1.try_into_value()?,
            value.2.try_into_value()?,
            value.3.try_into_value()?,
            value.4.try_into_value()?,
            value.5.try_into_value()?,
            value.6.try_into_value()?,
        ))
    }
}

impl<
    T1: TryIntoPatch<O1>,
    T2: TryIntoPatch<O2>,
    T3: TryIntoPatch<O3>,
    T4: TryIntoPatch<O4>,
    T5: TryIntoPatch<O5>,
    T6: TryIntoPatch<O6>,
    T7: TryIntoPatch<O7>,
    T8: TryIntoPatch<O8>,
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
    O7,
    O8,
> TryFromPatch<(T1, T2, T3, T4, T5, T6, T7, T8)>
    for (O1, O2, O3, O4, O5, O6, O7, O8)
{
    fn try_from_value(value: (T1, T2, T3, T4, T5, T6, T7, T8)) -> Option<Self> {
        Some((
            value.0.try_into_value()?,
            value.1.try_into_value()?,
            value.2.try_into_value()?,
            value.3.try_into_value()?,
            value.4.try_into_value()?,
            value.5.try_into_value()?,
            value.6.try_into_value()?,
            value.7.try_into_value()?,
        ))
    }
}

impl<
    T1: TryIntoPatch<O1>,
    T2: TryIntoPatch<O2>,
    T3: TryIntoPatch<O3>,
    T4: TryIntoPatch<O4>,
    T5: TryIntoPatch<O5>,
    T6: TryIntoPatch<O6>,
    T7: TryIntoPatch<O7>,
    T8: TryIntoPatch<O8>,
    T9: TryIntoPatch<O9>,
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
    O7,
    O8,
    O9,
> TryFromPatch<(T1, T2, T3, T4, T5, T6, T7, T8, T9)>
    for (O1, O2, O3, O4, O5, O6, O7, O8, O9)
{
    fn try_from_value(
        value: (T1, T2, T3, T4, T5, T6, T7, T8, T9),
    ) -> Option<Self> {
        Some((
            value.0.try_into_value()?,
            value.1.try_into_value()?,
            value.2.try_into_value()?,
            value.3.try_into_value()?,
            value.4.try_into_value()?,
            value.5.try_into_value()?,
            value.6.try_into_value()?,
            value.7.try_into_value()?,
            value.8.try_into_value()?,
        ))
    }
}

impl<
    T1: TryIntoPatch<O1>,
    T2: TryIntoPatch<O2>,
    T3: TryIntoPatch<O3>,
    T4: TryIntoPatch<O4>,
    T5: TryIntoPatch<O5>,
    T6: TryIntoPatch<O6>,
    T7: TryIntoPatch<O7>,
    T8: TryIntoPatch<O8>,
    T9: TryIntoPatch<O9>,
    T10: TryIntoPatch<O10>,
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
    O7,
    O8,
    O9,
    O10,
> TryFromPatch<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)>
    for (O1, O2, O3, O4, O5, O6, O7, O8, O9, O10)
{
    fn try_from_value(
        value: (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10),
    ) -> Option<Self> {
        Some((
            value.0.try_into_value()?,
            value.1.try_into_value()?,
            value.2.try_into_value()?,
            value.3.try_into_value()?,
            value.4.try_into_value()?,
            value.5.try_into_value()?,
            value.6.try_into_value()?,
            value.7.try_into_value()?,
            value.8.try_into_value()?,
            value.9.try_into_value()?,
        ))
    }
}

impl<
    T1: TryIntoPatch<O1>,
    T2: TryIntoPatch<O2>,
    T3: TryIntoPatch<O3>,
    T4: TryIntoPatch<O4>,
    T5: TryIntoPatch<O5>,
    T6: TryIntoPatch<O6>,
    T7: TryIntoPatch<O7>,
    T8: TryIntoPatch<O8>,
    T9: TryIntoPatch<O9>,
    T10: TryIntoPatch<O10>,
    T11: TryIntoPatch<O11>,
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
    O7,
    O8,
    O9,
    O10,
    O11,
> TryFromPatch<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)>
    for (O1, O2, O3, O4, O5, O6, O7, O8, O9, O10, O11)
{
    fn try_from_value(
        value: (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11),
    ) -> Option<Self> {
        Some((
            value.0.try_into_value()?,
            value.1.try_into_value()?,
            value.2.try_into_value()?,
            value.3.try_into_value()?,
            value.4.try_into_value()?,
            value.5.try_into_value()?,
            value.6.try_into_value()?,
            value.7.try_into_value()?,
            value.8.try_into_value()?,
            value.9.try_into_value()?,
            value.10.try_into_value()?,
        ))
    }
}

impl<
    T1: TryIntoPatch<O1>,
    T2: TryIntoPatch<O2>,
    T3: TryIntoPatch<O3>,
    T4: TryIntoPatch<O4>,
    T5: TryIntoPatch<O5>,
    T6: TryIntoPatch<O6>,
    T7: TryIntoPatch<O7>,
    T8: TryIntoPatch<O8>,
    T9: TryIntoPatch<O9>,
    T10: TryIntoPatch<O10>,
    T11: TryIntoPatch<O11>,
    T12: TryIntoPatch<O12>,
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
    O7,
    O8,
    O9,
    O10,
    O11,
    O12,
> TryFromPatch<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)>
    for (O1, O2, O3, O4, O5, O6, O7, O8, O9, O10, O11, O12)
{
    fn try_from_value(
        value: (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12),
    ) -> Option<Self> {
        Some((
            value.0.try_into_value()?,
            value.1.try_into_value()?,
            value.2.try_into_value()?,
            value.3.try_into_value()?,
            value.4.try_into_value()?,
            value.5.try_into_value()?,
            value.6.try_into_value()?,
            value.7.try_into_value()?,
            value.8.try_into_value()?,
            value.9.try_into_value()?,
            value.10.try_into_value()?,
            value.11.try_into_value()?,
        ))
    }
}
use crate::{FromPatch, IntoPatch};

impl<T: IntoPatch<O>, O> FromPatch<(T,)> for (O,) {
    fn from_value(value: (T,)) -> Self {
        (value.0.into_value(),)
    }
}

impl<T1: IntoPatch<O1>, T2: IntoPatch<O2>, O1, O2> FromPatch<(T1, T2)>
    for (O1, O2)
{
    fn from_value(value: (T1, T2)) -> Self {
        (value.0.into_value(), value.1.into_value())
    }
}

impl<T1: IntoPatch<O1>, T2: IntoPatch<O2>, T3: IntoPatch<O3>, O1, O2, O3>
    FromPatch<(T1, T2, T3)> for (O1, O2, O3)
{
    fn from_value(value: (T1, T2, T3)) -> Self {
        (value.0.into_value(), value.1.into_value(), value.2.into_value())
    }
}

impl<
    T1: IntoPatch<O1>,
    T2: IntoPatch<O2>,
    T3: IntoPatch<O3>,
    T4: IntoPatch<O4>,
    O1,
    O2,
    O3,
    O4,
> FromPatch<(T1, T2, T3, T4)> for (O1, O2, O3, O4)
{
    fn from_value(value: (T1, T2, T3, T4)) -> Self {
        (
            value.0.into_value(),
            value.1.into_value(),
            value.2.into_value(),
            value.3.into_value(),
        )
    }
}

impl<
    T1: IntoPatch<O1>,
    T2: IntoPatch<O2>,
    T3: IntoPatch<O3>,
    T4: IntoPatch<O4>,
    T5: IntoPatch<O5>,
    O1,
    O2,
    O3,
    O4,
    O5,
> FromPatch<(T1, T2, T3, T4, T5)> for (O1, O2, O3, O4, O5)
{
    fn from_value(value: (T1, T2, T3, T4, T5)) -> Self {
        (
            value.0.into_value(),
            value.1.into_value(),
            value.2.into_value(),
            value.3.into_value(),
            value.4.into_value(),
        )
    }
}

impl<
    T1: IntoPatch<O1>,
    T2: IntoPatch<O2>,
    T3: IntoPatch<O3>,
    T4: IntoPatch<O4>,
    T5: IntoPatch<O5>,
    T6: IntoPatch<O6>,
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
> FromPatch<(T1, T2, T3, T4, T5, T6)> for (O1, O2, O3, O4, O5, O6)
{
    fn from_value(value: (T1, T2, T3, T4, T5, T6)) -> Self {
        (
            value.0.into_value(),
            value.1.into_value(),
            value.2.into_value(),
            value.3.into_value(),
            value.4.into_value(),
            value.5.into_value(),
        )
    }
}

impl<
    T1: IntoPatch<O1>,
    T2: IntoPatch<O2>,
    T3: IntoPatch<O3>,
    T4: IntoPatch<O4>,
    T5: IntoPatch<O5>,
    T6: IntoPatch<O6>,
    T7: IntoPatch<O7>,
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
    O7,
> FromPatch<(T1, T2, T3, T4, T5, T6, T7)> for (O1, O2, O3, O4, O5, O6, O7)
{
    fn from_value(value: (T1, T2, T3, T4, T5, T6, T7)) -> Self {
        (
            value.0.into_value(),
            value.1.into_value(),
            value.2.into_value(),
            value.3.into_value(),
            value.4.into_value(),
            value.5.into_value(),
            value.6.into_value(),
        )
    }
}

impl<
    T1: IntoPatch<O1>,
    T2: IntoPatch<O2>,
    T3: IntoPatch<O3>,
    T4: IntoPatch<O4>,
    T5: IntoPatch<O5>,
    T6: IntoPatch<O6>,
    T7: IntoPatch<O7>,
    T8: IntoPatch<O8>,
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
    O7,
    O8,
> FromPatch<(T1, T2, T3, T4, T5, T6, T7, T8)>
    for (O1, O2, O3, O4, O5, O6, O7, O8)
{
    fn from_value(value: (T1, T2, T3, T4, T5, T6, T7, T8)) -> Self {
        (
            value.0.into_value(),
            value.1.into_value(),
            value.2.into_value(),
            value.3.into_value(),
            value.4.into_value(),
            value.5.into_value(),
            value.6.into_value(),
            value.7.into_value(),
        )
    }
}

impl<
    T1: IntoPatch<O1>,
    T2: IntoPatch<O2>,
    T3: IntoPatch<O3>,
    T4: IntoPatch<O4>,
    T5: IntoPatch<O5>,
    T6: IntoPatch<O6>,
    T7: IntoPatch<O7>,
    T8: IntoPatch<O8>,
    T9: IntoPatch<O9>,
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
    O7,
    O8,
    O9,
> FromPatch<(T1, T2, T3, T4, T5, T6, T7, T8, T9)>
    for (O1, O2, O3, O4, O5, O6, O7, O8, O9)
{
    fn from_value(value: (T1, T2, T3, T4, T5, T6, T7, T8, T9)) -> Self {
        (
            value.0.into_value(),
            value.1.into_value(),
            value.2.into_value(),
            value.3.into_value(),
            value.4.into_value(),
            value.5.into_value(),
            value.6.into_value(),
            value.7.into_value(),
            value.8.into_value(),
        )
    }
}

impl<
    T1: IntoPatch<O1>,
    T2: IntoPatch<O2>,
    T3: IntoPatch<O3>,
    T4: IntoPatch<O4>,
    T5: IntoPatch<O5>,
    T6: IntoPatch<O6>,
    T7: IntoPatch<O7>,
    T8: IntoPatch<O8>,
    T9: IntoPatch<O9>,
    T10: IntoPatch<O10>,
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
    O7,
    O8,
    O9,
    O10,
> FromPatch<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)>
    for (O1, O2, O3, O4, O5, O6, O7, O8, O9, O10)
{
    fn from_value(value: (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)) -> Self {
        (
            value.0.into_value(),
            value.1.into_value(),
            value.2.into_value(),
            value.3.into_value(),
            value.4.into_value(),
            value.5.into_value(),
            value.6.into_value(),
            value.7.into_value(),
            value.8.into_value(),
            value.9.into_value(),
        )
    }
}

impl<
    T1: IntoPatch<O1>,
    T2: IntoPatch<O2>,
    T3: IntoPatch<O3>,
    T4: IntoPatch<O4>,
    T5: IntoPatch<O5>,
    T6: IntoPatch<O6>,
    T7: IntoPatch<O7>,
    T8: IntoPatch<O8>,
    T9: IntoPatch<O9>,
    T10: IntoPatch<O10>,
    T11: IntoPatch<O11>,
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
    O7,
    O8,
    O9,
    O10,
    O11,
> FromPatch<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)>
    for (O1, O2, O3, O4, O5, O6, O7, O8, O9, O10, O11)
{
    fn from_value(
        value: (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11),
    ) -> Self {
        (
            value.0.into_value(),
            value.1.into_value(),
            value.2.into_value(),
            value.3.into_value(),
            value.4.into_value(),
            value.5.into_value(),
            value.6.into_value(),
            value.7.into_value(),
            value.8.into_value(),
            value.9.into_value(),
            value.10.into_value(),
        )
    }
}

impl<
    T1: IntoPatch<O1>,
    T2: IntoPatch<O2>,
    T3: IntoPatch<O3>,
    T4: IntoPatch<O4>,
    T5: IntoPatch<O5>,
    T6: IntoPatch<O6>,
    T7: IntoPatch<O7>,
    T8: IntoPatch<O8>,
    T9: IntoPatch<O9>,
    T10: IntoPatch<O10>,
    T11: IntoPatch<O11>,
    T12: IntoPatch<O12>,
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
    O7,
    O8,
    O9,
    O10,
    O11,
    O12,
> FromPatch<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)>
    for (O1, O2, O3, O4, O5, O6, O7, O8, O9, O10, O11, O12)
{
    fn from_value(
        value: (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12),
    ) -> Self {
        (
            value.0.into_value(),
            value.1.into_value(),
            value.2.into_value(),
            value.3.into_value(),
            value.4.into_value(),
            value.5.into_value(),
            value.6.into_value(),
            value.7.into_value(),
            value.8.into_value(),
            value.9.into_value(),
            value.10.into_value(),
            value.11.into_value(),
        )
    }
}
