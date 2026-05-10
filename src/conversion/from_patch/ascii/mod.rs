use crate::FromPatch;

#[cfg(feature = "std")]
impl FromPatch<ascii::AsciiString> for String {
    fn from_value(value: ascii::AsciiString) -> Self {
        value.as_str().to_string()
    }
}

impl FromPatch<ascii::AsciiChar> for char {
    fn from_value(value: ascii::AsciiChar) -> Self {
        value.as_char()
    }
}

impl<'a> FromPatch<&'a ascii::AsciiStr> for &'a str {
    fn from_value(value: &'a ascii::AsciiStr) -> Self {
        value.as_str()
    }
}
