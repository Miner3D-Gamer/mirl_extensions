use crate::{FromPatch, IntoPatch};

impl<T1: IntoPatch<T2>, T2, S: ::std::hash::BuildHasher + Default>
    FromPatch<std::collections::HashSet<T1, S>> for Vec<T2>
{
    fn from_value(v: std::collections::HashSet<T1, S>) -> Self {
        v.into_iter().map(super::IntoPatch::into_value).collect()
    }
}

impl<
    T1: IntoPatch<T2>,
    T2: core::hash::Hash + Eq,
    S: ::std::hash::BuildHasher + Default,
> FromPatch<Vec<T1>> for std::collections::HashSet<T2, S>
{
    fn from_value(v: Vec<T1>) -> Self {
        let mut new = Self::with_capacity_and_hasher(v.len(), S::default());
        for item in v {
            new.insert(item.into_value());
        }
        new
    }
}

impl<
    T1: IntoPatch<T2>,
    T2: core::hash::Hash + Eq,
    S1: ::std::hash::BuildHasher + Default,
    S2: ::std::hash::BuildHasher + Default,
> FromPatch<std::collections::HashSet<T1, S1>>
    for std::collections::HashSet<T2, S2>
{
    fn from_value(v: std::collections::HashSet<T1, S1>) -> Self {
        let mut new = Self::with_capacity_and_hasher(v.len(), S2::default());
        for item in v {
            new.insert(item.into_value());
        }
        new
    }
}

impl<
    K1: IntoPatch<K2>,
    V1: IntoPatch<V2>,
    K2: core::hash::Hash + Eq,
    V2: core::hash::Hash + Eq,
    S1: ::std::hash::BuildHasher + Default,
    S2: ::std::hash::BuildHasher + Default,
> FromPatch<std::collections::HashMap<K1, V1, S1>>
    for std::collections::HashSet<(K2, V2), S2>
{
    fn from_value(v: std::collections::HashMap<K1, V1, S1>) -> Self {
        let mut new = Self::with_capacity_and_hasher(v.len(), S2::default());
        for (k, v) in v {
            new.insert((k.into_value(), v.into_value()));
        }
        new
    }
}

impl<
    K1: core::hash::Hash + Eq + IntoPatch<K2>,
    V1: core::hash::Hash + Eq + IntoPatch<V2>,
    K2: core::hash::Hash + Eq,
    V2,
    S1: ::std::hash::BuildHasher + Default,
    S2: ::std::hash::BuildHasher + Default,
> FromPatch<std::collections::HashSet<(K1, V1), S1>>
    for std::collections::HashMap<K2, V2, S2>
{
    fn from_value(v: std::collections::HashSet<(K1, V1), S1>) -> Self {
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
    V2: core::hash::Hash + Eq,
    S1: ::std::hash::BuildHasher + Default,
    S2: ::std::hash::BuildHasher + Default,
> FromPatch<indexmap::IndexMap<K1, V1, S1>>
    for std::collections::HashSet<(K2, V2), S2>
{
    fn from_value(v: indexmap::IndexMap<K1, V1, S1>) -> Self {
        let mut new = Self::with_capacity_and_hasher(v.len(), S2::default());
        for (k, v) in v {
            new.insert((k.into_value(), v.into_value()));
        }
        new
    }
}

#[cfg(feature = "indexmap")]
impl<
    K1: core::hash::Hash + Eq + IntoPatch<K2>,
    V1: core::hash::Hash + Eq + IntoPatch<V2>,
    K2: core::hash::Hash + Eq,
    V2,
    S1: ::std::hash::BuildHasher + Default,
    S2: ::std::hash::BuildHasher + Default,
> FromPatch<std::collections::HashSet<(K1, V1), S1>>
    for indexmap::IndexMap<K2, V2, S2>
{
    fn from_value(v: std::collections::HashSet<(K1, V1), S1>) -> Self {
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
    V2: core::hash::Hash + Eq,
    S: ::std::hash::BuildHasher + Default,
> FromPatch<Vec<(K1, V1)>> for indexmap::IndexSet<(K2, V2), S>
{
    fn from_value(v: Vec<(K1, V1)>) -> Self {
        let mut new = Self::with_capacity_and_hasher(v.len(), S::default());
        for (k, v) in v {
            new.insert((k.into_value(), v.into_value()));
        }
        new
    }
}

#[cfg(feature = "indexmap")]
impl<
    K1: core::hash::Hash + Eq + IntoPatch<K2>,
    V1: core::hash::Hash + Eq + IntoPatch<V2>,
    K2,
    V2,
    S: ::std::hash::BuildHasher + Default,
> FromPatch<indexmap::IndexSet<(K1, V1), S>> for Vec<(K2, V2)>
{
    fn from_value(v: indexmap::IndexSet<(K1, V1), S>) -> Self {
        v.into_iter().map(|(k, v)| (k.into_value(), v.into_value())).collect()
    }
}

#[cfg(feature = "indexmap")]
impl<
    K1: IntoPatch<K2>,
    V1: IntoPatch<V2>,
    K2: core::hash::Hash + Eq,
    V2: core::hash::Hash + Eq,
    S1: ::std::hash::BuildHasher + Default,
    S2: ::std::hash::BuildHasher + Default,
> FromPatch<std::collections::HashMap<K1, V1, S1>>
    for indexmap::IndexSet<(K2, V2), S2>
{
    fn from_value(v: std::collections::HashMap<K1, V1, S1>) -> Self {
        let mut new = Self::with_capacity_and_hasher(v.len(), S2::default());
        for (k, v) in v {
            new.insert((k.into_value(), v.into_value()));
        }
        new
    }
}

#[cfg(feature = "indexmap")]
impl<
    K1: core::hash::Hash + Eq + IntoPatch<K2>,
    V1: core::hash::Hash + Eq + IntoPatch<V2>,
    K2: core::hash::Hash + Eq,
    V2,
    S1: ::std::hash::BuildHasher + Default,
    S2: ::std::hash::BuildHasher + Default,
> FromPatch<indexmap::IndexSet<(K1, V1), S1>>
    for std::collections::HashMap<K2, V2, S2>
{
    fn from_value(v: indexmap::IndexSet<(K1, V1), S1>) -> Self {
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
    V2: core::hash::Hash + Eq,
    S1: ::std::hash::BuildHasher + Default,
    S2: ::std::hash::BuildHasher + Default,
> FromPatch<indexmap::IndexMap<K1, V1, S1>>
    for indexmap::IndexSet<(K2, V2), S2>
{
    fn from_value(v: indexmap::IndexMap<K1, V1, S1>) -> Self {
        let mut new = Self::with_capacity_and_hasher(v.len(), S2::default());
        for (k, v) in v {
            new.insert((k.into_value(), v.into_value()));
        }
        new
    }
}

#[cfg(feature = "indexmap")]
impl<
    K1: core::hash::Hash + Eq + IntoPatch<K2>,
    V1: core::hash::Hash + Eq + IntoPatch<V2>,
    K2: core::hash::Hash + Eq,
    V2,
    S1: ::std::hash::BuildHasher + Default,
    S2: ::std::hash::BuildHasher + Default,
> FromPatch<indexmap::IndexSet<(K1, V1), S1>>
    for indexmap::IndexMap<K2, V2, S2>
{
    fn from_value(v: indexmap::IndexSet<(K1, V1), S1>) -> Self {
        let mut new = Self::with_capacity_and_hasher(v.len(), S2::default());
        for (k, v) in v {
            new.insert(k.into_value(), v.into_value());
        }
        new
    }
}

#[cfg(feature = "dashmap")]
impl<
    K1: core::hash::Hash + Eq + Clone + IntoPatch<K2>,
    V1: Clone + IntoPatch<V2>,
    K2: core::hash::Hash + Eq,
    V2: core::hash::Hash + Eq,
    S: ::std::hash::BuildHasher + Default,
> FromPatch<dashmap::DashMap<K1, V1>>
    for std::collections::HashSet<(K2, V2), S>
{
    fn from_value(v: dashmap::DashMap<K1, V1>) -> Self {
        let mut new = Self::with_capacity_and_hasher(v.len(), S::default());
        for r in &v {
            new.insert((
                r.key().clone().into_value(),
                r.value().clone().into_value(),
            ));
        }
        new
    }
}

#[cfg(feature = "dashmap")]
impl<
    K1: core::hash::Hash + Eq + IntoPatch<K2>,
    V1: core::hash::Hash + Eq + IntoPatch<V2>,
    K2: core::hash::Hash + Eq,
    V2,
> FromPatch<std::collections::HashSet<(K1, V1)>> for dashmap::DashMap<K2, V2>
{
    fn from_value(v: std::collections::HashSet<(K1, V1)>) -> Self {
        let new = Self::with_capacity(v.len());
        for (k, v) in v {
            new.insert(k.into_value(), v.into_value());
        }
        new
    }
}

#[cfg(all(feature = "dashmap", feature = "indexmap"))]
impl<
    K1: core::hash::Hash + Eq + Clone + IntoPatch<K2>,
    V1: Clone + IntoPatch<V2>,
    K2: core::hash::Hash + Eq,
    V2: core::hash::Hash + Eq,
    S: ::std::hash::BuildHasher + Default,
> FromPatch<dashmap::DashMap<K1, V1>> for indexmap::IndexSet<(K2, V2), S>
{
    fn from_value(v: dashmap::DashMap<K1, V1>) -> Self {
        let mut new = Self::with_capacity_and_hasher(v.len(), S::default());
        for r in &v {
            new.insert((
                r.key().clone().into_value(),
                r.value().clone().into_value(),
            ));
        }
        new
    }
}

#[cfg(all(feature = "dashmap", feature = "indexmap"))]
impl<
    K1: core::hash::Hash + Eq + IntoPatch<K2>,
    V1: core::hash::Hash + Eq + IntoPatch<V2>,
    K2: core::hash::Hash + Eq,
    V2,
    S: ::std::hash::BuildHasher + Default,
> FromPatch<indexmap::IndexSet<(K1, V1), S>> for dashmap::DashMap<K2, V2>
{
    fn from_value(v: indexmap::IndexSet<(K1, V1), S>) -> Self {
        let new = Self::with_capacity(v.len());
        for (k, v) in v {
            new.insert(k.into_value(), v.into_value());
        }
        new
    }
}

#[cfg(feature = "masstree")]
impl<
    'a,
    T1: super::IntoPatch<&'a [u8]>,
    V1: core::hash::Hash + Eq + IntoPatch<V2>,
    V2: std::marker::Sync + std::marker::Send + 'static + masstree::InlineBits,
    S: ::std::hash::BuildHasher + Default,
> FromPatch<std::collections::HashSet<(T1, V1), S>>
    for masstree::MassTree15Inline<V2>
{
    fn from_value(v: std::collections::HashSet<(T1, V1), S>) -> Self {
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
    T1: core::hash::Hash + Eq + super::IntoPatch<&'a [u8]>,
    S: ::std::hash::BuildHasher + Default,
> FromPatch<indexmap::IndexSet<T1, S>> for masstree::MassTree15<()>
{
    fn from_value(v: indexmap::IndexSet<T1, S>) -> Self {
        let new = Self::new();
        for item in v {
            new.insert(item.into_value(), ());
        }
        new
    }
}

#[cfg(all(feature = "indexmap", feature = "masstree"))]
impl<
    'a,
    T1: core::hash::Hash + Eq + super::IntoPatch<&'a [u8]>,
    S: ::std::hash::BuildHasher + Default,
> FromPatch<indexmap::IndexSet<T1, S>> for masstree::MassTree15Inline<()>
where
    (): masstree::InlineBits,
{
    fn from_value(v: indexmap::IndexSet<T1, S>) -> Self {
        let new = Self::new();
        for item in v {
            new.insert(item.into_value(), ());
        }
        new
    }
}

#[cfg(all(feature = "indexmap", feature = "masstree"))]
impl<
    'a,
    K1: core::hash::Hash + Eq + super::IntoPatch<&'a [u8]>,
    V1: core::hash::Hash + Eq + IntoPatch<V2>,
    V2: std::marker::Sync + std::marker::Send + 'static + Clone,
    S: ::std::hash::BuildHasher + Default,
> FromPatch<indexmap::IndexSet<(K1, V1), S>> for masstree::MassTree15<V2>
{
    fn from_value(v: indexmap::IndexSet<(K1, V1), S>) -> Self {
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
    K1: core::hash::Hash + Eq + super::IntoPatch<&'a [u8]>,
    V1: core::hash::Hash + Eq + IntoPatch<V2>,
    V2: std::marker::Sync + std::marker::Send + 'static + masstree::InlineBits,
    S: ::std::hash::BuildHasher + Default,
> FromPatch<indexmap::IndexSet<(K1, V1), S>>
    for masstree::MassTree15Inline<V2>
{
    fn from_value(v: indexmap::IndexSet<(K1, V1), S>) -> Self {
        let new = Self::new();
        for (k, v) in v {
            new.insert(k.into_value(), v.into_value());
        }
        new
    }
}
