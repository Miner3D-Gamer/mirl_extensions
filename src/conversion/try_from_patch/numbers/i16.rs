// i16
impl const TryFromPatch<i16> for i8 {
    fn try_from_value(v: i16) -> Option<Self> {
        Self::try_from(v).ok()
    }
}
impl const TryFromPatch<i16> for u8 {
    fn try_from_value(v: i16) -> Option<Self> {
        Self::try_from(v).ok()
    }
}
impl const TryFromPatch<i16> for u16 {
    fn try_from_value(v: i16) -> Option<Self> {
        Self::try_from(v).ok()
    }
}
impl const TryFromPatch<i16> for u32 {
    fn try_from_value(v: i16) -> Option<Self> {
        Self::try_from(v).ok()
    }
}
impl const TryFromPatch<i16> for u64 {
    fn try_from_value(v: i16) -> Option<Self> {
        Self::try_from(v).ok()
    }
}
impl const TryFromPatch<i16> for u128 {
    fn try_from_value(v: i16) -> Option<Self> {
        Self::try_from(v).ok()
    }
}
impl const TryFromPatch<i16> for usize {
    fn try_from_value(v: i16) -> Option<Self> {
        Self::try_from(v).ok()
    }
}
use crate::TryFromPatch;
