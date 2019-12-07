use std::ops::DerefMut;
use std::ops::Deref;
use std::marker::PhantomData;
//use crate::core::util::qwutils::impl_scoped_mut_inner;
use super::*;

mod imp;

/// put a type or mutable reference implementing ILabel inside this to enforce view as Label
pub struct AsLabel<T,E> where T: ILabel<E>, E: Context + 'static {
    pub inner: T,
    _e: PhantomData<E>,
}

impl<T,E> AsLabel<T,E> where T: ILabel<E>, E: Context + 'static {
    #[inline]
    pub fn new(inner: T) -> Self {
        Self{
            inner,
            _e: PhantomData,
        }
    }
}

impl<T,E> From<T> for AsLabel<T,E> where T: ILabel<E>, E: Context + 'static {
    #[inline]
    fn from(inner: T) -> Self {
        Self::new(inner)
    }
}

impl<T,E> Deref for AsLabel<T,E> where T: ILabel<E>, E: Context + 'static {
    type Target=T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T,E> DerefMut for AsLabel<T,E> where T: ILabel<E>, E: Context + 'static {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

/*impl<T,E> ScopedMut for AsLabel<T,E> where T: ILabel<E>, E: Context + 'static {
    impl_scoped_mut_inner!(T);
}*/