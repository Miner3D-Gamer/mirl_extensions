// use std::{collections::HashMap, hash::BuildHasher};

// impl<
//         K1: core::cmp::Eq + core::hash::Hash + TryFromPatch<K2>,
//         K2: Clone,
//         V1: TryFromPatch<V2>,
//         V2: Clone,
//         S: BuildHasher + Default,
//     > TryFromPatch<HashMap<K2, V2, S>> for HashMap<K1, V1, S>
// {
//     fn try_from_value(value: HashMap<K2, V2, S>) -> Option<Self> {
//         let mut new = Self::with_hasher(S::default());
//         for (key, item) in value {
//             new.insert(K1::try_from_value(key)?, V1::try_from_value(item)?);
//         }
//         Some(new)
//     }
// }
use std::collections::BTreeMap;

use crate::{TryFromPatch, TryIntoPatch};

// TODO: Implement all functions from FromPatch here as well

impl<K1: Ord, K2: Clone + TryIntoPatch<K1>, V1, V2: Clone + TryIntoPatch<V1>>
    TryFromPatch<BTreeMap<K2, V2>> for BTreeMap<K1, V1>
{
    fn try_from_value(value: BTreeMap<K2, V2>) -> Option<Self> {
        let mut new = Self::new();
        for (key, item) in value {
            new.insert((key).try_into_value()?, (item).try_into_value()?);
        }
        Some(new)
    }
}
