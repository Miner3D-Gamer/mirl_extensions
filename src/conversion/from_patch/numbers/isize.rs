// isize
impl const FromPatch<isize> for i64 {
    fn from_value(v: isize) -> Self {
        v as Self
    }
}
impl const FromPatch<isize> for i128 {
    fn from_value(v: isize) -> Self {
        v as Self
    }
}
impl const FromPatch<isize> for f32 {
    fn from_value(v: isize) -> Self {
        v as Self
    }
}
impl const FromPatch<isize> for f64 {
    fn from_value(v: isize) -> Self {
        v as Self
    }
}
impl const FromPatch<isize> for f16 {
    fn from_value(v: isize) -> Self {
        v as Self
    }
}
impl const FromPatch<isize> for f128 {
    fn from_value(v: isize) -> Self {
        v as Self
    }
}
use crate::FromPatch;
