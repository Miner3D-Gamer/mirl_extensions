// use core::cell::RefCell;
// use crate::*;

// impl<T: crate::ConstZero> SetCellToZero for core::cell::RefCell<T> {
//     fn set_zero(&self) {
//         self.replace(T::ZERO);
//     }
// }
// impl<T> const CellIntOps<T> for RefCell<T>
// where
//     T: Copy
//         + [const] core::ops::Add<Output = T>
//         + [const] core::ops::Sub<Output = T>
//         + [const] core::ops::Mul<Output = T>
//         + [const] core::ops::Div<Output = T>,
// {
//     fn add(&self, rhs: T) {
//         self.replace(self.into_inner() + rhs);
//     }

//     fn sub(&self, rhs: T) {
//         self.replace(self.into_inner() - rhs);
//     }

//     fn mul(&self, rhs: T) {
//         self.replace(self.into_inner() * rhs);
//     }

//     fn div(&self, rhs: T) {
//         self.replace(self.into_inner() / rhs);
//     }
// }

// impl<T> CellIntReturnOps<T> for RefCell<T>
// where
//     T: Copy
//         + core::ops::Add<Output = T>
//         + core::ops::Sub<Output = T>
//         + core::ops::Mul<Output = T>
//         + core::ops::Div<Output = T>,
// {
//     fn added(&self, rhs: T) -> T {
//         let result = self.into_inner() + rhs;
//         self.replace(result);
//         result
//     }

//     fn subtracted(&self, rhs: T) -> T {
//         let result = self.into_inner() - rhs;
//         self.replace(result);
//         result
//     }

//     fn multiplied(&self, rhs: T) -> T {
//         let result = self.into_inner() * rhs;
//         self.replace(result);
//         result
//     }

//     fn divided(&self, rhs: T) -> T {
//         let result = self.into_inner() / rhs;
//         self.replace(result);
//         result
//     }
// }

// impl<T> const CellIntSaturatedOps<T> for RefCell<T>
// where
//     T: Copy
//         + [const] crate::SaturatingAdd<T, Output = T>
//         + [const] crate::SaturatingSub<T, Output = T>
//         + [const] crate::SaturatingMul<T, Output = T>,
// {
//     fn saturating_add(&self, rhs: T) {
//         self.replace(self.into_inner().saturating_add(rhs));
//     }

//     fn saturating_sub(&self, rhs: T) {
//         self.replace(self.into_inner().saturating_sub(rhs));
//     }

//     fn saturating_mul(&self, rhs: T) {
//         self.replace(self.into_inner().saturating_mul(rhs));
//     }
// }

// impl<T> const CellIntSaturatedReturnOps<T> for RefCell<T>
// where
//     T: Copy
//         + [const] crate::SaturatingAdd<T, Output = T>
//         + [const] crate::SaturatingSub<T, Output = T>
//         + [const] crate::SaturatingMul<T, Output = T>,
// {
//     fn saturating_added(&self, rhs: T) -> T {
//         self.into_inner().saturating_add(rhs)
//     }

//     fn saturating_subtracted(&self, rhs: T) -> T {
//         self.into_inner().saturating_sub(rhs)
//     }

//     fn saturating_multiplied(&self, rhs: T) -> T {
//         self.into_inner().saturating_mul(rhs)
//     }
// }

// impl<T> CellIntClampingOps<T> for RefCell<T>
// where
//     T: Copy + Ord,
// {
//     fn min(&self, rhs: T) {
//         self.replace(self.into_inner().min(rhs));
//     }

//     fn max(&self, rhs: T) {
//         self.replace(self.into_inner().max(rhs));
//     }

//     fn clamp(&self, min: T, max: T) {
//         self.replace(self.into_inner().clamp(min, max));
//     }
// }

// impl<T> CellIntClampingReturnOps<T> for RefCell<T>
// where
//     T: Copy + Ord,
// {
//     fn minned(&self, rhs: T) -> T {
//         self.into_inner().min(rhs)
//     }

//     fn maxed(&self, rhs: T) -> T {
//         self.into_inner().max(rhs)
//     }

//     fn clamped(&self, min: T, max: T) -> T {
//         self.into_inner().clamp(min, max)
//     }
// }

// impl<T> const CellIntSignedOps<T> for RefCell<T>
// where
//     T: Copy
//         + [const] crate::Abs
//         + [const] core::ops::Neg<Output = T>
//         + [const] crate::Sign,
// {
//     fn abs(&self) {
//         self.replace(self.into_inner().abs());
//     }

//     fn neg(&self) {
//         self.replace(-self.into_inner());
//     }

//     fn sign(&self) {
//         self.replace(self.into_inner().sign());
//     }
// }

// impl<T> const CellIntSignedReturnOps<T> for RefCell<T>
// where
//     T: Copy
//         + [const] crate::Abs
//         + [const] core::ops::Neg<Output = T>
//         + [const] crate::Sign,
// {
//     fn absed(&self) -> T {
//         self.into_inner().abs()
//     }

//     fn neged(&self) -> T {
//         -self.into_inner()
//     }

//     fn sign(&self) -> T {
//         self.into_inner().sign()
//     }
// }

// impl<T> const CellIntSaturatedSignedOps<T> for RefCell<T>
// where
//     T: Copy
//         + [const] crate::SaturatingAbs<Output = T>
//         + [const] crate::SaturatingNeg<Output = T>,
// {
//     fn saturating_abs(&self) {
//         self.replace(self.into_inner().saturating_abs());
//     }

//     fn saturating_neg(&self) {
//         self.replace(self.into_inner().saturating_neg());
//     }
// }

// impl<T> const CellIntSaturatedSignedReturnOps<T> for RefCell<T>
// where
//     T: Copy
//         + [const] crate::SaturatingAbs<Output = T>
//         + [const] crate::SaturatingNeg<Output = T>,
// {
//     fn saturating_absed(&self) -> T {
//         self.into_inner().saturating_abs()
//     }

//     fn saturating_neged(&self) -> T {
//         self.into_inner().saturating_neg()
//     }
// }
