use crate::FromPatch;

impl FromPatch<String> for Vec<u8> {
    fn from_value(v: String) -> Self {
        v.into_bytes()
    }
}

impl FromPatch<&str> for Vec<u8> {
    fn from_value(v: &str) -> Self {
        v.as_bytes().to_vec()
    }
}
impl const FromPatch<Self> for String {
    fn from_value(v: Self) -> Self {
        v
    }
}

impl FromPatch<&str> for String {
    fn from_value(v: &str) -> Self {
        v.to_string()
    }
}

impl<'a> FromPatch<&'a String> for &'a [u8] {
    fn from_value(value: &'a String) -> Self {
        value.as_bytes()
    }
}
