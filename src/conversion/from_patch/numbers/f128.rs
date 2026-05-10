// f64

impl const FromPatch<f128> for f64 {
    fn from_value(v: f128) -> Self {
        (v as Self)
    }
}
impl const FromPatch<Self> for f128 {
    fn from_value(v: Self) -> Self {
        (v)
    }
}
impl const FromPatch<f128> for f16 {
    fn from_value(v: f128) -> Self {
        (v as Self)
    }
}
impl const FromPatch<f128> for f32 {
    fn from_value(v: f128) -> Self {
        (v as Self)
    }
}
use crate::FromPatch;
