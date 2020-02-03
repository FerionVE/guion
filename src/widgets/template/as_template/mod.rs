use std::borrow::BorrowMut;
use std::borrow::Borrow;
use std::ops::DerefMut;
use std::ops::Deref;
use std::marker::PhantomData;
//use util::qwutils::impl_scoped_mut_inner;
use super::*;

mod imp;

/// put a type or mutable reference implementing ITemplate inside this to enforce view as Template
#[repr(transparent)]
pub struct AsTemplate<T,E,C> where C: Borrow<T> + BorrowMut<T>, T: ITemplate<E>, E: Env + 'static {
    pub inner: C,
    _e: PhantomData<E>,
    _t: PhantomData<T>,
}
#[allow(dead_code)]
impl<T,E,C> AsTemplate<T,E,C> where C: Borrow<T> + BorrowMut<T>, T: ITemplate<E>, E: Env + 'static {
    #[inline]
    pub fn new(inner: C) -> Self {
        Self{
            inner,
            _e: PhantomData,
            _t: PhantomData,
        }
    }
    #[inline]
    pub fn from_ref<'a>(r: &'a C) -> &'a Self {
        unsafe{
            &*(r as *const C as *const Self)
        }
    }
    #[inline]
    pub fn from_ref_mut<'a>(r: &'a mut C) -> &'a mut Self {
        unsafe{
            &mut *(r as *mut C as *mut Self)
        }
    }
}

impl<T,E,C> From<C> for AsTemplate<T,E,C> where C: Borrow<T> + BorrowMut<T>, T: ITemplate<E>, E: Env + 'static {
    #[inline]
    fn from(inner: C) -> Self {
        Self::new(inner)
    }
}

impl<T,E,C> Deref for AsTemplate<T,E,C> where C: Borrow<T> + BorrowMut<T>, T: ITemplate<E>, E: Env + 'static {
    type Target=T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.inner.borrow()
    }
}

impl<T,E,C> DerefMut for AsTemplate<T,E,C> where C: Borrow<T> + BorrowMut<T>, T: ITemplate<E>, E: Env + 'static {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.borrow_mut()
    }
}

/*impl<T,E,C> ScopedMut for AsTemplate<T,E,C> where C: Borrow<T> + BorrowMut<T>, T: ITemplate<E>, E: Env + 'static {
    impl_scoped_mut_inner!(T);
}*/