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
use mirl_extensions_core::ListLike;

use crate::{ConstZero, TryIntoPatch};

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

/// Returns the combination of 2 lists
pub const trait ListsCombined<T: Clone + Sized> {
    /// Returns what it would be if `T` was pushed onto [`Vec<T>`]
    fn combined_with(self, other: Vec<T>) -> Vec<T>;
}

/// Get additions to a list
pub const trait ListAverage<T> {
    /// Get additions to a list
    fn average(&self) -> Option<T>;
}

/// Returns if the list has duplicate values
///
/// This evaluates in O(N^2) time
pub const trait ListHasDuplicates<T: PartialEq> {
    /// Returns if the list has duplicate values
    ///
    /// This evaluates in O(N^2) time
    fn has_duplicates(&self) -> bool;
}
/// Find the first instance of T
pub const trait Index<T: core::cmp::PartialEq> {
    /// Find the first instance of T
    fn find(&self, item: &T) -> Option<usize>;
}
impl<T: core::cmp::Eq> Index<T> for Vec<T> {
    fn find(&self, item: &T) -> Option<usize> {
        self.iter().position(|x| *x == *item)
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
impl<T, List: ListLike<T, usize>> ListPushOrReplaceOnMaxSize<T> for List {
    fn push_or_replace_on_max_size(&mut self, max_size: usize, item: T) {
        self.push(item);
        if self.len() < max_size {
            return;
        }
        let to_remove = self.len() - max_size;
        for _ in 0..to_remove {
            self.remove(0);
        }
    }
}
impl<T: PartialEq, List: ListLike<T, usize>> ListHasDuplicates<T> for List {
    fn has_duplicates(&self) -> bool {
        for i in 0..self.len() {
            for j in (i + 1)..self.len() {
                if unsafe { self.get_unchecked(i).eq(self.get_unchecked(j)) } {
                    return true;
                }
            }
        }
        false
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
        let mut sub_vec: Self = Self::new();

        for y in cutout_y..cutout_y + cutout_height {
            for x in cutout_x..cutout_x + cutout_width {
                sub_vec.push(self[y * vec_width + x]);
            }
        }
        sub_vec
    }
}
impl<'a, T: PartialEq, List: ListLike<T, usize>> ListGetNewItems<'a, T> for List {
    fn get_new_items(&'a self, old: &'a [T]) -> Vec<&'a T> {
        let mut result = Vec::new();
        for i in self.iter() {
            if !old.contains(i) {
                result.push(i);
            }
        }
        result
    }
    fn get_old_items(&'a self, new: &'a [T]) -> Vec<&'a T> {
        let mut result = Vec::new();
        for i in new {
            if !self.contains(i) {
                result.push(i);
            }
        }
        result
    }
}
#[allow(clippy::use_self)] // No clippy, Self is not allowed in this context
impl<T: core::cmp::PartialEq + Clone> ListGetNewItemsCloned<T> for Vec<T> {
    fn get_new_items_cloned(&self, old: &[T]) -> Vec<T> {
        let mut result = Vec::new();
        for i in self {
            if !old.contains(i) {
                result.push(i.clone());
            }
        }
        result
    }
    fn get_old_items_cloned(&self, new: &[T]) -> Vec<T> {
        new.get_new_items_cloned(self)
    }
}
#[allow(clippy::use_self)] // No clippy, Self is not allowed in this context
impl<T: core::cmp::PartialEq + Clone> ListGetNewItemsCloned<T> for [T] {
    fn get_new_items_cloned(&self, old: &[T]) -> Vec<T> {
        let mut result = Vec::new();
        for i in self {
            if !old.contains(i) {
                result.push(i.clone());
            }
        }
        result
    }
    fn get_old_items_cloned(&self, new: &[T]) -> Vec<T> {
        new.get_new_items_cloned(self)
    }
}

impl<T: core::clone::Clone> ListCombined<T> for Vec<T> {
    fn combined(self, other: T) -> Self {
        let mut new_vec = self;
        new_vec.push(other);
        new_vec
    }
}

impl<T: core::clone::Clone> ListsCombined<T> for Vec<T> {
    fn combined_with(self, other: Self) -> Self {
        let mut new_vec = self;
        new_vec.reserve(other.len());
        new_vec.extend(other);
        new_vec
    }
}

impl<T: ConstZero + Copy + PartialEq + core::ops::Add<Output = T> + core::ops::Div<Output = T>>
    ListAverage<T> for [T]
where
    usize: TryIntoPatch<T>,
{
    fn average(&self) -> Option<T> {
        let len = (self.len()).try_into_value()?;
        if core::intrinsics::unlikely(len == T::ZERO) {
            return None;
        }
        let sum: T = self.iter().copied().fold(T::ZERO, |a, b| a + b);

        Some(sum / len)
    }
}
