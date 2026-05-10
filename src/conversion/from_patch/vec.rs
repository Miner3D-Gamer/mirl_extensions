use crate::{FromPatch, IntoPatch};

// Vec<T> conversions

impl<T: IntoPatch<S>, S, const N: usize> FromPatch<[T; N]> for Vec<S> {
    fn from_value(v: [T; N]) -> Self {
        let mut o = Self::with_capacity(N);
        for i in v {
            o.push(i.into_value());
        }
        o
    }
}
impl<T: Clone + IntoPatch<S>, S> FromPatch<&[T]> for Vec<S> {
    fn from_value(v: &[T]) -> Self {
        v.iter().map(|x| x.clone().into_value()).collect()
    }
}

// Vec<T> with element conversion
impl<T: IntoPatch<S>, S> FromPatch<Vec<T>> for Vec<S> {
    fn from_value(v: Vec<T>) -> Self {
        let mut o = Self::with_capacity(v.len());
        for i in v {
            o.push(i.into_value());
        }
        o
    }
}

// Array to array with element conversion
impl<T: Clone + IntoPatch<S>, S, const N: usize> FromPatch<[T; N]> for [S; N] {
    fn from_value(v: [T; N]) -> Self {
        let vec: Vec<S> = v.into_iter().map(IntoPatch::into_value).collect();

        // Safety: We know that the in and output slices have the same length so there is no need to worry
        unsafe { vec.try_into().unwrap_unchecked() }
    }
}
// impl<I, O, const N: usize> FromPatch<[(I,); N]> for [O; N] {
//     fn from_value(value: [(I,); N]) -> Self {

//     }
// }

// impl<T: Clone, S: FromPatch<T>, const N: usize> FromPatch<&[T; N]> for [S; N] {
//     fn from_value(v: &[T; N]) -> Self {
//         let vec: Vec<S> = v.iter().map(|x| S::from_value(x.clone())).collect();

//         // Safety: We know that the in and output slices have the same length so there is no need to worry
//         vec.try_into().easy_unwrap_unchecked()
//     }
// }

// impl<T> FromPatch<T> for T {
//     fn from_value(value: T) -> Self {
//         value
//     }
// }
