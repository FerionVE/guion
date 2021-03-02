//! RefCell-based Shared Mutable Access helper for mutable immediate widgets e.g. when multiple parts need to mutably reference to the same thing
use crate::widgets::util::{caption::Caption, caption::CaptionMut, state::{AtomState, AtomStateMut}};

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

    fn mutate(&mut self) -> Result<&mut dyn AtomStateMut<E,V>,GuionError<E>> {
        todo!()
    }

    fn try_set(&mut self, v: V, c: &mut E::Context) -> Result<(),GuionError<E>> {
        self.borrow_mut().try_set(v,c)
    }

    fn try_set_direct(&mut self, v: V) -> Result<(),GuionError<E>> {
        self.borrow_mut().try_set_direct(v)
    }

    fn ref_box<'s>(&'s self) -> Box<dyn AtomState<E,V>+'_> where Self: 's {
        Box::new(self)
    }

    fn mut_box<'s>(&'s mut self) -> Box<dyn AtomState<E,V>+'_> where Self: 's {
        Box::new(self)
    }
}
impl<'a,E,T,U,V,F> AtomState<E,V> for &SMA<'a,T,U,F> where 
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

    fn mutate(&mut self) -> Result<&mut dyn AtomStateMut<E,V>,GuionError<E>> {
        todo!()
    }

    fn try_set(&mut self, v: V, c: &mut E::Context) -> Result<(),GuionError<E>> {
        self.borrow_mut().try_set(v,c)
    }

    fn try_set_direct(&mut self, v: V) -> Result<(),GuionError<E>> {
        self.borrow_mut().try_set_direct(v)
    }

    fn ref_box<'s>(&'s self) -> Box<dyn AtomState<E,V>+'_> where Self: 's {
        Box::new(*self)
    }

    fn mut_box<'s>(&'s mut self) -> Box<dyn AtomState<E,V>+'_> where Self: 's {
        Box::new(*self)
    }
}
impl<'a,E,T,U,V,F> AtomState<E,V> for &mut SMA<'a,T,U,F> where 
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

    fn mutate(&mut self) -> Result<&mut dyn AtomStateMut<E,V>,GuionError<E>> {
        todo!()
    }

    fn try_set(&mut self, v: V, c: &mut E::Context) -> Result<(),GuionError<E>> {
        self.borrow_mut().try_set(v,c)
    }

    fn try_set_direct(&mut self, v: V) -> Result<(),GuionError<E>> {
        self.borrow_mut().try_set_direct(v)
    }

    fn ref_box<'s>(&'s self) -> Box<dyn AtomState<E,V>+'_> where Self: 's {
        Box::new(*self)
    }

    fn mut_box<'s>(&'s mut self) -> Box<dyn AtomState<E,V>+'_> where Self: 's {
        Box::new(*self)
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

impl<'w,'a,E,T,U,F> Caption<E> for SMA<'a,T,U,F> where 
    E: Env,
    U: Caption<E>+'w,
    F: SMALens<T,U>
{
    fn caption(&self) -> std::borrow::Cow<str> {
        let g = self.borrow_mut();
        let c = g.caption();
        std::borrow::Cow::Owned( c.into_owned() )
    }
    fn len(&self) -> usize {
        self.borrow_mut().len()
    }
}

impl<'w,'a,E,T,U,F> CaptionMut<E> for SMA<'a,T,U,F> where 
    E: Env,
    U: CaptionMut<E>+'w,
    F: SMALens<T,U>
{
    fn push(&mut self, off: usize, s: &str) {
        self.borrow_mut().push(off,s)
    }
    fn pop_left(&mut self, off: usize, n: usize) {
        self.borrow_mut().pop_left(off,n)
    }
    fn replace(&mut self, s: &str) {
        self.borrow_mut().replace(s)
    }
}
