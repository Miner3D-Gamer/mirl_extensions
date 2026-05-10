use core::cmp::Ordering;

/// Allows for < > >= <= ==
pub const trait TupleCmp<Rhs = Self> {
    /// <
    fn lt(self, rhs: Rhs) -> bool;
    /// <=
    fn le(self, rhs: Rhs) -> bool;
    /// >
    fn gt(self, rhs: Rhs) -> bool;
    /// >=
    fn ge(self, rhs: Rhs) -> bool;
    /// ==
    fn cmp(self, rhs: Rhs) -> Ordering;
}
macro_rules! impl_helper {
    (@type $_:tt $t:ty) => {
        $t
    };
}

macro_rules! tuple_cmp {
    ($($n:tt),+) => {
        impl<T> TupleCmp for ($(impl_helper!(@type $n T),)+)
        where
            T: PartialOrd + Copy
        {
            fn lt(self, rhs: Self) -> bool {
                tuple_cmp_helper!(@lt self, rhs; $($n),+)
            }

            fn le(self, rhs: Self) -> bool {
                tuple_cmp_helper!(@le self, rhs; $($n),+)
            }

            fn gt(self, rhs: Self) -> bool {
                tuple_cmp_helper!(@gt self, rhs; $($n),+)
            }

            fn ge(self, rhs: Self) -> bool {
                tuple_cmp_helper!(@ge self, rhs; $($n),+)
            }

            fn cmp(self, rhs: Self) -> Ordering {
                tuple_cmp_helper!(@cmp self, rhs; $($n),+)
            }
        }
    };
}

macro_rules! tuple_cmp_helper {
    (@lt $self:expr, $rhs:expr; $first:tt $(, $rest:tt)*) => {
        match $self.$first.partial_cmp(&$rhs.$first) {
            Some(Ordering::Less) => true,
            Some(Ordering::Equal) => tuple_cmp_helper!(@lt $self, $rhs; $($rest),*),
            _ => false,
        }
    };
    (@lt $self:expr, $rhs:expr;) => { false };

    (@le $self:expr, $rhs:expr; $first:tt $(, $rest:tt)*) => {
        match $self.$first.partial_cmp(&$rhs.$first) {
            Some(Ordering::Less) => true,
            Some(Ordering::Equal) => tuple_cmp_helper!(@le $self, $rhs; $($rest),*),
            Some(Ordering::Greater) => false,
            None => false,
        }
    };
    (@le $self:expr, $rhs:expr;) => { true };

    (@gt $self:expr, $rhs:expr; $first:tt $(, $rest:tt)*) => {
        match $self.$first.partial_cmp(&$rhs.$first) {
            Some(Ordering::Greater) => true,
            Some(Ordering::Equal) => tuple_cmp_helper!(@gt $self, $rhs; $($rest),*),
            _ => false,
        }
    };
    (@gt $self:expr, $rhs:expr;) => { false };

    (@ge $self:expr, $rhs:expr; $first:tt $(, $rest:tt)*) => {
        match $self.$first.partial_cmp(&$rhs.$first) {
            Some(Ordering::Greater) => true,
            Some(Ordering::Equal) => tuple_cmp_helper!(@ge $self, $rhs; $($rest),*),
            Some(Ordering::Less) => false,
            None => false,
        }
    };
    (@ge $self:expr, $rhs:expr;) => { true };

    (@cmp $self:expr, $rhs:expr; $first:tt $(, $rest:tt)*) => {
        match $self.$first.partial_cmp(&$rhs.$first) {
            Some(Ordering::Equal) => tuple_cmp_helper!(@cmp $self, $rhs; $($rest),*),
            Some(ord) => ord,
            None => Ordering::Equal, // Handle NaN case
        }
    };
    (@cmp $self:expr, $rhs:expr;) => { Ordering::Equal };
}

tuple_cmp!(0);
tuple_cmp!(0, 1);
tuple_cmp!(0, 1, 2);
tuple_cmp!(0, 1, 2, 3);
tuple_cmp!(0, 1, 2, 3, 4);
tuple_cmp!(0, 1, 2, 3, 4, 5);
tuple_cmp!(0, 1, 2, 3, 4, 5, 6);
tuple_cmp!(0, 1, 2, 3, 4, 5, 6, 7);
tuple_cmp!(0, 1, 2, 3, 4, 5, 6, 7, 8);
tuple_cmp!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
tuple_cmp!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
tuple_cmp!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
tuple_cmp!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
tuple_cmp!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
tuple_cmp!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14);
tuple_cmp!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
tuple_cmp!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
tuple_cmp!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17);
tuple_cmp!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18);
tuple_cmp!(
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19
);
tuple_cmp!(
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20
);
tuple_cmp!(
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    21
);
tuple_cmp!(
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    21, 22
);
tuple_cmp!(
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    21, 22, 23
);
tuple_cmp!(
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    21, 22, 23, 24
);
tuple_cmp!(
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    21, 22, 23, 24, 25
);
tuple_cmp!(
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    21, 22, 23, 24, 25, 26
);
tuple_cmp!(
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    21, 22, 23, 24, 25, 26, 27
);
tuple_cmp!(
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    21, 22, 23, 24, 25, 26, 27, 28
);
tuple_cmp!(
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    21, 22, 23, 24, 25, 26, 27, 28, 29
);
tuple_cmp!(
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    21, 22, 23, 24, 25, 26, 27, 28, 29, 30
);
tuple_cmp!(
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31
);
