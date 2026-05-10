/// A trait for turning `Vec<Option<T>>` into `Option<Vec<T>>`
pub const trait CollectOptions
where
    Self: Sized,
{
    /// `Option<Vec<T>>`
    type Output;
    /// Turn `Vec<Option<T>>` into `Option<Vec<T>>`
    fn collect_options(self) -> Self::Output;
}
impl<T> CollectOptions for Vec<Option<T>> {
    type Output = Option<Vec<T>>;
    fn collect_options(self) -> Self::Output {
        self.into_iter().collect()
    }
}
/// Helper functions containing the actual implementations
pub mod helper_functions_list;
use crate::{ConstZero, TryIntoPatch, helper_functions_list::*};

/// Add item to list without exceeding the specified maximal size
pub const trait ListPushOrReplaceOnMaxSize<T> {
    /// Add item to list without exceeding the specified maximal size
    fn push_or_replace_on_max_size(&mut self, max_size: usize, item: T);
}

/// Cut out a 2d area from a 1d array and return it as a 1d array
pub const trait ListGetRegion<T: Copy> {
    /// Cut out a 2d area from a 1d array and return it as a 1d array
    fn get_region(
        &self,
        vec_width: usize,
        cutout_x: usize,
        cutout_y: usize,
        cutout_width: usize,
        cutout_height: usize,
    ) -> Vec<T>;
}

/// Returns what it would be if `T` was pushed onto [`Vec<T>`]
pub const trait ListCombined<T: Clone + Sized> {
    /// Returns what it would be if `T` was pushed onto [`Vec<T>`]
    fn combined(self, other: T) -> Vec<T>;
}

/// Get additions to a list
pub const trait ListAverage<T> {
    /// Get additions to a list
    fn average(&self) -> Option<T>;
}

/// Returns if the list has duplicate values
pub const trait ListHasDuplicates<T: core::hash::Hash + Eq> {
    /// Returns if the list has duplicate values
    fn has_duplicates(&self) -> bool;
}
/// Find the first instance of T
pub const trait Index<T: core::cmp::PartialEq> {
    /// Find the first instance of T
    fn find(&self, item: &T) -> Option<usize>;
}
impl<T: core::cmp::Eq> Index<T> for Vec<T> {
    fn find(&self, item: &T) -> Option<usize> {
        find_in_list(self, item)
    }
}
/// Other list functions
pub const trait ListMisc<T> {
    #[must_use]
    #[allow(patterns_in_fns_without_body)]
    /// Return the sorted version of ourself
    fn sorted(mut self) -> Self;
    #[must_use]
    #[allow(patterns_in_fns_without_body)]
    /// Return the sorted version of ourself
    fn sorted_by<F>(mut self, compare: F) -> Self
    where
        F: FnMut(&T, &T) -> std::cmp::Ordering;
}
impl<T: std::cmp::Ord> ListMisc<T> for Vec<T> {
    fn sorted(mut self) -> Self {
        self.sort();
        self
    }
    fn sorted_by<F>(mut self, compare: F) -> Self
    where
        F: FnMut(&T, &T) -> std::cmp::Ordering,
    {
        self.sort_by(compare);
        self
    }
}

/// Get the difference between 2 lists
pub const trait ListGetNewItems<'a, T: core::cmp::PartialEq> {
    /// Get what is new in the list compared to another
    fn get_new_items(&'a self, old: &'a [T]) -> Vec<&'a T>;
    /// Get what is new in the other list compared to this one
    fn get_old_items(&'a self, new: &'a [T]) -> Vec<&'a T>;
}

/// Get the difference between 2 lists
pub const trait ListGetNewItemsCloned<T: core::cmp::PartialEq + Clone> {
    /// Get what is new in the list compared to another
    fn get_new_items_cloned(&self, old: &[T]) -> Vec<T>;
    /// Get what is new in the other list compared to this one
    fn get_old_items_cloned(&self, new: &[T]) -> Vec<T>;
}
impl<T> ListPushOrReplaceOnMaxSize<T> for Vec<T> {
    fn push_or_replace_on_max_size(&mut self, max_size: usize, item: T) {
        add_item_to_max_sized_list(self, max_size, item);
    }
}
impl<T: core::cmp::Eq + core::hash::Hash> ListHasDuplicates<T> for Vec<T> {
    fn has_duplicates(&self) -> bool {
        has_duplicates(self)
    }
}
impl<T: Copy> ListGetRegion<T> for Vec<T> {
    fn get_region(
        &self,
        vec_width: usize,
        cutout_x: usize,
        cutout_y: usize,
        cutout_width: usize,
        cutout_height: usize,
    ) -> Self {
        get_sub_vec_of_vec(
            self,
            vec_width,
            cutout_x,
            cutout_y,
            cutout_width,
            cutout_height,
        )
    }
}
impl<'a, T: core::cmp::PartialEq> ListGetNewItems<'a, T> for Vec<T> {
    fn get_new_items(&'a self, old: &'a [T]) -> Vec<&'a T> {
        get_difference_new(old, self)
    }
    fn get_old_items(&'a self, new: &'a [T]) -> Vec<&'a T> {
        get_difference_new(self, new)
    }
}
#[allow(clippy::use_self)] // No clippy, Self is not allowed in this context
impl<T: core::cmp::PartialEq + Clone> ListGetNewItemsCloned<T> for Vec<T> {
    fn get_new_items_cloned(&self, old: &[T]) -> Vec<T> {
        get_difference_new_cloned(old, self)
    }
    fn get_old_items_cloned(&self, new: &[T]) -> Vec<T> {
        get_difference_new_cloned(self, new)
    }
}

impl<T: core::clone::Clone> ListCombined<T> for Vec<T> {
    fn combined(self, other: T) -> Self {
        combined(self, other)
    }
}

impl<
    T: ConstZero
        + Copy
        + PartialEq
        + core::ops::Add<Output = T>
        + core::ops::Div<Output = T>,
> ListAverage<T> for Vec<T>
where
    usize: TryIntoPatch<T>,
{
    fn average(&self) -> Option<T> {
        average(self)
    }
}

/// A trait for structs that are supposed to act like lists
///
/// The required helper trait [`ListLikeHelper`] is automatically implemented for all structs that implement this trait
pub const trait ListLike<T, Idx>:
    core::ops::Index<Idx> + core::ops::IndexMut<Idx> + ListLikeHelper<T, Idx>
{
    /// Add an item into the container and return a mutable reference to it
    fn push_mut(&mut self, value: T) -> &mut T;
    /// Try to remove the value
    fn try_remove(&mut self, index: Idx) -> Option<T>;
    /// Swap 2 values without disrupting the rest of the list
    fn swap_values(&mut self, a: Idx, b: Idx);
    /// Get how many items are in the container
    fn len(&self) -> usize;
    /// Remove the last item from the container and return it if it exists
    fn pop(&mut self) -> Option<T>;
    /// Insert an item anywhere into the container
    fn insert_mut(&mut self, index: usize, value: T) -> &mut T;
    /// Replace the value in the container with the given value
    fn try_replace(&mut self, index: usize, value: T) -> Option<T>;
    /// Reserve space for X more values if possible
    ///
    /// # Errors
    /// When it fails to allocate more which most often happens when the total length goes beyond [`isize::MAX`]
    fn try_reserve(
        &mut self,
        amount: usize,
    ) -> Result<(), std::collections::TryReserveError>;
    /// Returns the idx at which the given item was found
    fn find_position(&self, item: &T) -> Option<Idx>;
    /// Remove all values within the container
    fn clear(&mut self);
}
/// A few helper functions for any list like object to reduce code duplication
///
/// It is automatically implemented for all structs that implement [`ListLike`]
pub const trait ListLikeHelper<T, Idx> {
    /// Remove an item from the container
    ///
    /// This function is unsafe and may cause a panic, for the non panic version call [`try_remove`](ListLike::try_remove)
    fn remove(&mut self, index: Idx) -> T;
    /// Add an item to the container
    fn push(&mut self, value: T);
    /// Checks if the current amount of items is equal to zero
    fn is_empty(&mut self) -> bool;
    /// Insert an item anywhere into the container
    fn insert(&mut self, index: usize, value: T);
    /// Replace the value in the container with the given value
    ///
    /// This function is unsafe and may cause a panic, for the non panic version call [`try_replace`](ListLike::try_replace)
    fn replace(&mut self, index: usize, value: T) -> T;
    /// Check if the container contains the given item
    fn contains(&self, item: &T) -> bool;
    /// Reserve space for X more values
    ///
    /// This function is unsafe and may cause a panic, for the non panic version call [`try_reserve`](ListLike::try_reserve)
    fn reserve(&mut self, amount: usize);
}

impl<T: std::cmp::PartialEq> ListLike<T, usize> for Vec<T> {
    fn push_mut(&mut self, value: T) -> &mut T {
        self.push_mut(value)
    }
    fn try_remove(&mut self, index: usize) -> Option<T> {
        vec_try_remove(self, index)
    }
    fn swap_values(&mut self, a: usize, b: usize) {
        self.swap(a, b);
    }
    fn len(&self) -> usize {
        self.len()
    }
    fn insert_mut(&mut self, index: usize, value: T) -> &mut T {
        self.insert_mut(index, value)
    }
    fn pop(&mut self) -> Option<T> {
        self.pop()
    }
    fn try_replace(&mut self, index: usize, value: T) -> Option<T> {
        if index >= self.len() {
            return None;
        }
        let mut value = value;

        core::mem::swap(&mut self[index], &mut value);

        Some(value)
    }
    fn find_position(&self, item: &T) -> Option<usize> {
        self.iter().position(|x| (*x).eq(item))
    }
    fn try_reserve(
        &mut self,
        amount: usize,
    ) -> Result<(), std::collections::TryReserveError> {
        self.try_reserve(amount)
    }
    fn clear(&mut self) {
        self.clear();
    }
}
impl<S: ListLike<T, Idx>, T, Idx> ListLikeHelper<T, Idx> for S {
    fn remove(&mut self, index: Idx) -> T {
        self.try_remove(index).unwrap()
    }
    fn push(&mut self, value: T) {
        self.push_mut(value);
    }
    fn is_empty(&mut self) -> bool {
        self.len() == 0
    }
    fn insert(&mut self, index: usize, value: T) {
        self.insert_mut(index, value);
    }
    fn replace(&mut self, index: usize, value: T) -> T {
        self.try_replace(index, value).unwrap()
    }
    fn contains(&self, item: &T) -> bool {
        self.find_position(item).is_some()
    }
    fn reserve(&mut self, amount: usize) {
        self.try_reserve(amount).unwrap();
    }
}

use mirl_core::misc::vec_try_remove;

/// Encode `Vec<String>` into `Vec<u8>`
pub const trait StringListEncoder {
    /// Encode `Vec<String>` into `Vec<u8>`
    fn strings_to_bytes(&self) -> Vec<u8>;
}

/// Decode `Vec<u8>` into `Vec<String>`
pub const trait StringListDecoder: Sized {
    /// Decode `Vec<u8>` into `Vec<String>`
    fn bytes_to_strings(&self) -> Option<Vec<String>>;
}
impl StringListEncoder for &[String] {
    fn strings_to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&(self.len() as u32).to_le_bytes());
        for s in *self {
            let string_bytes = s.as_bytes();
            bytes.extend_from_slice(&(string_bytes.len() as u32).to_le_bytes());
            bytes.extend_from_slice(string_bytes);
        }
        bytes
    }
}
impl StringListEncoder for Vec<String> {
    fn strings_to_bytes(&self) -> Vec<u8> {
        self.as_slice().strings_to_bytes()
    }
}

impl StringListDecoder for &[u8] {
    #[allow(clippy::cast_possible_truncation)]
    fn bytes_to_strings(&self) -> Option<Vec<String>> {
        let mut cursor = 0;
        let mut strings = Vec::new();

        let num_strings = self
            .get(cursor..cursor + 4)
            .map(|b| u32::from_le_bytes(b.try_into().unwrap()) as usize)?;
        cursor += 4;

        for _ in 0..num_strings {
            let len = self
                .get(cursor..cursor + 4)
                .map(|b| u32::from_le_bytes(b.try_into().unwrap()) as usize)?;
            cursor += 4;

            let s = self
                .get(cursor..cursor + len)
                .and_then(|b| String::from_utf8(b.to_vec()).ok())?;
            strings.push(s);
            cursor += len;
        }

        Some(strings)
    }
}

impl StringListDecoder for Vec<u8> {
    #[allow(clippy::cast_possible_truncation)]
    fn bytes_to_strings(&self) -> Option<Vec<String>> {
        self.as_slice().bytes_to_strings()
    }
}
