//! RefCell-based Shared Mutable Access helper for mutable immediate widgets e.g. when multiple parts need to mutably reference to the same thing
use crate::widgets::util::{state::{AtomState, AtomStateMut}};

use super::*;
use std::{rc::Rc, cell::{RefMut, RefCell}, marker::PhantomData};

/// RefCell-based Shared Mutable Access helper for mutable immediate widgets e.g. when multiple parts need to mutably reference to the same thing
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
    pub fn borrow_mut(&self) -> RefMut<U> {
        let r = self.inner.borrow_mut();
        let f = &self.f;
        RefMut::map(r,#[inline] move |v| f.lens_mut(v) )
    }

    #[inline]
    pub fn fork(&self) -> Self {
        self.refc()
    }
}

/*impl<'a,T,U,F> Clone for SMA<'a,T,U,F> where F: SMALens<T,U> {
    fn clone(&self) -> Self {
        Self{
            inner: self.inner.refc(),
            f: self.f.clone(),
            p: PhantomData,
        }
    }
}*/
impl<'a,T,U,F> RefClonable for SMA<'a,T,U,F> where F: SMALens<T,U> {
    fn refc(&self) -> Self {
        Self{
            inner: self.inner.refc(),
            f: self.f.clone(),
            p: PhantomData,
        }
    }
}

pub trait SMALens<T,U>: Clone {
    fn lens_mut<'a>(&self, v: &'a mut T) -> &'a mut U;
}

impl<T,U,F> SMALens<T,U> for F where F: Fn(&mut T) -> &mut U + Clone {
    #[inline]
    fn lens_mut<'a>(&self, v: &'a mut T) -> &'a mut U {
        self(v)
    }
}

impl<T> SMALens<T,T> for () {
    #[inline]
    fn lens_mut<'a>(&self, v: &'a mut T) -> &'a mut T {
        v
    }
}

impl<'a,E,T,U,V,F> AtomState<E,V> for SMA<'a,T,U,F> where 
    E: Env,
    U: AtomState<E,V>,
    F: SMALens<T,U>
{
    fn get_direct(&self) -> Result<V,()> {
        self.borrow_mut().get_direct()
    }
    fn get(&self, c: &mut E::Context) -> V {
        self.borrow_mut().get(c)
    }
}

impl<'a,E,T,U,V,F> AtomStateMut<E,V> for SMA<'a,T,U,F> where 
    E: Env,
    U: AtomStateMut<E,V>,
    F: SMALens<T,U>
{
    fn set_direct(&mut self, v: V) -> Result<(),()> {
        self.borrow_mut().set_direct(v)
    }
    fn set(&mut self, v: V, c: &mut E::Context) {
        self.borrow_mut().set(v,c)
    }
}
