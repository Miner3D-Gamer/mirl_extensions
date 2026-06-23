use crate::*;
use mirl_input::mouse::{MouseButton, cursors::CursorResolution};

/// Get the size of the given cursor resolution
pub const trait GetCursorResolution {
    #[must_use]
    /// Get the size of the cursor in pixels
    fn get_cursor_resolution<T: [const] FromPatch<u8>>(&self) -> T;
    #[must_use]
    /// Try to get the size of the cursor in pixels
    fn try_get_cursor_resolution<T: [const] TryFromPatch<u8>>(&self) -> Option<T>;
}
const impl GetCursorResolution for CursorResolution {
    fn get_cursor_resolution<T: [const] FromPatch<u8>>(&self) -> T {
        T::from_value(self.get_size_pos())
    }
    fn try_get_cursor_resolution<T: [const] TryFromPatch<u8>>(&self) -> Option<T> {
        T::try_from_value(self.get_size_pos())
    }
}

/// Turn the mouse button into a logical number
pub const trait MouseButtonToNumber {
    /// Turn the mouse button into a logical number
    fn to_number<T: [const] UpperBounded + ConstNumbers128 + ConstOne + ConstZero>(&self) -> T;
}
const impl MouseButtonToNumber for MouseButton {
    fn to_number<T: [const] UpperBounded + ConstNumbers128 + ConstOne + ConstZero>(&self) -> T {
        match self {
            MouseButton::Left => T::CONST_0,
            MouseButton::Right => T::CONST_1,
            MouseButton::Middle => T::CONST_2,
            MouseButton::Extra1 => T::CONST_3,
            MouseButton::Extra2 => T::CONST_4,
            MouseButton::Extra3 => T::CONST_5,
            MouseButton::Extra4 => T::CONST_6,
            MouseButton::Unsupported => T::max_bound(),
        }
    }
}
