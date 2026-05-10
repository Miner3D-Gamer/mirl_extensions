use crate::FromPatch;

impl<T> FromPatch<T> for tokio::sync::Mutex<T> {
    fn from_value(value: T) -> Self {
        Self::new(value)
    }
}

impl<T> FromPatch<T> for tokio::sync::RwLock<T> {
    fn from_value(value: T) -> Self {
        Self::new(value)
    }
}
