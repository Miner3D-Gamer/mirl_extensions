use core::cell::Cell;

/// Set the value inside the Cell to zero
pub const trait SetCellToZero {
    /// Zero out cell's value
    fn set_zero(&self);
}

impl<T: crate::ConstZero> SetCellToZero for core::cell::Cell<T> {
    fn set_zero(&self) {
        self.replace(T::ZERO);
    }
}

/// Basic arithmetic operations that modify the cell in place.
pub const trait CellIntOps<T> {
    /// Adds `rhs` to the cell's value.
    fn add(&self, rhs: T);
    /// Subtracts `rhs` from the cell's value.
    fn sub(&self, rhs: T);
    /// Multiplies the cell's value by `rhs`.
    fn mul(&self, rhs: T);
    /// Divides the cell's value by `rhs`.
    fn div(&self, rhs: T);
}

/// Basic arithmetic operations that return a new value without modifying the cell.
pub const trait CellIntReturnOps<T> {
    /// Returns the cell's value plus `rhs`.
    fn added(&self, rhs: T) -> T;
    /// Returns the cell's value minus `rhs`.
    fn subtracted(&self, rhs: T) -> T;
    /// Returns the cell's value multiplied by `rhs`.
    fn multiplied(&self, rhs: T) -> T;
    /// Returns the cell's value divided by `rhs`.
    fn divided(&self, rhs: T) -> T;
}

/// Saturating arithmetic operations that modify the cell in place.
pub const trait CellIntSaturatedOps<T> {
    /// Adds `rhs` to the cell's value, saturating at numeric bounds.
    fn saturating_add(&self, rhs: T);
    /// Subtracts `rhs` from the cell's value, saturating at numeric bounds.
    fn saturating_sub(&self, rhs: T);
    /// Multiplies the cell's value by `rhs`, saturating at numeric bounds.
    fn saturating_mul(&self, rhs: T);
}

/// Saturating arithmetic operations that return a new value without modifying the cell.
pub const trait CellIntSaturatedReturnOps<T> {
    /// Returns the cell's value plus `rhs`, saturating at numeric bounds.
    fn saturating_added(&self, rhs: T) -> T;
    /// Returns the cell's value minus `rhs`, saturating at numeric bounds.
    fn saturating_subtracted(&self, rhs: T) -> T;
    /// Returns the cell's value multiplied by `rhs`, saturating at numeric bounds.
    fn saturating_multiplied(&self, rhs: T) -> T;
}

impl<T> const CellIntOps<T> for Cell<T>
where
    T: Copy
        + [const] core::ops::Add<Output = T>
        + [const] core::ops::Sub<Output = T>
        + [const] core::ops::Mul<Output = T>
        + [const] core::ops::Div<Output = T>,
{
    fn add(&self, rhs: T) {
        self.replace(self.get() + rhs);
    }

    fn sub(&self, rhs: T) {
        self.replace(self.get() - rhs);
    }

    fn mul(&self, rhs: T) {
        self.replace(self.get() * rhs);
    }

    fn div(&self, rhs: T) {
        self.replace(self.get() / rhs);
    }
}

impl<T> CellIntReturnOps<T> for Cell<T>
where
    T: Copy
        + core::ops::Add<Output = T>
        + core::ops::Sub<Output = T>
        + core::ops::Mul<Output = T>
        + core::ops::Div<Output = T>,
{
    fn added(&self, rhs: T) -> T {
        let result = self.get() + rhs;
        self.replace(result);
        result
    }

    fn subtracted(&self, rhs: T) -> T {
        let result = self.get() - rhs;
        self.replace(result);
        result
    }

    fn multiplied(&self, rhs: T) -> T {
        let result = self.get() * rhs;
        self.replace(result);
        result
    }

    fn divided(&self, rhs: T) -> T {
        let result = self.get() / rhs;
        self.replace(result);
        result
    }
}

impl<T> const CellIntSaturatedOps<T> for Cell<T>
where
    T: Copy
        + [const] crate::SaturatingAdd<T, Output = T>
        + [const] crate::SaturatingSub<T, Output = T>
        + [const] crate::SaturatingMul<T, Output = T>,
{
    fn saturating_add(&self, rhs: T) {
        self.replace(self.get().saturating_add(rhs));
    }

    fn saturating_sub(&self, rhs: T) {
        self.replace(self.get().saturating_sub(rhs));
    }

    fn saturating_mul(&self, rhs: T) {
        self.replace(self.get().saturating_mul(rhs));
    }
}

impl<T> const CellIntSaturatedReturnOps<T> for Cell<T>
where
    T: Copy
        + [const] crate::SaturatingAdd<T, Output = T>
        + [const] crate::SaturatingSub<T, Output = T>
        + [const] crate::SaturatingMul<T, Output = T>,
{
    fn saturating_added(&self, rhs: T) -> T {
        self.get().saturating_add(rhs)
    }

    fn saturating_subtracted(&self, rhs: T) -> T {
        self.get().saturating_sub(rhs)
    }

    fn saturating_multiplied(&self, rhs: T) -> T {
        self.get().saturating_mul(rhs)
    }
}

/// Clamping operations that modify the cell in place.
pub const trait CellIntClampingOps<T> {
    /// Sets the cell's value to the minimum of its current value and `rhs`.
    fn min(&self, rhs: T);
    /// Sets the cell's value to the maximum of its current value and `rhs`.
    fn max(&self, rhs: T);
    /// Clamps the cell's value between `min` and `max`.
    fn clamp(&self, min: T, max: T);
}

/// Clamping operations that return a new value without modifying the cell.
pub const trait CellIntClampingReturnOps<T> {
    /// Returns the minimum of the cell's value and `rhs`.
    fn minned(&self, rhs: T) -> T;
    /// Returns the maximum of the cell's value and `rhs`.
    fn maxed(&self, rhs: T) -> T;
    /// Returns the cell's value clamped between `min` and `max`.
    fn clamped(&self, min: T, max: T) -> T;
}

/// Sign-related operations that modify the cell in place.
pub const trait CellIntSignedOps<T> {
    /// Sets the cell's value to its absolute value.
    fn abs(&self);
    /// Negates the cell's value.
    fn neg(&self);
    /// Sets the cell's value to its sign (-1, 0, or 1).
    fn sign(&self);
}

/// Sign-related operations that return a new value without modifying the cell.
pub const trait CellIntSignedReturnOps<T> {
    /// Returns the absolute value of the cell's value.
    fn absed(&self) -> T;
    /// Returns the negation of the cell's value.
    fn neged(&self) -> T;
    /// Returns the sign of the cell's value (-1, 0, or 1).
    fn sign(&self) -> T;
}

/// Saturating sign-related operations that modify the cell in place.
pub const trait CellIntSaturatedSignedOps<T> {
    /// Sets the cell's value to its absolute value, saturating at numeric bounds.
    fn saturating_abs(&self);
    /// Negates the cell's value, saturating at numeric bounds.
    fn saturating_neg(&self);
}

/// Saturating sign-related operations that return a new value without modifying the cell.
pub const trait CellIntSaturatedSignedReturnOps<T> {
    /// Returns the absolute value of the cell's value, saturating at numeric bounds.
    fn saturating_absed(&self) -> T;
    /// Returns the negation of the cell's value, saturating at numeric bounds.
    fn saturating_neged(&self) -> T;
}

impl<T> CellIntClampingOps<T> for Cell<T>
where
    T: Copy + Ord,
{
    fn min(&self, rhs: T) {
        self.replace(self.get().min(rhs));
    }

    fn max(&self, rhs: T) {
        self.replace(self.get().max(rhs));
    }

    fn clamp(&self, min: T, max: T) {
        self.replace(self.get().clamp(min, max));
    }
}

impl<T> CellIntClampingReturnOps<T> for Cell<T>
where
    T: Copy + Ord,
{
    fn minned(&self, rhs: T) -> T {
        self.get().min(rhs)
    }

    fn maxed(&self, rhs: T) -> T {
        self.get().max(rhs)
    }

    fn clamped(&self, min: T, max: T) -> T {
        self.get().clamp(min, max)
    }
}

impl<T> const CellIntSignedOps<T> for Cell<T>
where
    T: Copy
        + [const] crate::Abs
        + [const] core::ops::Neg<Output = T>
        + [const] crate::Sign,
{
    fn abs(&self) {
        self.replace(self.get().abs());
    }

    fn neg(&self) {
        self.replace(-self.get());
    }

    fn sign(&self) {
        self.replace(self.get().sign());
    }
}

impl<T> const CellIntSignedReturnOps<T> for Cell<T>
where
    T: Copy
        + [const] crate::Abs
        + [const] core::ops::Neg<Output = T>
        + [const] crate::Sign,
{
    fn absed(&self) -> T {
        self.get().abs()
    }

    fn neged(&self) -> T {
        -self.get()
    }

    fn sign(&self) -> T {
        self.get().sign()
    }
}

impl<T> const CellIntSaturatedSignedOps<T> for Cell<T>
where
    T: Copy
        + [const] crate::SaturatingAbs<Output = T>
        + [const] crate::SaturatingNeg<Output = T>,
{
    fn saturating_abs(&self) {
        self.replace(self.get().saturating_abs());
    }

    fn saturating_neg(&self) {
        self.replace(self.get().saturating_neg());
    }
}

impl<T> const CellIntSaturatedSignedReturnOps<T> for Cell<T>
where
    T: Copy
        + [const] crate::SaturatingAbs<Output = T>
        + [const] crate::SaturatingNeg<Output = T>,
{
    fn saturating_absed(&self) -> T {
        self.get().saturating_abs()
    }

    fn saturating_neged(&self) -> T {
        self.get().saturating_neg()
    }
}
