use super::*;
use std::{ops::{DerefMut, Deref}, marker::PhantomData, borrow::{BorrowMut, Borrow}};

/// Simple wrapper type for data to track validity
pub struct Validated<E,T> {
    inner: T,
    valid: bool,
    p: PhantomData<E>,
}

impl<E,T> Validated<E,T> {
    #[inline]
    pub fn new(inner: T) -> Self {
        Self{
            inner,
            valid: false,
            p: PhantomData,
        }
    }
    #[inline]
    pub fn into(self) -> T {
        self.inner
    }
}

impl<E,T> Validation<E> for Validated<E,T> {
    //type Cached = ();

    #[inline]
    fn valid(&self, _: &dyn Any) -> bool {
        self.valid
    }
}
impl<E,T> ValidationMut<E> for Validated<E,T> {
    #[inline]
    fn validate(&mut self) -> Arc<dyn Any> {
        self.valid = true;
        Arc::new(())
    }
}

impl<E,T> From<T> for Validated<E,T> {
    #[inline]
    fn from(v: T) -> Self {
        Self::new(v)
    }
}

impl<E,T> Deref for Validated<E,T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<E,T> DerefMut for Validated<E,T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.valid = false;
        &mut self.inner
    }
}

impl<E,T> Borrow<T> for Validated<E,T> {
    #[inline]
    fn borrow(&self) -> &T {
        self
    }
}
impl<E,T> BorrowMut<T> for Validated<E,T> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut T {
        self
    }
}

impl<E,T> AsRef<T> for Validated<E,T> {
    #[inline]
    fn as_ref(&self) -> &T {
        self
    }
}
impl<E,T> AsMut<T> for Validated<E,T> {
    #[inline]
    fn as_mut(&mut self) -> &mut T {
        self
    }
}
