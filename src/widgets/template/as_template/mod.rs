use std::ops::DerefMut;
use std::ops::Deref;
use std::marker::PhantomData;
//use crate::core::util::qwutils::impl_scoped_mut_inner;
use super::*;

mod imp;

/// put a type or mutable reference implementing ITemplate inside this to enforce view as Template
pub struct AsTemplate<T,E,C> where C: AsRef<T> + AsMut<T>, T: ITemplate<E>, E: Context + 'static {
    pub inner: C,
    _e: PhantomData<E>,
    _t: PhantomData<T>,
}

impl<T,E,C> AsTemplate<T,E,C> where C: AsRef<T> + AsMut<T>, T: ITemplate<E>, E: Context + 'static {
    #[inline]
    pub fn new(inner: C) -> Self {
        Self{
            inner,
            _e: PhantomData,
            _t: PhantomData,
        }
    }
}

impl<T,E,C> From<C> for AsTemplate<T,E,C> where C: AsRef<T> + AsMut<T>, T: ITemplate<E>, E: Context + 'static {
    #[inline]
    fn from(inner: C) -> Self {
        Self::new(inner)
    }
}

impl<T,E,C> Deref for AsTemplate<T,E,C> where C: AsRef<T> + AsMut<T>, T: ITemplate<E>, E: Context + 'static {
    type Target=T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

impl<T,E,C> DerefMut for AsTemplate<T,E,C> where C: AsRef<T> + AsMut<T>, T: ITemplate<E>, E: Context + 'static {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.as_mut()
    }
}

/*impl<T,E,C> ScopedMut for AsTemplate<T,E,C> where C: AsRef<T> + AsMut<T>, T: ITemplate<E>, E: Context + 'static {
    impl_scoped_mut_inner!(T);
}*/