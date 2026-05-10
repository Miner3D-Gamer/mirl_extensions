// u128
impl const FromPatch<u128> for f32 {
    fn from_value(v: u128) -> Self {
        v as Self
    }
}
impl const FromPatch<u128> for f64 {
    fn from_value(v: u128) -> Self {
        v as Self
    }
}
impl const FromPatch<u128> for f16 {
    fn from_value(v: u128) -> Self {
        v as Self
    }
}
impl const FromPatch<u128> for f128 {
    fn from_value(v: u128) -> Self {
        v as Self
    }
}
use crate::FromPatch;
