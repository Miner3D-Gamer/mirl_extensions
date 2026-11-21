use crate::*;
use mirl_input::mouse::cursors::CursorResolution;

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

#[cfg(all(feature = "f128", feature = "f16"))]
use mirl_input::mouse::MouseButton;
#[cfg(all(feature = "f128", feature = "f16"))]
/// Turn the mouse button into a logical number
pub const trait MouseButtonToNumber {
    /// Turn the mouse button into a logical number
    fn to_number<T: [const] UpperBounded + [const] Numbers2Pow6 + [const] One + [const] Zero>(
        &self,
    ) -> T;
}
#[cfg(all(feature = "f128", feature = "f16"))]
const impl MouseButtonToNumber for MouseButton {
    fn to_number<T: [const] UpperBounded + [const] Numbers2Pow6 + [const] One + [const] Zero>(
        &self,
    ) -> T {
        match self {
            MouseButton::Left => T::num_0(),
            MouseButton::Right => T::num_1(),
            MouseButton::Middle => T::num_2(),
            MouseButton::Extra1 => T::num_3(),
            MouseButton::Extra2 => T::num_4(),
            MouseButton::Extra3 => T::num_5(),
            MouseButton::Extra4 => T::num_6(),
            MouseButton::Extra5 => T::num_7(),
            MouseButton::Extra6 => T::num_8(),
        }
    }
}
