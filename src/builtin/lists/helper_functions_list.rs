use crate::{ConstZero, TryIntoPatch};

/// Add item to list without exceeding the specified maximal size
pub fn add_item_to_max_sized_list<T>(
    list: &mut Vec<T>,
    max_size: usize,
    item: T,
) {
    list.push(item);
    if core::intrinsics::unlikely(list.len() < max_size) {
        return;
    }
    let to_remove = list.len() - max_size;
    for _ in 0..to_remove {
        list.remove(0);
    }
}
/// Get a 1d cut out from a 1d color list (1d internally, 2d textures)
#[must_use]
pub fn get_sub_vec_of_vec<T: Copy>(
    vec: &[T],
    width: usize,
    cutout_x: usize,
    cutout_y: usize,
    cutout_width: usize,
    cutout_height: usize,
) -> Vec<T> {
    let mut sub_vec: Vec<T> = Vec::new();

    for y in cutout_y..cutout_y + cutout_height {
        for x in cutout_x..cutout_x + cutout_width {
            sub_vec.push(vec[y * width + x]);
        }
    }
    sub_vec
}
/// Returns what it would be if T was pushed onto [`Vec<T>`]
pub fn combined<T: Clone + Sized>(vec: Vec<T>, other: T) -> Vec<T> {
    let mut new_vec = vec;
    new_vec.push(other);
    new_vec
}
/// Returns what it would be if T was pushed onto [`Vec<T>`]
pub fn combined_list<T: Clone + Sized>(vec: &[T], other: &[T]) -> Vec<T> {
    let mut new_vec = vec.to_vec();
    new_vec.reserve(other.len());
    new_vec.extend_from_slice(other);
    new_vec
}
#[must_use]
/// Returns what it would be if T was pushed onto [`Vec<T>`]
pub fn combined_contents<T: Clone + Sized>(
    vec: Vec<T>,
    other: Vec<T>,
) -> Vec<T> {
    let mut new = vec;
    for i in other {
        new.push(i);
    }
    new
}
/// Get the average value of a list
///
/// Returns None if the length of the input is 0
#[must_use]
pub fn average<
    T: ConstZero
        + Copy
        + PartialEq
        + core::ops::Add<Output = T>
        + core::ops::Div<Output = T>,
>(
    vec: &[T],
) -> Option<T>
where
    usize: TryIntoPatch<T>,
{
    let len = (vec.len()).try_into_value()?;
    if core::intrinsics::unlikely(len == T::ZERO) {
        return None;
    }
    let sum: T = vec.iter().copied().fold(T::ZERO, |a, b| a + b);

    Some(sum / len)
}
#[must_use]
/// Get additions to a list
pub fn get_difference_new<'a, T: core::cmp::PartialEq>(
    old: &'a [T],
    new: &'a [T],
) -> Vec<&'a T> {
    let mut result = Vec::new();
    for i in new {
        if !old.contains(i) {
            result.push(i);
        }
    }
    result
}
#[must_use]
/// Get additions to a list
pub fn get_difference_new_cloned<T: core::cmp::PartialEq + Clone>(
    old: &[T],
    new: &[T],
) -> Vec<T> {
    let mut result = Vec::new();
    for i in new {
        if !old.contains(i) {
            result.push(i.clone());
        }
    }
    result
}
/// Returns if the list has duplicate values
#[must_use]
pub fn has_duplicates<T: core::hash::Hash + Eq>(vec: &Vec<T>) -> bool {
    let mut seen = std::collections::HashSet::new();
    for item in vec {
        if !seen.insert(item) {
            return true;
        }
    }
    false
}
/// Find an item in a list
pub fn find_in_list<T: core::cmp::PartialEq>(
    vec: &[T],
    item: &T,
) -> Option<usize> {
    vec.iter().position(|x| *x == *item)
}
#[must_use]
/// Turn `Vec<Option<T>>` into `Option<Vec<T>>`
pub fn collect_options<T>(vec: Vec<Option<T>>) -> Option<Vec<T>> {
    vec.into_iter().collect()
}
