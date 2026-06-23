use mirl_geometry_core::text_position::TextPosition;

use crate::{IntoPatch, TryIntoPatch};
/// Get the position of a character from an offset
pub const trait ToTextPosition<Offset> {
    /// Get the position of a character from an offset
    fn to_text_position(&self, offset: Offset) -> TextPosition;
}
/// Get the position of a character from an offset
pub const trait TryToTextPosition<Offset> {
    /// Get the position of a character from an offset
    fn try_to_text_position(&self, offset: Offset) -> Option<TextPosition>;
}
/// Get the position of a character of a given text
pub const trait AsTextPositionOffset {
    /// Get the position of a character of a given text
    fn to_char_offset(self, text: &str) -> TextPosition;
}
/// Get the position of a character of a given text
pub const trait TryAsTextPositionOffset {
    /// Get the position of a character of a given text
    fn try_to_char_offset(self, text: &str) -> Option<TextPosition>;
}
impl<T: TryIntoPatch<usize> + Copy> TryAsTextPositionOffset for T {
    fn try_to_char_offset(self, text: &str) -> Option<TextPosition> {
        Some(TextPosition::from_offset(self.try_into_value()?, text))
    }
}

impl<T: IntoPatch<usize>> AsTextPositionOffset for T {
    fn to_char_offset(self, text: &str) -> TextPosition {
        TextPosition::from_offset(self.into_value(), text)
    }
}
impl<T: AsTextPositionOffset> ToTextPosition<T> for str {
    fn to_text_position(&self, offset: T) -> TextPosition {
        T::to_char_offset(offset, self)
    }
}

impl<T: TryAsTextPositionOffset> TryToTextPosition<T> for str {
    fn try_to_text_position(&self, offset: T) -> Option<TextPosition> {
        T::try_to_char_offset(offset, self)
    }
}

impl<T: TryAsTextPositionOffset> TryToTextPosition<T> for &str {
    fn try_to_text_position(&self, offset: T) -> Option<TextPosition> {
        T::try_to_char_offset(offset, self)
    }
}

impl<T: TryAsTextPositionOffset> TryToTextPosition<T> for String {
    fn try_to_text_position(&self, offset: T) -> Option<TextPosition> {
        T::try_to_char_offset(offset, self)
    }
}
