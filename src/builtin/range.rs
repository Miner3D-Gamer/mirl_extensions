/// Additional functions for `core::ops::Range` (val..val)
pub const trait RangeExtension<T, F> {
    /// If the range goes from 10 to 20 and a 0.5 is inputted, 15 is returned
    fn get_value_from_percent(&self, percentage: F) -> Option<T>;
    /// If the range goes from 10 to 20 and a 15 is inputted, 0.5 is returned
    fn get_percent_from_value(&self, value: T) -> Option<F>;
}

impl<
    T: [const] crate::TryIntoPatch<F> + Copy,
    F: [const] crate::TryIntoPatch<T>
        + [const] core::ops::Mul<Output = F>
        + [const] core::ops::Div<Output = F>
        + [const] crate::Round
        + Copy,
> const RangeExtension<T, F> for core::ops::Range<T>
{
    fn get_value_from_percent(&self, percentage: F) -> Option<T> {
        ((self.end.try_into_value()? * percentage).round()).try_into_value()
    }
    fn get_percent_from_value(&self, value: T) -> Option<F> {
        Some((value).try_into_value()? / ((self.end).try_into_value()?))
    }
}
