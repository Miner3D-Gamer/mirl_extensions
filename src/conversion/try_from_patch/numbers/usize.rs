// usize
impl const TryFromPatch<usize> for i8 {
    fn try_from_value(v: usize) -> Option<Self> {
        Self::try_from(v).ok()
    }
}
impl const TryFromPatch<usize> for i16 {
    fn try_from_value(v: usize) -> Option<Self> {
        Self::try_from(v).ok()
    }
}
impl const TryFromPatch<usize> for i32 {
    fn try_from_value(v: usize) -> Option<Self> {
        Self::try_from(v).ok()
    }
}
impl const TryFromPatch<usize> for i64 {
    fn try_from_value(v: usize) -> Option<Self> {
        Self::try_from(v).ok()
    }
}
impl const TryFromPatch<usize> for isize {
    fn try_from_value(v: usize) -> Option<Self> {
        Self::try_from(v).ok()
    }
}
impl const TryFromPatch<usize> for u8 {
    fn try_from_value(v: usize) -> Option<Self> {
        Self::try_from(v).ok()
    }
}
impl const TryFromPatch<usize> for u16 {
    fn try_from_value(v: usize) -> Option<Self> {
        Self::try_from(v).ok()
    }
}
impl const TryFromPatch<usize> for u32 {
    fn try_from_value(v: usize) -> Option<Self> {
        Self::try_from(v).ok()
    }
}
use crate::TryFromPatch;
