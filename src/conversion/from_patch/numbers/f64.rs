// f64

impl const FromPatch<f64> for f128 {
    fn from_value(v: f64) -> Self {
        v as Self
    }
}
impl const FromPatch<f64> for f16 {
    fn from_value(v: f64) -> Self {
        v as Self
    }
}
impl const FromPatch<f64> for f32 {
    fn from_value(v: f64) -> Self {
        v as Self
    }
}
use crate::FromPatch;
