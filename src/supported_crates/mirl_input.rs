use mirl_input::mouse::cursors::CursorResolution;
use crate::*;

/// Get the size of the given cursor resolution
pub const trait GetCursorResolution {
    #[must_use]
    /// Get the size of the cursor in pixels
    fn get_cursor_resolution<T: [const] FromPatch<u8>>(&self) -> T;
    #[must_use]
    /// Try to get the size of the cursor in pixels
    fn try_get_cursor_resolution<T: [const] TryFromPatch<u8>>(
        &self,
    ) -> Option<T>;
}
impl const GetCursorResolution for CursorResolution {
    fn get_cursor_resolution<T: [const] FromPatch<u8>>(&self) -> T {
        T::from_value(self.get_size_pos())
    }
    fn try_get_cursor_resolution<T: [const] TryFromPatch<u8>>(
        &self,
    ) -> Option<T> {
        T::try_from_value(self.get_size_pos())
    }
}
