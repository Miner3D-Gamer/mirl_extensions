// u64
impl const FromPatch<u64> for i128 {
    fn from_value(v: u64) -> Self {
        v as Self
    }
}
impl const FromPatch<u64> for u128 {
    fn from_value(v: u64) -> Self {
        v as Self
    }
}
impl const FromPatch<u64> for usize {
    fn from_value(v: u64) -> Self {
        v as Self
    }
}
impl const FromPatch<u64> for f32 {
    fn from_value(v: u64) -> Self {
        v as Self
    }
}
impl const FromPatch<u64> for f64 {
    fn from_value(v: u64) -> Self {
        v as Self
    }
}
impl const FromPatch<u64> for f16 {
    fn from_value(v: u64) -> Self {
        v as Self
    }
}
impl const FromPatch<u64> for f128 {
    fn from_value(v: u64) -> Self {
        v as Self
    }
}
use crate::FromPatch;
