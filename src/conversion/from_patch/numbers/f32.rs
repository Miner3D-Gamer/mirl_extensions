use crate::FromPatch;

// f32
impl const FromPatch<f32> for f16 {
    fn from_value(v: f32) -> Self {
        v as Self
    }
}
impl const FromPatch<f32> for f64 {
    fn from_value(v: f32) -> Self {
        v as Self
    }
}

impl const FromPatch<f32> for f128 {
    fn from_value(v: f32) -> Self {
        v as Self
    }
}
