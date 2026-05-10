use crate::{FromPatch, IntoPatch};

impl<
    K1: IntoPatch<K2>,
    V1: IntoPatch<V2>,
    K2,
    V2,
    S: ::std::hash::BuildHasher + Default,
> FromPatch<std::collections::HashMap<K1, V1, S>> for Vec<(K2, V2)>
{
    fn from_value(v: std::collections::HashMap<K1, V1, S>) -> Self {
        let mut new = Self::new();
        for (k, v) in v {
            new.push((k.into_value(), v.into_value()));
        }
        new
    }
}

impl<
    K1: IntoPatch<K2>,
    V1: IntoPatch<V2>,
    K2: core::hash::Hash + Eq,
    V2,
    S: ::std::hash::BuildHasher + Default,
> FromPatch<Vec<(K1, V1)>> for std::collections::HashMap<K2, V2, S>
{
    fn from_value(v: Vec<(K1, V1)>) -> Self {
        let mut new = Self::with_capacity_and_hasher(v.len(), S::default());
        for (k, v) in v {
            new.insert(k.into_value(), v.into_value());
        }
        new
    }
}

impl<
    K1: IntoPatch<K2>,
    V1: IntoPatch<V2>,
    K2: core::hash::Hash + Eq,
    V2,
    S1: ::std::hash::BuildHasher + Default,
    S2: ::std::hash::BuildHasher + Default,
> FromPatch<std::collections::HashMap<K1, V1, S1>>
    for std::collections::HashMap<K2, V2, S2>
{
    fn from_value(v: std::collections::HashMap<K1, V1, S1>) -> Self {
        let mut new = Self::with_capacity_and_hasher(v.len(), S2::default());
        for (k, v) in v {
            new.insert(k.into_value(), v.into_value());
        }
        new
    }
}

#[cfg(feature = "indexmap")]
impl<
    K1: IntoPatch<K2>,
    V1: IntoPatch<V2>,
    K2,
    V2,
    S: ::std::hash::BuildHasher + Default,
> FromPatch<indexmap::IndexMap<K1, V1, S>> for Vec<(K2, V2)>
{
    fn from_value(v: indexmap::IndexMap<K1, V1, S>) -> Self {
        let mut new = Self::new();
        for (k, v) in v {
            new.push((k.into_value(), v.into_value()));
        }
        new
    }
}

#[cfg(feature = "indexmap")]
impl<
    K1: IntoPatch<K2>,
    V1: IntoPatch<V2>,
    K2: core::hash::Hash + Eq,
    V2,
    S: ::std::hash::BuildHasher + Default,
> FromPatch<Vec<(K1, V1)>> for indexmap::IndexMap<K2, V2, S>
{
    fn from_value(v: Vec<(K1, V1)>) -> Self {
        let mut new = Self::with_capacity_and_hasher(v.len(), S::default());
        for (k, v) in v {
            new.insert(k.into_value(), v.into_value());
        }
        new
    }
}

#[cfg(feature = "dashmap")]
impl<
    K1: std::cmp::Eq + std::hash::Hash + Clone + IntoPatch<K2>,
    V1: Clone + IntoPatch<V2>,
    K2: std::hash::Hash + Eq,
    V2,
> FromPatch<dashmap::DashMap<K1, V1>> for dashmap::DashMap<K2, V2>
{
    fn from_value(v: dashmap::DashMap<K1, V1>) -> Self {
        let new = Self::with_capacity(v.len());
        for r in &v {
            new.insert(
                r.key().clone().into_value(),
                r.value().clone().into_value(),
            );
        }
        new
    }
}

#[cfg(feature = "dashmap")]
impl<K1: IntoPatch<K2>, V1: IntoPatch<V2>, K2: std::hash::Hash + Eq, V2>
    FromPatch<Vec<(K1, V1)>> for dashmap::DashMap<K2, V2>
{
    fn from_value(v: Vec<(K1, V1)>) -> Self {
        let new = Self::with_capacity(v.len());
        for (k, v) in v {
            new.insert(k.into_value(), v.into_value());
        }
        new
    }
}

#[cfg(feature = "indexmap")]
impl<
    K1: IntoPatch<K2>,
    V1: IntoPatch<V2>,
    K2: core::hash::Hash + Eq,
    V2,
    S1: ::std::hash::BuildHasher + Default,
    S2: ::std::hash::BuildHasher + Default,
> FromPatch<std::collections::HashMap<K1, V1, S1>>
    for indexmap::IndexMap<K2, V2, S2>
{
    fn from_value(v: std::collections::HashMap<K1, V1, S1>) -> Self {
        let mut new = Self::with_capacity_and_hasher(v.len(), S2::default());
        for (k, v) in v {
            new.insert(k.into_value(), v.into_value());
        }
        new
    }
}

#[cfg(feature = "indexmap")]
impl<
    K1: IntoPatch<K2>,
    V1: IntoPatch<V2>,
    K2: core::hash::Hash + Eq,
    V2,
    S1: ::std::hash::BuildHasher + Default,
    S2: ::std::hash::BuildHasher + Default,
> FromPatch<indexmap::IndexMap<K1, V1, S1>>
    for std::collections::HashMap<K2, V2, S2>
{
    fn from_value(v: indexmap::IndexMap<K1, V1, S1>) -> Self {
        let mut new = Self::with_capacity_and_hasher(v.len(), S2::default());
        for (k, v) in v {
            new.insert(k.into_value(), v.into_value());
        }
        new
    }
}

#[cfg(feature = "indexmap")]
impl<
    K1: IntoPatch<K2>,
    V1: IntoPatch<V2>,
    K2: core::hash::Hash + Eq,
    V2,
    S1: ::std::hash::BuildHasher + Default,
    S2: ::std::hash::BuildHasher + Default,
> FromPatch<indexmap::IndexMap<K1, V1, S1>> for indexmap::IndexMap<K2, V2, S2>
{
    fn from_value(v: indexmap::IndexMap<K1, V1, S1>) -> Self {
        let mut new = Self::with_capacity_and_hasher(v.len(), S2::default());
        for (k, v) in v {
            new.insert(k.into_value(), v.into_value());
        }
        new
    }
}

#[cfg(feature = "dashmap")]
impl<
    K1: core::hash::Hash + Eq + IntoPatch<K2>,
    V1: IntoPatch<V2>,
    K2: core::hash::Hash + Eq,
    V2,
    S: ::std::hash::BuildHasher + Default,
> FromPatch<std::collections::HashMap<K1, V1, S>> for dashmap::DashMap<K2, V2>
{
    fn from_value(v: std::collections::HashMap<K1, V1, S>) -> Self {
        let new = Self::with_capacity(v.len());
        for (k, v) in v {
            new.insert(k.into_value(), v.into_value());
        }
        new
    }
}

#[cfg(feature = "dashmap")]
impl<
    K1: std::cmp::Eq + std::hash::Hash + Clone + IntoPatch<K2>,
    V1: Clone + IntoPatch<V2>,
    K2: core::hash::Hash + Eq,
    V2,
    S: ::std::hash::BuildHasher + Default,
> FromPatch<dashmap::DashMap<K1, V1>> for std::collections::HashMap<K2, V2, S>
{
    fn from_value(v: dashmap::DashMap<K1, V1>) -> Self {
        let mut new = Self::with_capacity_and_hasher(v.len(), S::default());
        for r in &v {
            new.insert(
                r.key().clone().into_value(),
                r.value().clone().into_value(),
            );
        }
        new
    }
}

#[cfg(all(feature = "indexmap", feature = "dashmap"))]
impl<
    K1: std::cmp::Eq + std::hash::Hash + Clone + IntoPatch<K2>,
    V1: Clone + IntoPatch<V2>,
    K2: core::hash::Hash + Eq,
    V2,
    S: ::std::hash::BuildHasher + Default,
> FromPatch<dashmap::DashMap<K1, V1>> for indexmap::IndexMap<K2, V2, S>
{
    fn from_value(v: dashmap::DashMap<K1, V1>) -> Self {
        let mut new = Self::with_capacity_and_hasher(v.len(), S::default());
        for r in &v {
            new.insert(
                r.key().clone().into_value(),
                r.value().clone().into_value(),
            );
        }
        new
    }
}

#[cfg(all(feature = "indexmap", feature = "dashmap"))]
impl<
    K1: core::hash::Hash + Eq + IntoPatch<K2>,
    V1: IntoPatch<V2>,
    K2: core::hash::Hash + Eq,
    V2,
    S: ::std::hash::BuildHasher + Default,
> FromPatch<indexmap::IndexMap<K1, V1, S>> for dashmap::DashMap<K2, V2>
{
    fn from_value(v: indexmap::IndexMap<K1, V1, S>) -> Self {
        let new = Self::with_capacity(v.len());
        for (k, v) in v {
            new.insert(k.into_value(), v.into_value());
        }
        new
    }
}

#[cfg(feature = "dashmap")]
impl<
    K1: std::cmp::Eq + std::hash::Hash + Clone + IntoPatch<K2>,
    V1: Clone + IntoPatch<V2>,
    K2,
    V2,
> FromPatch<dashmap::DashMap<K1, V1>> for Vec<(K2, V2)>
{
    fn from_value(v: dashmap::DashMap<K1, V1>) -> Self {
        let mut new = Self::with_capacity(v.len());
        for r in &v {
            new.push((
                r.key().clone().into_value(),
                r.value().clone().into_value(),
            ));
        }
        new
    }
}

#[cfg(all(feature = "indexmap", feature = "masstree"))]
impl<
    'a,
    K: core::hash::Hash + Eq + super::IntoPatch<&'a [u8]>,
    V1: IntoPatch<V2>,
    V2: std::marker::Sync + std::marker::Send + 'static + Clone,
    S: ::std::hash::BuildHasher + Default,
> FromPatch<indexmap::IndexMap<K, V1, S>> for masstree::MassTree15<V2>
{
    fn from_value(v: indexmap::IndexMap<K, V1, S>) -> Self {
        let new = Self::new();
        for (k, v) in v {
            new.insert(k.into_value(), v.into_value());
        }
        new
    }
}

#[cfg(all(feature = "indexmap", feature = "masstree"))]
impl<
    'a,
    K: core::hash::Hash + Eq + super::IntoPatch<&'a [u8]>,
    V1: IntoPatch<V2>,
    V2: std::marker::Sync + std::marker::Send + 'static + masstree::InlineBits,
    S: ::std::hash::BuildHasher + Default,
> FromPatch<indexmap::IndexMap<K, V1, S>> for masstree::MassTree15Inline<V2>
{
    fn from_value(v: indexmap::IndexMap<K, V1, S>) -> Self {
        let new = Self::new();
        for (k, v) in v {
            new.insert(k.into_value(), v.into_value());
        }
        new
    }
}

#[cfg(feature = "masstree")]
impl<
    'a,
    K: core::hash::Hash + Eq + super::IntoPatch<&'a [u8]>,
    V1: IntoPatch<V2>,
    V2: std::marker::Sync + std::marker::Send + 'static + Clone,
    S: ::std::hash::BuildHasher + Default,
> FromPatch<std::collections::HashMap<K, V1, S>> for masstree::MassTree15<V2>
{
    fn from_value(v: std::collections::HashMap<K, V1, S>) -> Self {
        let new = Self::new();
        for (k, v) in v {
            new.insert(k.into_value(), v.into_value());
        }
        new
    }
}

#[cfg(feature = "masstree")]
impl<
    'a,
    K: core::hash::Hash + Eq + super::IntoPatch<&'a [u8]>,
    V1: IntoPatch<V2>,
    V2: std::marker::Sync + std::marker::Send + 'static + masstree::InlineBits,
    S: ::std::hash::BuildHasher + Default,
> FromPatch<std::collections::HashMap<K, V1, S>>
    for masstree::MassTree15Inline<V2>
{
    fn from_value(v: std::collections::HashMap<K, V1, S>) -> Self {
        let new = Self::new();
        for (k, v) in v {
            new.insert(k.into_value(), v.into_value());
        }
        new
    }
}

#[cfg(feature = "masstree")]
impl<
    'a,
    K: super::IntoPatch<&'a [u8]>,
    V1: IntoPatch<V2>,
    V2: std::marker::Sync + std::marker::Send + 'static + Clone,
> FromPatch<Vec<(K, V1)>> for masstree::MassTree15<V2>
{
    fn from_value(v: Vec<(K, V1)>) -> Self {
        let new = Self::new();
        for (k, v) in v {
            new.insert(k.into_value(), v.into_value());
        }
        new
    }
}

#[cfg(feature = "masstree")]
impl<
    'a,
    K: super::IntoPatch<&'a [u8]>,
    V1: IntoPatch<V2>,
    V2: std::marker::Sync + std::marker::Send + 'static + masstree::InlineBits,
> FromPatch<Vec<(K, V1)>> for masstree::MassTree15Inline<V2>
{
    fn from_value(v: Vec<(K, V1)>) -> Self {
        let new = Self::new();
        for (k, v) in v {
            new.insert(k.into_value(), v.into_value());
        }
        new
    }
}

#[cfg(all(feature = "dashmap", feature = "masstree"))]
impl<
    'a,
    K: core::hash::Hash + Eq + Clone + super::IntoPatch<&'a [u8]>,
    V1: Clone + IntoPatch<V2>,
    V2: std::marker::Sync + std::marker::Send + 'static + Clone,
> FromPatch<dashmap::DashMap<K, V1>> for masstree::MassTree15<V2>
{
    fn from_value(v: dashmap::DashMap<K, V1>) -> Self {
        let new = Self::new();
        for r in &v {
            new.insert(
                r.key().clone().into_value(),
                r.value().clone().into_value(),
            );
        }
        new
    }
}

#[cfg(all(feature = "dashmap", feature = "masstree"))]
impl<
    'a,
    K: core::hash::Hash + Eq + Clone + super::IntoPatch<&'a [u8]>,
    V1: Clone + IntoPatch<V2>,
    V2: std::marker::Sync + std::marker::Send + 'static + masstree::InlineBits,
> FromPatch<dashmap::DashMap<K, V1>> for masstree::MassTree15Inline<V2>
{
    fn from_value(v: dashmap::DashMap<K, V1>) -> Self {
        let new = Self::new();
        for r in &v {
            new.insert(
                r.key().clone().into_value(),
                r.value().clone().into_value(),
            );
        }
        new
    }
}
