// i32
impl const TryFromPatch<i32> for i8 {
    fn try_from_value(v: i32) -> Option<Self> {
        Self::try_from(v).ok()
    }
}
impl const TryFromPatch<i32> for i16 {
    fn try_from_value(v: i32) -> Option<Self> {
        Self::try_from(v).ok()
    }
}
impl const TryFromPatch<i32> for u8 {
    fn try_from_value(v: i32) -> Option<Self> {
        Self::try_from(v).ok()
    }
}
impl const TryFromPatch<i32> for u16 {
    fn try_from_value(v: i32) -> Option<Self> {
        Self::try_from(v).ok()
    }
}
impl const TryFromPatch<i32> for u32 {
    fn try_from_value(v: i32) -> Option<Self> {
        Self::try_from(v).ok()
    }
}
impl const TryFromPatch<i32> for u64 {
    fn try_from_value(v: i32) -> Option<Self> {
        Self::try_from(v).ok()
    }
}
impl const TryFromPatch<i32> for u128 {
    fn try_from_value(v: i32) -> Option<Self> {
        Self::try_from(v).ok()
    }
}
impl const TryFromPatch<i32> for usize {
    fn try_from_value(v: i32) -> Option<Self> {
        Self::try_from(v).ok()
    }
}
use crate::TryFromPatch;
