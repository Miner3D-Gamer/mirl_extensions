const impl<T: crate::ConstNumbers128 + [const] crate::UpperBounded> ShapeDirectionType for T {
    default fn bottom_direction() -> Self {
        Self::CONST_5
    }
    default fn bottom_left_direction() -> Self {
        Self::CONST_6
    }
    default fn bottom_right_direction() -> Self {
        Self::CONST_4
    }
    default fn left_direction() -> Self {
        Self::CONST_7
    }
    default fn none_directional() -> Self {
        Self::max_bound()
    }
    default fn right_direction() -> Self {
        Self::CONST_3
    }
    default fn top_direction() -> Self {
        Self::CONST_1
    }
    default fn top_left_direction() -> Self {
        Self::CONST_0
    }
    default fn top_right_direction() -> Self {
        Self::CONST_2
    }
}

/// When implemented, return the value corresponding with the location
///
/// TODO: Add unicode arrows for ever location
pub const trait ShapeDirectionType {
    /// The top left corner
    fn top_left_direction() -> Self;
    /// The top right corner
    fn top_right_direction() -> Self;
    /// The bottom left corner
    fn bottom_left_direction() -> Self;
    /// The bottom right corner
    fn bottom_right_direction() -> Self;
    /// The top edge
    fn top_direction() -> Self;
    /// The right edge
    fn right_direction() -> Self;
    /// The left edge
    fn left_direction() -> Self;
    /// The bottom edge
    fn bottom_direction() -> Self;
    /// Not near an edge
    fn none_directional() -> Self;
}

/// Check if the given direction is true in [`NormalDirections`](mirl_geometry_core::directions::NormalDirections)
pub const trait IsDirectionTrue {
    /// Check if the given direction is true in [`NormalDirections`](mirl_geometry_core::directions::NormalDirections)
    fn is_direction_true(
        &self,
        directions: &mirl_geometry_core::directions::NormalDirections,
    ) -> bool;
}
const impl IsDirectionTrue for u8 {
    fn is_direction_true(
        &self,
        directions: &mirl_geometry_core::directions::NormalDirections,
    ) -> bool {
        match self {
            0 => directions.top_left,
            1 => directions.top,
            2 => directions.top_right,
            3 => directions.right,
            4 => directions.bottom_right,
            5 => directions.bottom,
            6 => directions.bottom_left,
            7 => directions.left,
            _ => false,
        }
    }
}

// TODO: Add this trait to other directions
const impl IsDirectionTrue for mirl_geometry_core::directions::Directions {
    fn is_direction_true(
        &self,
        directions: &mirl_geometry_core::directions::NormalDirections,
    ) -> bool {
        match self {
            Self::North => directions.top,
            Self::South => directions.bottom,
            Self::West => directions.left,
            Self::East => directions.right,
        }
    }
}
/// Allow [`NormalDirections`](mirl_geometry_core::directions::NormalDirections) to itself check if a direction is true
pub const trait IsDirectionTrueForNormalDirection<T: IsDirectionTrue> {
    /// Is the given direction set to true for ourselves
    fn is_direction_true(&self, direction: &T) -> bool;
}

const impl<T: [const] IsDirectionTrue> IsDirectionTrueForNormalDirection<T>
    for mirl_geometry_core::directions::NormalDirections
{
    fn is_direction_true(&self, direction: &T) -> bool {
        direction.is_direction_true(self)
    }
}
