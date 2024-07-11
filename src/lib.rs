/// This takes inspiration from the `SendWrapper` crate.
/// But is specialized for JsValue based objects. Includes support for some extra traits
/// and a way to convert values into ThreadSafeJsValue.
///
/// There is also a macro to implement the From trait for ThreadSafeJsValue, that adds a type alias
/// with a suffix.
use std::{
    mem::{self, ManuallyDrop},
    ops::Deref,
};

use paste::paste;
use wasm_bindgen::{convert::IntoWasmAbi, JsValue};

const NOT_ON_CURRENT_THREAD: &str = "ThreadSafeJsValue is not on the current thread";

/// A wrapper around a value that is intended to be passed between threads.
///
/// The main use is to wrap JsValue based objects that are not Send or Sync.
/// There is a small overhead to check if the value is on the current thread.
pub struct ThreadSafeJsValue<T> {
    value: ManuallyDrop<T>,
    thread_id: std::thread::ThreadId,
}

impl<T> ThreadSafeJsValue<T>
where
    T: IntoWasmAbi,
{
    /// Creates a new ThreadSafeJsValue.
    ///
    /// # Example
    ///
    /// ```
    /// use wasm_bindgen::JsValue;
    /// use thread_safe_js_value::ThreadSafeJsValue;
    ///
    /// let js_value = JsValue::from(42);
    ///
    /// let js_value_ts = ThreadSafeJsValue::new(js_value);
    ///
    /// assert_eq!(js_value_ts.value(), &JsValue::from(42));
    /// ```
    pub fn new(value: T) -> Self {
        Self {
            value: ManuallyDrop::new(value),
            thread_id: std::thread::current().id(),
        }
    }
}

impl<T> Drop for ThreadSafeJsValue<T> {
    /// Drops the value if it is on the current thread.
    ///
    /// # Panics
    ///
    /// Panics if the value is not on the current thread, except when the value does not need to be dropped.
    #[track_caller]
    fn drop(&mut self) {
        if !mem::needs_drop::<T>() || self.thread_id == std::thread::current().id() {
            unsafe {
                ManuallyDrop::drop(&mut self.value);
            }
        } else {
            invalid_thread();
        }
    }
}

impl<T> ThreadSafeJsValue<T> {
    /// Checks if the ThreadSafeJsValue is valid for the current thread.
    ///
    /// # Panics
    ///
    /// Panics if the ThreadSafeJsValue is not valid for the current thread.
    #[track_caller]
    fn check_thread(&self) {
        if self.thread_id != std::thread::current().id() {
            invalid_thread();
        }
    }

    /// Checks if the ThreadSafeJsValue is valid for the current thread.
    pub fn is_valid(&self) -> bool {
        self.thread_id == std::thread::current().id()
    }

    /// Gets the value from the ThreadSafeJsValue.
    ///
    /// # Panics
    ///
    /// Panics if the ThreadSafeJsValue is not valid for the current thread.
    #[track_caller]
    pub fn value(&self) -> &T {
        self.check_thread();
        &self.value
    }

    /// Tries to get the value from the ThreadSafeJsValue.
    #[track_caller]
    pub fn try_value(&self) -> Result<&T, std::io::Error> {
        if self.thread_id == std::thread::current().id() {
            Ok(&self.value)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                NOT_ON_CURRENT_THREAD,
            ))
        }
    }
}

/// # Safety
/// We assert that the thread_id is the same as the current thread_id
/// when we dereference the value.
unsafe impl<T> Send for ThreadSafeJsValue<T> {}

/// # Safety
/// We assert that the thread_id is the same as the current thread_id
/// when we dereference the value.
unsafe impl<T> Sync for ThreadSafeJsValue<T> {}

impl<T> Clone for ThreadSafeJsValue<T>
where
    T: Clone,
{
    /// Clones the ThreadSafeJsValue.
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            thread_id: self.thread_id,
        }
    }
}

impl<T> std::fmt::Debug for ThreadSafeJsValue<T>
where
    T: std::fmt::Debug,
{
    /// Formats the value for debugging.
    ///
    /// # Panics
    ///
    /// Panics if the ThreadSafeJsValue is not valid for the current thread.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("ThreadId:{:?}", self.thread_id))?;
        f.write_fmt(format_args!("Value:{:?}", self.value.deref()))
    }
}

impl<T> std::fmt::Display for ThreadSafeJsValue<T>
where
    T: std::fmt::Display,
{
    /// Formats the value.
    ///
    /// # Panics
    ///
    /// Panics if the ThreadSafeJsValue is not valid for the current thread.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.deref().fmt(f)
    }
}

impl<T> PartialEq for ThreadSafeJsValue<T>
where
    T: PartialEq,
{
    /// Compares the value for equality.
    ///
    /// # Panics
    ///
    /// Panics if the ThreadSafeJsValue is not valid for the current thread.
    fn eq(&self, other: &Self) -> bool {
        self.value.deref() == other.value.deref()
    }
}

impl<T> Eq for ThreadSafeJsValue<T> where T: Eq {}

impl<T> std::hash::Hash for ThreadSafeJsValue<T>
where
    T: std::hash::Hash,
{
    /// Hashes the value.
    ///
    /// # Panics
    ///
    /// Panics if the ThreadSafeJsValue is not valid for the current thread.
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.deref().hash(state)
    }
}

impl<T> std::cmp::PartialOrd for ThreadSafeJsValue<T>
where
    T: std::cmp::PartialOrd,
{
    /// Compares the value for ordering.
    ///
    /// # Panics
    ///
    /// Panics if the ThreadSafeJsValue is not valid for the current thread.
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.deref().partial_cmp(other.value.deref())
    }
}

impl<T> std::cmp::Ord for ThreadSafeJsValue<T>
where
    T: std::cmp::Ord,
{
    /// Compares the value for ordering.
    ///
    /// # Panics
    ///
    /// Panics if the ThreadSafeJsValue is not valid for the current thread.
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.deref().cmp(other.value.deref())
    }
}

impl<T> std::ops::Deref for ThreadSafeJsValue<T> {
    type Target = T;

    /// Dereferences the value.
    ///
    /// # Panics
    ///
    /// Panics if the ThreadSafeJsValue is not valid for the current thread.
    #[track_caller]
    fn deref(&self) -> &Self::Target {
        self.check_thread();
        &self.value
    }
}

impl<T> std::ops::DerefMut for ThreadSafeJsValue<T> {
    /// Dereferences the value as a mutable reference.
    ///
    /// # Panics
    ///
    /// Panics if the ThreadSafeJsValue is not valid for the current thread.
    #[track_caller]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.check_thread();
        &mut self.value
    }
}

/// A trait for converting a value into a ThreadSafeJsValue.
///
/// This is useful for converting values that are not Send or Sync.
/// When they don't have the From trait implemented for ThreadSafeJsValue.
pub trait IntoThreadSafeJsValue: Sized {
    /// Converts the value into a ThreadSafeJsValue.
    fn into_thread_safe_js_value(self) -> ThreadSafeJsValue<Self>
    where
        Self: IntoWasmAbi;
}

impl<T> IntoThreadSafeJsValue for T
where
    T: IntoWasmAbi,
{
    fn into_thread_safe_js_value(self) -> ThreadSafeJsValue<Self> {
        ThreadSafeJsValue::new(self)
    }
}

#[cold]
#[track_caller]
#[inline(never)]
fn invalid_thread() -> ! {
    panic!("{}", NOT_ON_CURRENT_THREAD);
}

/// This is a helper macro to implement the From trait for ThreadSafeJsValue.
///
/// This also adds a type alias for the ThreadSafeJsValue with a suffix.
/// e.g. JsValue -> JsValueTS
#[macro_export]
macro_rules! impl_thread_safe_js_value {
    ($type:ty) => {
        impl From<$type> for ThreadSafeJsValue<$type> {
            fn from(value: $type) -> Self {
                Self::new(value)
            }
        }
        paste! {pub type [<$type TS>] = ThreadSafeJsValue<$type>;}
    };
    ($type:ty, $suffix:expr) => {
        impl From<$type> for ThreadSafeJsValue<$type> {
            fn from(value: $type) -> Self {
                Self::new(value)
            }
        }
        paste! {pub type [< $type $suffix>] = ThreadSafeJsValue<$type>;}
    };
}

impl_thread_safe_js_value!(JsValue);

#[cfg(all(test, target_arch = "wasm32"))]
mod tests {
    #[cfg(test)]
    use super::*;
    #[cfg(test)]
    use wasm_bindgen::JsValue;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn test_thread_safe_js_value() {
        let js_value = JsValue::from(42);
        let thread_safe_js_value = ThreadSafeJsValue::new(js_value);
        assert_eq!(thread_safe_js_value.value(), &JsValue::from(42));
    }

    #[wasm_bindgen_test]
    fn test_thread_safe_js_value_clone() {
        let js_value = JsValue::from(42);
        let thread_safe_js_value = ThreadSafeJsValue::new(js_value);
        let cloned_thread_safe_js_value = thread_safe_js_value.clone();
        assert_eq!(cloned_thread_safe_js_value.value(), &JsValue::from(42));
    }

    #[wasm_bindgen_test]
    fn test_thread_safe_js_value_try_value() {
        let js_value = JsValue::from(42);
        let thread_safe_js_value = ThreadSafeJsValue::new(js_value);
        assert_eq!(
            thread_safe_js_value.try_value().unwrap(),
            &JsValue::from(42)
        );
    }

    #[wasm_bindgen_test]
    fn test_thread_into_thread_safe_js_value() {
        let js_value = JsValue::from(42);
        let thread_safe_js_value = js_value.into_thread_safe_js_value();
        assert_eq!(thread_safe_js_value.value(), &JsValue::from(42));
    }
}
