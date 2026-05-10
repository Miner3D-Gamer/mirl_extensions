// i16
impl const FromPatch<i16> for i32 {
    fn from_value(v: i16) -> Self {
        v as Self
    }
}
impl const FromPatch<i16> for i64 {
    fn from_value(v: i16) -> Self {
        v as Self
    }
}
impl const FromPatch<i16> for i128 {
    fn from_value(v: i16) -> Self {
        v as Self
    }
}
impl const FromPatch<i16> for isize {
    fn from_value(v: i16) -> Self {
        v as Self
    }
}
impl const FromPatch<i16> for f32 {
    fn from_value(v: i16) -> Self {
        v as Self
    }
}
impl const FromPatch<i16> for f64 {
    fn from_value(v: i16) -> Self {
        v as Self
    }
}
impl const FromPatch<i16> for f16 {
    fn from_value(v: i16) -> Self {
        v as Self
    }
}
impl const FromPatch<i16> for f128 {
    fn from_value(v: i16) -> Self {
        v as Self
    }
}
use crate::FromPatch;
