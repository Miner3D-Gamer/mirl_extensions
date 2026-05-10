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
