/// Obtain the 2d coordinate at which the object is located
pub const trait Get2DCoordinate<T> {
    #[must_use]
    /// Get the x component of a 2d coordinate
    fn get_pos_2d_x(&self) -> T;
    #[must_use]
    /// Get the y component of a 2d coordinate
    fn get_pos_2d_y(&self) -> T;
    /// Set the x component of a 2d coordinate
    fn set_pos_2d_x(&mut self, val: T);
    /// Set the y component of a 2d coordinate
    fn set_pos_2d_y(&mut self, val: T);
}
/// An automatically implementing helper trait for [`Get2DCoordinate`]
pub const trait Get2DCoordinateHelper<T> {
    #[must_use]
    /// Obtain the 2d coordinate at which the object is located
    fn get_pos_2d(&self) -> (T, T);
    /// Get a mutable
    fn set_pos_2d(&mut self, pos: (T, T));
}
impl<T: Get2DCoordinate<N>, N> Get2DCoordinateHelper<N> for T {
    default fn get_pos_2d(&self) -> (N, N) {
        (self.get_pos_2d_x(), self.get_pos_2d_y())
    }
    default fn set_pos_2d(&mut self, pos: (N, N)) {
        self.set_pos_2d_x(pos.0);
        self.set_pos_2d_y(pos.1);
    }
}
/// Obtain the 2d coordinate at which the middle of the object is located
pub const trait Get2DCoordinateCenter<T> {
    #[must_use]
    /// Obtain the 2d coordinate at which the middle of the object is located
    fn get_pos_2d_of_center(&self) -> (T, T);
}
/// In what direction if the other point relative to this one
pub const trait Directions2DFromTo<T, N> {
    /// In what direction if the other point relative to this one
    fn get_direction_relative_to<const CS: bool>(
        &self,
        other: T,
        target_ratio: N,
    ) -> mirl_core::directions::Directions;
}
impl<
    S: [const] Get2DCoordinateCenter<N>,
    T: [const] Get2DCoordinateCenter<N>,
    N: [const] core::ops::Sub<Output = N>
        + crate::ConstZero
        + [const] core::cmp::PartialOrd
        + [const] crate::Abs
        + [const] core::ops::Mul<Output = N>
        + Copy,
> const Directions2DFromTo<&T, N> for S
{
    default fn get_direction_relative_to<const CS: bool>(
        &self,
        other: &T,
        target_ratio: N,
    ) -> mirl_core::directions::Directions {
        let current = self.get_pos_2d_of_center();
        let target = other.get_pos_2d_of_center();
        let margin_x = target.0 - current.0;
        let margin_y = target.1 - current.1;

        if margin_y.abs() > margin_x.abs() * target_ratio {
            if (CS && margin_y > N::ZERO) || (!CS && margin_y < N::ZERO) {
                mirl_core::directions::Directions::North
            } else {
                mirl_core::directions::Directions::South
            }
        } else if margin_x > N::ZERO {
            mirl_core::directions::Directions::East
        } else {
            mirl_core::directions::Directions::West
        }
    }
}
