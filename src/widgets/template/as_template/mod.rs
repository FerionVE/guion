use std::ops::DerefMut;
use std::ops::Deref;
use std::marker::PhantomData;
use crate::core::util::ScopedMut;
//use crate::core::util::qwutils::impl_scoped_mut_inner;
use super::*;

mod imp;

/// put a type or mutable reference implementing ITemplate inside this to enforce view as Template
pub struct AsTemplate<T,U,E> where T: ScopedMut<T=U> + 'static, U: ITemplate<E>, E: Context + 'static {
    pub inner: T,
    _e: PhantomData<E>,
}

impl<T,U,E> AsTemplate<T,U,E> where T: ScopedMut<T=U> + 'static, U: ITemplate<E>, E: Context + 'static {
    #[inline]
    pub fn new(inner: T) -> Self {
        Self{
            inner,
            _e: PhantomData,
        }
    }
}

impl<T,U,E> From<T> for AsTemplate<T,U,E> where T: ScopedMut<T=U> + 'static, U: ITemplate<E>, E: Context + 'static {
    #[inline]
    fn from(inner: T) -> Self {
        Self::new(inner)
    }
}

impl<T,U,E> Deref for AsTemplate<T,U,E> where T: ScopedMut<T=U> + 'static, U: ITemplate<E>, E: Context + 'static {
    type Target=T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T,U,E> DerefMut for AsTemplate<T,U,E> where T: ScopedMut<T=U> + 'static, U: ITemplate<E>, E: Context + 'static {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

/*impl<T,U,E> ScopedMut for AsTemplate<T,U,E> where T: ScopedMut<T=U> + 'static, U: ITemplate<E>, E: Context + 'static {
    impl_scoped_mut_inner!(T);
}*/