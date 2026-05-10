use crate::TryFromPatch;

// Vec<T> conversions

impl<T, const N: usize> TryFromPatch<Vec<T>> for [T; N] {
    fn try_from_value(v: Vec<T>) -> Option<Self> {
        v.try_into().ok()
    }
}

// #[cfg(feature = "indexmap")]
// impl<K1, V1, K2: TryFromPatch<K1>, V2: TryFromPatch<V1>>
//     TryFromPatch<indexmap::IndexMap<K1, V1>> for Vec<(K2, V2)>
// {
//     fn try_from_value(v: indexmap::IndexMap<K1, V1>) -> Option<Self> {
//         let mut new = Self::new();
//         for (k, v) in v {
//             new.push((K2::try_from_value(k)?, V2::try_from_value(v)?));
//         }
//         Some(new)
//     }
// }

// #[cfg(feature = "indexmap")]
// impl<
//         K1,
//         V1,
//         K2: core::hash::Hash + Eq + TryFromPatch<K1>,
//         V2: TryFromPatch<V1>,
//     > TryFromPatch<Vec<(K1, V1)>> for indexmap::IndexMap<K2, V2>
// {
//     fn try_from_value(v: Vec<(K1, V1)>) -> Option<Self> {
//         let mut new = Self::new();
//         for (k, v) in v {
//             new.insert(K2::try_from_value(k)?, V2::try_from_value(v)?);
//         }
//         Some(new)
//     }
// }
