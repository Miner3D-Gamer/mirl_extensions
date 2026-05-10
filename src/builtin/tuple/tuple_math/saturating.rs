#![allow(missing_docs)]
use core::ops::Div;

use crate::{SaturatingAdd, SaturatingMul, SaturatingSub};

pub const trait SaturatingTupleOps<Rhs = Self> {
    type Output;
    fn saturating_add(self, rhs: Rhs) -> Self::Output;
    fn saturating_sub(self, rhs: Rhs) -> Self::Output;
    fn saturating_mul(self, rhs: Rhs) -> Self::Output;
    fn saturating_div(self, rhs: Rhs) -> Self::Output;
}

macro_rules! tuple_ops {
    ($($n:tt),+) => {
        impl<T> SaturatingTupleOps for ($(impl_helper!(@type $n T),)+)
        where
            T: SaturatingAdd<Output = T> + SaturatingSub<Output = T> + SaturatingMul<Output = T> + Div<Output = T> + Copy
        {
            type Output = ($(impl_helper!(@type $n T),)+);

            fn saturating_add(self, rhs: Self) -> Self::Output {
                ($(self.$n.saturating_add(rhs.$n),)+)
            }

            fn saturating_sub(self, rhs: Self) -> Self::Output {
                ($(self.$n.saturating_sub(rhs.$n),)+)
            }

            fn saturating_mul(self, rhs: Self) -> Self::Output {
                ($(self.$n.saturating_mul(rhs.$n),)+)
            }

            fn saturating_div(self, rhs: Self) -> Self::Output {
                ($(self.$n / rhs.$n,)+)
            }
        }
    };
}

macro_rules! impl_helper {
    (@type $_:tt $t:ty) => {
        $t
    };
}

tuple_ops! {0,1}
tuple_ops! {0,1,2}
tuple_ops! {0,1,2,3}
tuple_ops! {0,1,2,3,4}
tuple_ops! {0,1,2,3,4,5}
tuple_ops! {0,1,2,3,4,5,6}
tuple_ops! {0,1,2,3,4,5,6,7}
tuple_ops! {0,1,2,3,4,5,6,7,8}
tuple_ops! {0,1,2,3,4,5,6,7,8,9}
tuple_ops! {0,1,2,3,4,5,6,7,8,9,10}
tuple_ops! {0,1,2,3,4,5,6,7,8,9,10,11}
