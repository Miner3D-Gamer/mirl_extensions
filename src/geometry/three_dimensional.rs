/// Obtain the 3d coordinate at which the object is located
pub const trait Get3DCoordinate<T> {
    #[must_use]
    /// Get the x component of a 3d coordinate
    fn get_pos_3d_x(&self) -> T;
    #[must_use]
    /// Get the y component of a 3d coordinate
    fn get_pos_3d_y(&self) -> T;
    #[must_use]
    /// Get the z component of a 3d coordinate
    fn get_pos_3d_z(&self) -> T;

    /// Set the x component of a 3d coordinate
    fn set_pos_3d_x(&mut self, val: T);
    /// Set the y component of a 3d coordinate
    fn set_pos_3d_y(&mut self, val: T);
    /// Set the z component of a 3d coordinate
    fn set_pos_3d_z(&mut self, val: T);
}

/// Helper trait for working with 3d coordinates
pub const trait Get3DCoordinateHelper<T> {
    #[must_use]
    /// Obtain the 3d coordinate at which the object is located
    fn get_pos_3d(&self) -> (T, T, T);
    /// Set the 3d coordinate
    fn set_pos_3d(&mut self, pos: (T, T, T));
}

impl<T: Get3DCoordinate<N>, N> Get3DCoordinateHelper<N> for T {
    default fn get_pos_3d(&self) -> (N, N, N) {
        (
            self.get_pos_3d_x(),
            self.get_pos_3d_y(),
            self.get_pos_3d_z(),
        )
    }

    default fn set_pos_3d(&mut self, pos: (N, N, N)) {
        self.set_pos_3d_x(pos.0);
        self.set_pos_3d_y(pos.1);
        self.set_pos_3d_z(pos.2);
    }
}