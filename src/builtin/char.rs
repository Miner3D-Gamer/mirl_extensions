/// A trait from getting a char from
pub const trait CharGetPos {
    /// Get the character at the given char position
    fn get_at(&mut self, position: usize) -> Option<char>;

    /// Get the character at the given char position
    ///
    /// # Safety
    /// Doesn't check if the position actually exists
    unsafe fn get_at_unsafe(&mut self, position: usize) -> char;
}
impl CharGetPos for core::str::Chars<'_> {
    fn get_at(&mut self, position: usize) -> Option<char> {
        let _ = self.advance_by(position);
        self.next()
    }
    unsafe fn get_at_unsafe(&mut self, position: usize) -> char {
        let _ = self.advance_by(position);
        unsafe { self.next().unwrap_unchecked() }
    }
}
