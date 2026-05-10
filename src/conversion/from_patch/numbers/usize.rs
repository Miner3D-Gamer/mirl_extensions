// usize
impl const FromPatch<usize> for i128 {
    fn from_value(v: usize) -> Self {
        v as Self
    }
}
impl const FromPatch<usize> for u64 {
    fn from_value(v: usize) -> Self {
        v as Self
    }
}
impl const FromPatch<usize> for u128 {
    fn from_value(v: usize) -> Self {
        v as Self
    }
}
impl const FromPatch<usize> for f32 {
    fn from_value(v: usize) -> Self {
        v as Self
    }
}
impl const FromPatch<usize> for f64 {
    fn from_value(v: usize) -> Self {
        v as Self
    }
}
impl const FromPatch<usize> for f16 {
    fn from_value(v: usize) -> Self {
        v as Self
    }
}
impl const FromPatch<usize> for f128 {
    fn from_value(v: usize) -> Self {
        v as Self
    }
}
use crate::FromPatch;
