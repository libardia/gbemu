use std::ops::Deref;

pub struct Immutable<T> {
    inner: T,
}

impl<T> Deref for Immutable<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> From<T> for Immutable<T> {
    fn from(inner: T) -> Self {
        Immutable { inner }
    }
}
