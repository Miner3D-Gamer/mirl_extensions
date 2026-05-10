use crate::TryFromPatch;

#[cfg(feature = "std")]
impl TryFromPatch<String> for ascii::AsciiString {
    fn try_from_value(value: String) -> Option<Self> {
        if value.is_ascii() {
            Some(unsafe { Self::from_ascii_unchecked(value.as_bytes()) })
        } else {
            None
        }
    }
}

impl TryFromPatch<char> for ascii::AsciiChar {
    fn try_from_value(value: char) -> Option<Self> {
        if value.is_ascii() {
            Some(unsafe { Self::from_ascii_unchecked(value as u8) })
        } else {
            None
        }
    }
}

impl<'a> TryFromPatch<&'a str> for &'a ascii::AsciiStr {
    fn try_from_value(value: &'a str) -> Option<Self> {
        if value.is_ascii() {
            // We are just telling the compiler to treat the `&str` as `&ascii::AsciiStr`
            // TODO: Test if this causes UB
            Some(unsafe {
                &*(std::ptr::from_ref::<[u8]>(value.as_bytes()) as *const ascii::AsciiStr)
            })
        } else {
            None
        }
    }
}
