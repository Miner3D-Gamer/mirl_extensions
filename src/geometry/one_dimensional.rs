/// Obtain the 1d coordinate at which the object is located
pub const trait Get1DCoordinate<T> {
    #[must_use]
    /// Get the component of a 1d coordinate
    fn get_pos_1d(&self) -> T;
    /// Set the component of a 1d coordinate
    fn set_pos_1d(&mut self, val: T);
}

/// Helper trait for working with 1d coordinates
pub const trait Get1DCoordinateHelper<T> {
    #[must_use]
    /// Obtain the 1d coordinate at which the object is located
    fn get_pos_1d_tuple(&self) -> (T,);
    /// Set the 1d coordinate
    fn set_pos_1d_tuple(&mut self, pos: (T,));
}

impl<T: Get1DCoordinate<N>, N> Get1DCoordinateHelper<N> for T {
    default fn get_pos_1d_tuple(&self) -> (N,) {
        (self.get_pos_1d(),)
    }
    default fn set_pos_1d_tuple(&mut self, pos: (N,)) {
        self.set_pos_1d(pos.0);
    }
}