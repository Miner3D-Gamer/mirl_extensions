#![allow(clippy::type_complexity)]
use core::marker::Destruct;

use crate::TryIntoPatch;

/// Converts a tuple into another tuple of the same size with converted element types
pub const trait TryTupleInto<T> {
    /// The tuple version of `try_into()`
    fn try_tuple_into(self) -> Option<T>;
}

impl<Target, Source: [const] TryIntoPatch<Target> + [const] Destruct> const
    TryTupleInto<(Target,)> for (Source,)
where
    Target: [const] Destruct,
{
    fn try_tuple_into(self) -> Option<(Target,)> {
        Some((self.0.try_into_value()?,))
    }
}

impl<Target, Source0, Source1> const TryTupleInto<(Target, Target)>
    for (Source0, Source1)
where
    Source0: [const] TryIntoPatch<Target> + [const] Destruct,
    Source1: [const] TryIntoPatch<Target> + [const] Destruct,
    Target: [const] Destruct,
{
    fn try_tuple_into(self) -> Option<(Target, Target)> {
        Some((self.0.try_into_value()?, self.1.try_into_value()?))
    }
}

impl<Target, Source0, Source1, Source2> const
    TryTupleInto<(Target, Target, Target)> for (Source0, Source1, Source2)
where
    Source0: [const] TryIntoPatch<Target> + [const] Destruct,
    Source1: [const] TryIntoPatch<Target> + [const] Destruct,
    Source2: [const] TryIntoPatch<Target> + [const] Destruct,
    Target: [const] Destruct,
{
    fn try_tuple_into(self) -> Option<(Target, Target, Target)> {
        Some((
            self.0.try_into_value()?,
            self.1.try_into_value()?,
            self.2.try_into_value()?,
        ))
    }
}

impl<Target, Source0, Source1, Source2, Source3> const
    TryTupleInto<(Target, Target, Target, Target)>
    for (Source0, Source1, Source2, Source3)
where
    Source0: [const] TryIntoPatch<Target> + [const] Destruct,
    Source1: [const] TryIntoPatch<Target> + [const] Destruct,
    Source2: [const] TryIntoPatch<Target> + [const] Destruct,
    Source3: [const] TryIntoPatch<Target> + [const] Destruct,
    Target: [const] Destruct,
{
    fn try_tuple_into(self) -> Option<(Target, Target, Target, Target)> {
        Some((
            self.0.try_into_value()?,
            self.1.try_into_value()?,
            self.2.try_into_value()?,
            self.3.try_into_value()?,
        ))
    }
}

impl<Target, Source0, Source1, Source2, Source3, Source4> const
    TryTupleInto<(Target, Target, Target, Target, Target)>
    for (Source0, Source1, Source2, Source3, Source4)
where
    Source0: [const] TryIntoPatch<Target> + [const] Destruct,
    Source1: [const] TryIntoPatch<Target> + [const] Destruct,
    Source2: [const] TryIntoPatch<Target> + [const] Destruct,
    Source3: [const] TryIntoPatch<Target> + [const] Destruct,
    Source4: [const] TryIntoPatch<Target> + [const] Destruct,
    Target: [const] Destruct,
{
    fn try_tuple_into(
        self,
    ) -> Option<(Target, Target, Target, Target, Target)> {
        Some((
            self.0.try_into_value()?,
            self.1.try_into_value()?,
            self.2.try_into_value()?,
            self.3.try_into_value()?,
            self.4.try_into_value()?,
        ))
    }
}

impl<Target, Source0, Source1, Source2, Source3, Source4, Source5> const
    TryTupleInto<(Target, Target, Target, Target, Target, Target)>
    for (Source0, Source1, Source2, Source3, Source4, Source5)
where
    Source0: [const] TryIntoPatch<Target> + [const] Destruct,
    Source1: [const] TryIntoPatch<Target> + [const] Destruct,
    Source2: [const] TryIntoPatch<Target> + [const] Destruct,
    Source3: [const] TryIntoPatch<Target> + [const] Destruct,
    Source4: [const] TryIntoPatch<Target> + [const] Destruct,
    Source5: [const] TryIntoPatch<Target> + [const] Destruct,
    Target: [const] Destruct,
{
    fn try_tuple_into(
        self,
    ) -> Option<(Target, Target, Target, Target, Target, Target)> {
        Some((
            self.0.try_into_value()?,
            self.1.try_into_value()?,
            self.2.try_into_value()?,
            self.3.try_into_value()?,
            self.4.try_into_value()?,
            self.5.try_into_value()?,
        ))
    }
}

impl<Target, Source0, Source1, Source2, Source3, Source4, Source5, Source6> const
    TryTupleInto<(Target, Target, Target, Target, Target, Target, Target)>
    for (Source0, Source1, Source2, Source3, Source4, Source5, Source6)
where
    Source0: [const] TryIntoPatch<Target> + [const] Destruct,
    Source1: [const] TryIntoPatch<Target> + [const] Destruct,
    Source2: [const] TryIntoPatch<Target> + [const] Destruct,
    Source3: [const] TryIntoPatch<Target> + [const] Destruct,
    Source4: [const] TryIntoPatch<Target> + [const] Destruct,
    Source5: [const] TryIntoPatch<Target> + [const] Destruct,
    Source6: [const] TryIntoPatch<Target> + [const] Destruct,
    Target: [const] Destruct,
{
    fn try_tuple_into(
        self,
    ) -> Option<(Target, Target, Target, Target, Target, Target, Target)> {
        Some((
            self.0.try_into_value()?,
            self.1.try_into_value()?,
            self.2.try_into_value()?,
            self.3.try_into_value()?,
            self.4.try_into_value()?,
            self.5.try_into_value()?,
            self.6.try_into_value()?,
        ))
    }
}

impl<
    Target,
    Source0,
    Source1,
    Source2,
    Source3,
    Source4,
    Source5,
    Source6,
    Source7,
> const
    TryTupleInto<(
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
    )>
    for (Source0, Source1, Source2, Source3, Source4, Source5, Source6, Source7)
where
    Source0: [const] TryIntoPatch<Target> + [const] Destruct,
    Source1: [const] TryIntoPatch<Target> + [const] Destruct,
    Source2: [const] TryIntoPatch<Target> + [const] Destruct,
    Source3: [const] TryIntoPatch<Target> + [const] Destruct,
    Source4: [const] TryIntoPatch<Target> + [const] Destruct,
    Source5: [const] TryIntoPatch<Target> + [const] Destruct,
    Source6: [const] TryIntoPatch<Target> + [const] Destruct,
    Source7: [const] TryIntoPatch<Target> + [const] Destruct,
    Target: [const] Destruct,
{
    fn try_tuple_into(
        self,
    ) -> Option<(Target, Target, Target, Target, Target, Target, Target, Target)>
    {
        Some((
            self.0.try_into_value()?,
            self.1.try_into_value()?,
            self.2.try_into_value()?,
            self.3.try_into_value()?,
            self.4.try_into_value()?,
            self.5.try_into_value()?,
            self.6.try_into_value()?,
            self.7.try_into_value()?,
        ))
    }
}

impl<
    Target,
    Source0,
    Source1,
    Source2,
    Source3,
    Source4,
    Source5,
    Source6,
    Source7,
    Source8,
> const
    TryTupleInto<(
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
    )>
    for (
        Source0,
        Source1,
        Source2,
        Source3,
        Source4,
        Source5,
        Source6,
        Source7,
        Source8,
    )
where
    Source0: [const] TryIntoPatch<Target> + [const] Destruct,
    Source1: [const] TryIntoPatch<Target> + [const] Destruct,
    Source2: [const] TryIntoPatch<Target> + [const] Destruct,
    Source3: [const] TryIntoPatch<Target> + [const] Destruct,
    Source4: [const] TryIntoPatch<Target> + [const] Destruct,
    Source5: [const] TryIntoPatch<Target> + [const] Destruct,
    Source6: [const] TryIntoPatch<Target> + [const] Destruct,
    Source7: [const] TryIntoPatch<Target> + [const] Destruct,
    Source8: [const] TryIntoPatch<Target> + [const] Destruct,
    Target: [const] Destruct,
{
    fn try_tuple_into(
        self,
    ) -> Option<(
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
    )> {
        Some((
            self.0.try_into_value()?,
            self.1.try_into_value()?,
            self.2.try_into_value()?,
            self.3.try_into_value()?,
            self.4.try_into_value()?,
            self.5.try_into_value()?,
            self.6.try_into_value()?,
            self.7.try_into_value()?,
            self.8.try_into_value()?,
        ))
    }
}

impl<
    Target,
    Source0,
    Source1,
    Source2,
    Source3,
    Source4,
    Source5,
    Source6,
    Source7,
    Source8,
    Source9,
> const
    TryTupleInto<(
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
    )>
    for (
        Source0,
        Source1,
        Source2,
        Source3,
        Source4,
        Source5,
        Source6,
        Source7,
        Source8,
        Source9,
    )
where
    Source0: [const] TryIntoPatch<Target> + [const] Destruct,
    Source1: [const] TryIntoPatch<Target> + [const] Destruct,
    Source2: [const] TryIntoPatch<Target> + [const] Destruct,
    Source3: [const] TryIntoPatch<Target> + [const] Destruct,
    Source4: [const] TryIntoPatch<Target> + [const] Destruct,
    Source5: [const] TryIntoPatch<Target> + [const] Destruct,
    Source6: [const] TryIntoPatch<Target> + [const] Destruct,
    Source7: [const] TryIntoPatch<Target> + [const] Destruct,
    Source8: [const] TryIntoPatch<Target> + [const] Destruct,
    Source9: [const] TryIntoPatch<Target> + [const] Destruct,
    Target: [const] Destruct,
{
    fn try_tuple_into(
        self,
    ) -> Option<(
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
    )> {
        Some((
            self.0.try_into_value()?,
            self.1.try_into_value()?,
            self.2.try_into_value()?,
            self.3.try_into_value()?,
            self.4.try_into_value()?,
            self.5.try_into_value()?,
            self.6.try_into_value()?,
            self.7.try_into_value()?,
            self.8.try_into_value()?,
            self.9.try_into_value()?,
        ))
    }
}

impl<
    Target,
    Source0,
    Source1,
    Source2,
    Source3,
    Source4,
    Source5,
    Source6,
    Source7,
    Source8,
    Source9,
    Source10,
> const
    TryTupleInto<(
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
    )>
    for (
        Source0,
        Source1,
        Source2,
        Source3,
        Source4,
        Source5,
        Source6,
        Source7,
        Source8,
        Source9,
        Source10,
    )
where
    Source0: [const] TryIntoPatch<Target> + [const] Destruct,
    Source1: [const] TryIntoPatch<Target> + [const] Destruct,
    Source2: [const] TryIntoPatch<Target> + [const] Destruct,
    Source3: [const] TryIntoPatch<Target> + [const] Destruct,
    Source4: [const] TryIntoPatch<Target> + [const] Destruct,
    Source5: [const] TryIntoPatch<Target> + [const] Destruct,
    Source6: [const] TryIntoPatch<Target> + [const] Destruct,
    Source7: [const] TryIntoPatch<Target> + [const] Destruct,
    Source8: [const] TryIntoPatch<Target> + [const] Destruct,
    Source9: [const] TryIntoPatch<Target> + [const] Destruct,
    Source10: [const] TryIntoPatch<Target> + [const] Destruct,
    Target: [const] Destruct,
{
    fn try_tuple_into(
        self,
    ) -> Option<(
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
    )> {
        Some((
            self.0.try_into_value()?,
            self.1.try_into_value()?,
            self.2.try_into_value()?,
            self.3.try_into_value()?,
            self.4.try_into_value()?,
            self.5.try_into_value()?,
            self.6.try_into_value()?,
            self.7.try_into_value()?,
            self.8.try_into_value()?,
            self.9.try_into_value()?,
            self.10.try_into_value()?,
        ))
    }
}

impl<
    Target,
    Source0,
    Source1,
    Source2,
    Source3,
    Source4,
    Source5,
    Source6,
    Source7,
    Source8,
    Source9,
    Source10,
    Source11,
> const
    TryTupleInto<(
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
    )>
    for (
        Source0,
        Source1,
        Source2,
        Source3,
        Source4,
        Source5,
        Source6,
        Source7,
        Source8,
        Source9,
        Source10,
        Source11,
    )
where
    Source0: [const] TryIntoPatch<Target> + [const] Destruct,
    Source1: [const] TryIntoPatch<Target> + [const] Destruct,
    Source2: [const] TryIntoPatch<Target> + [const] Destruct,
    Source3: [const] TryIntoPatch<Target> + [const] Destruct,
    Source4: [const] TryIntoPatch<Target> + [const] Destruct,
    Source5: [const] TryIntoPatch<Target> + [const] Destruct,
    Source6: [const] TryIntoPatch<Target> + [const] Destruct,
    Source7: [const] TryIntoPatch<Target> + [const] Destruct,
    Source8: [const] TryIntoPatch<Target> + [const] Destruct,
    Source9: [const] TryIntoPatch<Target> + [const] Destruct,
    Source10: [const] TryIntoPatch<Target> + [const] Destruct,
    Source11: [const] TryIntoPatch<Target> + [const] Destruct,
    Target: [const] Destruct,
{
    fn try_tuple_into(
        self,
    ) -> Option<(
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
    )> {
        Some((
            self.0.try_into_value()?,
            self.1.try_into_value()?,
            self.2.try_into_value()?,
            self.3.try_into_value()?,
            self.4.try_into_value()?,
            self.5.try_into_value()?,
            self.6.try_into_value()?,
            self.7.try_into_value()?,
            self.8.try_into_value()?,
            self.9.try_into_value()?,
            self.10.try_into_value()?,
            self.11.try_into_value()?,
        ))
    }
}
