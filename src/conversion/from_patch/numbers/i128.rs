// i128
impl const FromPatch<i128> for f32 {
    fn from_value(v: i128) -> Self {
        v as Self
    }
}
impl const FromPatch<i128> for f64 {
    fn from_value(v: i128) -> Self {
        v as Self
    }
}
impl const FromPatch<i128> for f16 {
    fn from_value(v: i128) -> Self {
        v as Self
    }
}
impl const FromPatch<i128> for f128 {
    fn from_value(v: i128) -> Self {
        v as Self
    }
}
use crate::FromPatch;
