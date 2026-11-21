// Needs these at the crate root:
// #![feature(const_trait_impl, const_destruct)]

use std::marker::Destruct;

/// Allow for `.map()` on homogeneous tuples, applying the function to each
/// element individually
pub const trait TupleMap {
    /// The element type of the tuple
    type Item;
    /// The same tuple shape, with every element replaced by a `U`
    type Output<U>;

    /// Apply `f` to every element, in order, and collect the results
    fn map<U, F>(self, f: F) -> Self::Output<U>
    where
        F: [const] Fn(Self::Item) -> U + [const] Destruct;
}

macro_rules! tuple_map {
    (@type $_i:tt $t:ty) => {
        $t
    };

    ($($n:tt),+) => {
        const impl<T: [const] core::marker::Destruct> TupleMap for ($(tuple_map!(@type $n T),)+) {
            type Item = T;
            type Output<U> = ($(tuple_map!(@type $n U),)+);

            fn map<U, F>(self, f: F) -> Self::Output<U>
            where
                F: [const] Fn(T) -> U + [const] Destruct,
            {
                ($(f(self.$n),)+)
            }
        }
    };
}

tuple_map!(0);
tuple_map!(0, 1);
tuple_map!(0, 1, 2);
tuple_map!(0, 1, 2, 3);
tuple_map!(0, 1, 2, 3, 4);
tuple_map!(0, 1, 2, 3, 4, 5);
tuple_map!(0, 1, 2, 3, 4, 5, 6);
tuple_map!(0, 1, 2, 3, 4, 5, 6, 7);
tuple_map!(0, 1, 2, 3, 4, 5, 6, 7, 8);
tuple_map!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
tuple_map!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
tuple_map!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
// tuple_map!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
// tuple_map!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
// tuple_map!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14);
// tuple_map!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
// tuple_map!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
// tuple_map!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17);
// tuple_map!(
//     0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18
// );
// tuple_map!(
//     0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19
// );
// tuple_map!(
//     0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20
// );
// tuple_map!(
//     0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21
// );
// tuple_map!(
//     0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22
// );
// tuple_map!(
//     0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23
// );
// tuple_map!(
//     0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24
// );
// tuple_map!(
//     0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25
// );
// tuple_map!(
//     0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
//     26
// );
// tuple_map!(
//     0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
//     26, 27
// );
// tuple_map!(
//     0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
//     26, 27, 28
// );
// tuple_map!(
//     0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
//     26, 27, 28, 29
// );
// tuple_map!(
//     0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
//     26, 27, 28, 29, 30
// );
// tuple_map!(
//     0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
//     26, 27, 28, 29, 30, 31
// );
