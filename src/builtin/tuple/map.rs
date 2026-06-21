//! Don't even question why I though this would be a good idea.

use std::marker::Destruct;

/// Allow for `.map()` on tuples
pub const trait TupleMap<Function, Output>
where
    Self: std::marker::Sized,
    Function: [const] FnOnce(Self) -> Output + [const] Destruct,
{
    /// Map the inner value to another
    fn map(self, f: Function) -> Output;
}
const impl<Type, Function, Output> TupleMap<Function, Output> for (Type,)
where
    Self: std::marker::Sized,
    Function: [const] FnOnce(Self) -> Output + [const] Destruct,
{
    fn map(self, f: Function) -> Output {
        f(self)
    }
}

const impl<Type1, Type2, Function, Output> TupleMap<Function, Output> for (Type1, Type2)
where
    Self: std::marker::Sized,
    Function: [const] FnOnce(Self) -> Output + [const] Destruct,
{
    fn map(self, f: Function) -> Output {
        f(self)
    }
}
