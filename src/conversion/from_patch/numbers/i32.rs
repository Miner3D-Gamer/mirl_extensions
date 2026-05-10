// i32
impl const FromPatch<i32> for i64 {
    fn from_value(v: i32) -> Self {
        v as Self
    }
}
impl const FromPatch<i32> for i128 {
    fn from_value(v: i32) -> Self {
        v as Self
    }
}
impl const FromPatch<i32> for isize {
    fn from_value(v: i32) -> Self {
        v as Self
    }
}
impl const FromPatch<i32> for f32 {
    fn from_value(v: i32) -> Self {
        v as Self
    }
}
impl const FromPatch<i32> for f64 {
    fn from_value(v: i32) -> Self {
        v as Self
    }
}
impl const FromPatch<i32> for f16 {
    fn from_value(v: i32) -> Self {
        v as Self
    }
}
impl const FromPatch<i32> for f128 {
    fn from_value(v: i32) -> Self {
        v as Self
    }
}
use crate::FromPatch;
