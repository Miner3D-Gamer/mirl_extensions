use crate::TryFromPatch;

// String conversions
impl<const N: usize> TryFromPatch<[u8; N]> for String {
    fn try_from_value(v: [u8; N]) -> Option<Self> {
        Self::from_utf8(v.to_vec()).ok()
    }
}

impl<const N: usize> TryFromPatch<&[u8; N]> for String {
    fn try_from_value(v: &[u8; N]) -> Option<Self> {
        Self::from_utf8(v.to_vec()).ok()
    }
}

impl TryFromPatch<Vec<u8>> for String {
    fn try_from_value(v: Vec<u8>) -> Option<Self> {
        Self::from_utf8(v).ok()
    }
}

impl TryFromPatch<&[u8]> for String {
    fn try_from_value(v: &[u8]) -> Option<Self> {
        Self::from_utf8(v.to_vec()).ok()
    }
}

// Unsigned integers
impl TryFromPatch<String> for u8 {
    fn try_from_value(v: String) -> Option<Self> {
        v.parse().ok()
    }
}

impl TryFromPatch<&str> for u8 {
    fn try_from_value(v: &str) -> Option<Self> {
        v.parse().ok()
    }
}

impl TryFromPatch<String> for u16 {
    fn try_from_value(v: String) -> Option<Self> {
        v.parse().ok()
    }
}

impl TryFromPatch<&str> for u16 {
    fn try_from_value(v: &str) -> Option<Self> {
        v.parse().ok()
    }
}

impl TryFromPatch<String> for u32 {
    fn try_from_value(v: String) -> Option<Self> {
        v.parse().ok()
    }
}

impl TryFromPatch<&str> for u32 {
    fn try_from_value(v: &str) -> Option<Self> {
        v.parse().ok()
    }
}

impl TryFromPatch<String> for u64 {
    fn try_from_value(v: String) -> Option<Self> {
        v.parse().ok()
    }
}

impl TryFromPatch<&str> for u64 {
    fn try_from_value(v: &str) -> Option<Self> {
        v.parse().ok()
    }
}

impl TryFromPatch<String> for u128 {
    fn try_from_value(v: String) -> Option<Self> {
        v.parse().ok()
    }
}

impl TryFromPatch<&str> for u128 {
    fn try_from_value(v: &str) -> Option<Self> {
        v.parse().ok()
    }
}

impl TryFromPatch<String> for usize {
    fn try_from_value(v: String) -> Option<Self> {
        v.parse().ok()
    }
}

impl TryFromPatch<&str> for usize {
    fn try_from_value(v: &str) -> Option<Self> {
        v.parse().ok()
    }
}

// Signed integers
impl TryFromPatch<String> for i8 {
    fn try_from_value(v: String) -> Option<Self> {
        v.parse().ok()
    }
}

impl TryFromPatch<&str> for i8 {
    fn try_from_value(v: &str) -> Option<Self> {
        v.parse().ok()
    }
}

impl TryFromPatch<String> for i16 {
    fn try_from_value(v: String) -> Option<Self> {
        v.parse().ok()
    }
}

impl TryFromPatch<&str> for i16 {
    fn try_from_value(v: &str) -> Option<Self> {
        v.parse().ok()
    }
}

impl TryFromPatch<String> for i32 {
    fn try_from_value(v: String) -> Option<Self> {
        v.parse().ok()
    }
}

impl TryFromPatch<&str> for i32 {
    fn try_from_value(v: &str) -> Option<Self> {
        v.parse().ok()
    }
}

impl TryFromPatch<String> for i64 {
    fn try_from_value(v: String) -> Option<Self> {
        v.parse().ok()
    }
}

impl TryFromPatch<&str> for i64 {
    fn try_from_value(v: &str) -> Option<Self> {
        v.parse().ok()
    }
}

impl TryFromPatch<String> for i128 {
    fn try_from_value(v: String) -> Option<Self> {
        v.parse().ok()
    }
}

impl TryFromPatch<&str> for i128 {
    fn try_from_value(v: &str) -> Option<Self> {
        v.parse().ok()
    }
}

impl TryFromPatch<String> for isize {
    fn try_from_value(v: String) -> Option<Self> {
        v.parse().ok()
    }
}

impl TryFromPatch<&str> for isize {
    fn try_from_value(v: &str) -> Option<Self> {
        v.parse().ok()
    }
}

// Floating point
impl TryFromPatch<String> for f32 {
    fn try_from_value(v: String) -> Option<Self> {
        v.parse().ok()
    }
}

impl TryFromPatch<&str> for f32 {
    fn try_from_value(v: &str) -> Option<Self> {
        v.parse().ok()
    }
}

impl TryFromPatch<String> for f64 {
    fn try_from_value(v: String) -> Option<Self> {
        v.parse().ok()
    }
}

impl TryFromPatch<&str> for f64 {
    fn try_from_value(v: &str) -> Option<Self> {
        v.parse().ok()
    }
}
impl TryFromPatch<String> for f128 {
    fn try_from_value(v: String) -> Option<Self> {
        v.parse::<f64>().ok().map(|x| x as Self) // TODO: Replace with native call once available
    }
}

impl TryFromPatch<&str> for f128 {
    fn try_from_value(v: &str) -> Option<Self> {
        v.parse::<f64>().ok().map(|x| x as Self)
    }
}

impl TryFromPatch<String> for f16 {
    fn try_from_value(v: String) -> Option<Self> {
        v.parse().ok()
    }
}

impl TryFromPatch<&str> for f16 {
    fn try_from_value(v: &str) -> Option<Self> {
        v.parse().ok()
    }
}
// Boolean
impl TryFromPatch<String> for bool {
    fn try_from_value(v: String) -> Option<Self> {
        v.parse().ok()
    }
}

impl TryFromPatch<&str> for bool {
    fn try_from_value(v: &str) -> Option<Self> {
        v.parse().ok()
    }
}

// Character
impl TryFromPatch<String> for char {
    fn try_from_value(v: String) -> Option<Self> {
        let mut chars = v.chars();
        match (chars.next(), chars.next()) {
            (Some(c), None) => Some(c),
            _ => None,
        }
    }
}

impl TryFromPatch<&str> for char {
    fn try_from_value(v: &str) -> Option<Self> {
        let mut chars = v.chars();
        match (chars.next(), chars.next()) {
            (Some(c), None) => Some(c),
            _ => None,
        }
    }
}
