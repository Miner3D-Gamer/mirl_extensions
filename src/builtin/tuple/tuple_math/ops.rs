#![allow(missing_docs)]
use core::ops::{Add, Div, Mul, Sub};

pub const trait TupleOps<Rhs = Self> {
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
    fn sub(self, rhs: Rhs) -> Self::Output;
    fn mul(self, rhs: Rhs) -> Self::Output;
    fn div(self, rhs: Rhs) -> Self::Output;
}

macro_rules! impl_helper {
    (@type $_:tt $t:ty) => {
        $t
    };
}

// Tuple OP Tuple
macro_rules! tuple_ops {
    ( $($n:tt),+ ) => {
        impl<T> const TupleOps for ( $( impl_helper!(@type $n T), )+ )
        where
            T: [const] Add<Output = T>
                + [const] Sub<Output = T>
                + [const] Mul<Output = T>
                + [const] Div<Output = T>
                + Copy
        {
            type Output = ( $( impl_helper!(@type $n T), )+ );

            fn add(self, rhs: Self) -> Self::Output {
                ( $( self.$n + rhs.$n, )+ )
            }
            fn sub(self, rhs: Self) -> Self::Output {
                ( $( self.$n - rhs.$n, )+ )
            }
            fn mul(self, rhs: Self) -> Self::Output {
                ( $( self.$n * rhs.$n, )+ )
            }
            fn div(self, rhs: Self) -> Self::Output {
                ( $( self.$n / rhs.$n, )+ )
            }
        }
    };
}

// Tuple OP Scalar
macro_rules! tuple_scalar_ops {
    ( $($n:tt),+ ) => {
        impl<T> const TupleOps<T> for ( $( impl_helper!(@type $n T), )+ )
        where
            T: [const] Add<Output = T>
                + [const] Sub<Output = T>
                + [const] Mul<Output = T>
                + [const] Div<Output = T>
                + Copy
        {
            type Output = ( $( impl_helper!(@type $n T), )+ );

            fn add(self, rhs: T) -> Self::Output {
                ( $( self.$n + rhs, )+ )
            }
            fn sub(self, rhs: T) -> Self::Output {
                ( $( self.$n - rhs, )+ )
            }
            fn mul(self, rhs: T) -> Self::Output {
                ( $( self.$n * rhs, )+ )
            }
            fn div(self, rhs: T) -> Self::Output {
                ( $( self.$n / rhs, )+ )
            }
        }
    };
}

tuple_ops! {0,1}
tuple_scalar_ops! {0,1}
tuple_ops! {0,1,2}
tuple_scalar_ops! {0,1,2}
tuple_ops! {0,1,2,3}
tuple_scalar_ops! {0,1,2,3}
tuple_ops! {0,1,2,3,4}
tuple_scalar_ops! {0,1,2,3,4}
tuple_ops! {0,1,2,3,4,5}
tuple_scalar_ops! {0,1,2,3,4,5}
tuple_ops! {0,1,2,3,4,5,6}
tuple_scalar_ops! {0,1,2,3,4,5,6}
tuple_ops! {0,1,2,3,4,5,6,7}
tuple_scalar_ops! {0,1,2,3,4,5,6,7}
tuple_ops! {0,1,2,3,4,5,6,7,8}
tuple_scalar_ops! {0,1,2,3,4,5,6,7,8}
tuple_ops! {0,1,2,3,4,5,6,7,8,9}
tuple_scalar_ops! {0,1,2,3,4,5,6,7,8,9}
tuple_ops! {0,1,2,3,4,5,6,7,8,9,10}
tuple_scalar_ops! {0,1,2,3,4,5,6,7,8,9,10}
tuple_ops! {0,1,2,3,4,5,6,7,8,9,10,11}
tuple_scalar_ops! {0,1,2,3,4,5,6,7,8,9,10,11}
