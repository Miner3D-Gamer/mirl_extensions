/// Obtain the 4d coordinate at which the object is located
pub const trait Get4DCoordinate<T> {
    #[must_use]
    /// Get the x component of a 4d coordinate
    fn get_pos_4d_x(&self) -> T;
    #[must_use]
    /// Get the y component of a 4d coordinate
    fn get_pos_4d_y(&self) -> T;
    #[must_use]
    /// Get the z component of a 4d coordinate
    fn get_pos_4d_z(&self) -> T;
    #[must_use]
    /// Get the w component of a 4d coordinate
    fn get_pos_4d_w(&self) -> T;

    /// Set the x component of a 4d coordinate
    fn set_pos_4d_x(&mut self, val: T);
    /// Set the y component of a 4d coordinate
    fn set_pos_4d_y(&mut self, val: T);
    /// Set the z component of a 4d coordinate
    fn set_pos_4d_z(&mut self, val: T);
    /// Set the w component of a 4d coordinate
    fn set_pos_4d_w(&mut self, val: T);
}

/// Helper trait for working with 4d coordinates
pub const trait Get4DCoordinateHelper<T> {
    #[must_use]
    /// Obtain the 4d coordinate at which the object is located
    fn get_pos_4d(&self) -> (T, T, T, T);
    /// Set the 4d coordinate
    fn set_pos_4d(&mut self, pos: (T, T, T, T));
}

impl<T: Get4DCoordinate<N>, N> Get4DCoordinateHelper<N> for T {
    default fn get_pos_4d(&self) -> (N, N, N, N) {
        (
            self.get_pos_4d_x(),
            self.get_pos_4d_y(),
            self.get_pos_4d_z(),
            self.get_pos_4d_w(),
        )
    }

    default fn set_pos_4d(&mut self, pos: (N, N, N, N)) {
        self.set_pos_4d_x(pos.0);
        self.set_pos_4d_y(pos.1);
        self.set_pos_4d_z(pos.2);
        self.set_pos_4d_w(pos.3);
    }
}