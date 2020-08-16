use super::*;
use std::{rc::Rc, cell::{RefMut, RefCell}, marker::PhantomData};

pub struct SMA<'a,T,U,F> where F: SMALens<T,U> {
    inner: Rc<RefCell<&'a mut T>>,
    f: F,
    p: PhantomData<U>,
}

impl<'a,T> SMA<'a,T,T,()> {
    #[inline]
    pub fn new(v: &'a mut T) -> Self {
        Self{
            inner: Rc::new(RefCell::new(v)),
            f: (),
            p: PhantomData,
        }
    }
}

impl<'a,T,U,F> SMA<'a,T,U,F> where F: SMALens<T,U> {
    #[inline]
    pub fn fork_with_lens<V,G>(&self, lens: G) -> SMA<'a,T,V,G> where G: SMALens<T,V> {
        SMA{
            inner: self.inner.refc(),
            f: lens,
            p: PhantomData,
        }
    }

    #[inline]
    pub fn borrow_mut(&mut self) -> RefMut<'_,U> {
        let r = self.inner.borrow_mut();
        let f = &mut self.f;
        RefMut::map(r,#[inline] move |v| f.lens_mut(v) )
    }

    #[inline]
    pub fn fork(&self) -> Self {
        self.clone()
    }
}

impl<'a,T,U,F> Clone for SMA<'a,T,U,F> where F: SMALens<T,U> {
    fn clone(&self) -> Self {
        Self{
            inner: self.inner.refc(),
            f: self.f.clone(),
            p: PhantomData,
        }
    }
}
impl<'a,T,U,F> RefClonable for SMA<'a,T,U,F> where F: SMALens<T,U> {
    fn refc(&self) -> Self {
        self.clone()
    }
}

pub trait SMALens<T,U>: Clone {
    fn lens_mut<'a>(&mut self, v: &'a mut T) -> &'a mut U;
}

impl<T,U,F> SMALens<T,U> for F where F: FnMut(&mut T) -> &mut U + Clone {
    #[inline]
    fn lens_mut<'a>(&mut self, v: &'a mut T) -> &'a mut U {
        self(v)
    }
}

impl<T> SMALens<T,T> for () {
    #[inline]
    fn lens_mut<'a>(&mut self, v: &'a mut T) -> &'a mut T {
        v
    }
}
