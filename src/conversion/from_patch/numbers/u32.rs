// u32
impl const FromPatch<u32> for i64 {
    fn from_value(v: u32) -> Self {
        v as Self
    }
}
impl const FromPatch<u32> for i128 {
    fn from_value(v: u32) -> Self {
        v as Self
    }
}
impl const FromPatch<u32> for u64 {
    fn from_value(v: u32) -> Self {
        v as Self
    }
}
impl const FromPatch<u32> for u128 {
    fn from_value(v: u32) -> Self {
        v as Self
    }
}
impl const FromPatch<u32> for usize {
    fn from_value(v: u32) -> Self {
        v as Self
    }
}
impl const FromPatch<u32> for f32 {
    fn from_value(v: u32) -> Self {
        v as Self
    }
}
impl const FromPatch<u32> for f64 {
    fn from_value(v: u32) -> Self {
        v as Self
    }
}
impl const FromPatch<u32> for f16 {
    fn from_value(v: u32) -> Self {
        v as Self
    }
}
impl const FromPatch<u32> for f128 {
    fn from_value(v: u32) -> Self {
        v as Self
    }
}
use crate::FromPatch;
