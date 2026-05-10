/// A trait that allows for data to repeat
pub const trait RepeatData
where
    Self: Sized + Clone,
{
    /// Repeat the given data X times and return a Vec containing the repeated data
    fn repeat_value(self, times: usize) -> Vec<Self>;
}
impl<T: Sized + Clone> RepeatData for T {
    fn repeat_value(self, amount: usize) -> Vec<T> {
        // let mut buffer = Vec::with_capacity(total_size);
        // unsafe {
        //     let ptr = buffer.as_mut_ptr();
        //     core::ptr::write(ptr, color);
        //     let mut filled = 1;
        //     while filled < total_size {
        //         let copy_len = core::cmp::min(filled, total_size - filled);
        //         core::ptr::copy_nonoverlapping(ptr, ptr.add(filled), copy_len);
        //         filled += copy_len;
        //     }
        //     buffer.set_len(total_size);
        // }
        std::vec::from_elem(self, amount)
    }
}

/// A trait that allows for data to repeat inside another container
pub const trait RepeatDataInContainer {
    /// The output type of `Self<T>` should be `Self<Vec<T>>`
    type Output;
    /// Repeat the given data X times and return a Vec containing the repeated data
    fn repeat_value_inside(self, times: usize) -> Self::Output;
}
impl<T: RepeatData> RepeatDataInContainer for Option<T> {
    type Output = core::option::Option<Vec<T>>;
    fn repeat_value_inside(self, times: usize) -> Self::Output {
        self.map(|val| val.repeat_value(times))
    }
}

impl<T: RepeatData> RepeatDataInContainer for Box<T> {
    type Output = Box<Vec<T>>;
    fn repeat_value_inside(self, times: usize) -> Self::Output {
        Box::new((*self).repeat_value(times))
    }
}

impl<T: RepeatData> RepeatDataInContainer for core::cell::Cell<T> {
    type Output = core::cell::Cell<Vec<T>>;
    fn repeat_value_inside(self, times: usize) -> Self::Output {
        core::cell::Cell::new((self.into_inner()).repeat_value(times))
    }
}

impl<T: RepeatData> RepeatDataInContainer for std::rc::Rc<T> {
    type Output = std::rc::Rc<std::vec::Vec<T>>;
    fn repeat_value_inside(self, times: usize) -> Self::Output {
        std::rc::Rc::new((*self).clone().repeat_value(times))
    }
}

impl<T: RepeatData> RepeatDataInContainer for std::sync::Arc<T> {
    type Output = std::sync::Arc<std::vec::Vec<T>>;
    fn repeat_value_inside(self, times: usize) -> Self::Output {
        std::sync::Arc::new((*self).clone().repeat_value(times))
    }
}

impl<T: RepeatData> RepeatDataInContainer for core::cell::RefCell<T> {
    type Output = core::cell::RefCell<std::vec::Vec<T>>;
    fn repeat_value_inside(self, times: usize) -> Self::Output {
        core::cell::RefCell::new(self.into_inner().repeat_value(times))
    }
}

impl<T: RepeatData + Clone, E> RepeatDataInContainer for Result<T, E> {
    type Output = Result<std::vec::Vec<T>, E>;
    fn repeat_value_inside(self, times: usize) -> Self::Output {
        match self {
            Ok(v) => Ok(v.repeat_value(times)),
            Err(e) => Err(e),
        }
    }
}

impl<T: RepeatData> RepeatDataInContainer for std::task::Poll<T> {
    type Output = std::task::Poll<std::vec::Vec<T>>;
    fn repeat_value_inside(self, times: usize) -> Self::Output {
        match self {
            Self::Ready(v) => std::task::Poll::Ready(v.repeat_value(times)),
            Self::Pending => std::task::Poll::Pending,
        }
    }
}

impl<T: RepeatData> RepeatDataInContainer for core::num::Wrapping<T> {
    type Output = core::num::Wrapping<std::vec::Vec<T>>;
    fn repeat_value_inside(self, times: usize) -> Self::Output {
        core::num::Wrapping(self.0.repeat_value(times))
    }
}

impl<T: RepeatData> RepeatDataInContainer for core::cmp::Reverse<T> {
    type Output = core::cmp::Reverse<std::vec::Vec<T>>;
    fn repeat_value_inside(self, times: usize) -> Self::Output {
        core::cmp::Reverse(self.0.repeat_value(times))
    }
}

impl<T: RepeatData + Clone> RepeatDataInContainer for std::sync::Mutex<T> {
    type Output = std::sync::Mutex<std::vec::Vec<T>>;
    fn repeat_value_inside(self, times: usize) -> Self::Output {
        let inner = match Self::into_inner(self) {
            Ok(v) => v,
            Err(poisoned) => poisoned.into_inner(),
        };
        std::sync::Mutex::new(inner.repeat_value(times))
    }
}

impl<T: RepeatData + Clone> RepeatDataInContainer for std::sync::RwLock<T> {
    type Output = std::sync::RwLock<std::vec::Vec<T>>;
    fn repeat_value_inside(self, times: usize) -> Self::Output {
        let inner = match Self::into_inner(self) {
            Ok(v) => v,
            Err(poisoned) => poisoned.into_inner(),
        };
        std::sync::RwLock::new(inner.repeat_value(times))
    }
}
/// Repeat a single character X times and return a string with its repeated value
pub const trait RepeatChar {
    /// Repeat the given char X times and return a String containing the repeated data
    fn repeat_char(self, times: usize) -> String;
}
impl RepeatChar for char {
    fn repeat_char(self, amount: usize) -> String {
        self.repeat_value(amount).iter().collect()
    }
}
