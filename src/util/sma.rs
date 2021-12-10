//! RefCell-based Shared Mutable Access helper for mutable immediate widgets e.g. when multiple parts need to mutably reference to the same thing
use crate::{text::stor::{TextStor, TextStorMut}, validation::{Validation, ValidationMut}, widgets::util::state::{AtomState, AtomStateMut}};

use super::*;
use std::{cell::{RefMut, RefCell}, marker::PhantomData, rc::Rc, sync::Arc};

/// RefCell-based Shared Mutable Access helper for mutable immediate widgets e.g. when multiple parts need to mutably reference to the same thing
///
/// Mutable: RefCell<T>
/// Shared: Rc<RefCell<T>>
/// Lens: Rc<RefCell<T>>, Fn(&mut T) -> &mut V
pub struct SMA<'a,E,T,U,F> where F: SMALens<T,U> {
    inner: Rc<RefCell<&'a mut T>>,
    f: F,
    p: PhantomData<(E,U)>,
}

impl<'a,E,T> SMA<'a,E,T,T,()> {
    #[inline]
    pub fn new(v: &'a mut T) -> Self {
        Self{
            inner: Rc::new(RefCell::new(v)),
            f: (),
            p: PhantomData,
        }
    }
}

impl<'a,E,T,U,F> SMA<'a,E,T,U,F> where F: SMALens<T,U> {
    #[inline]
    pub fn fork_with_lens<V,G>(&self, lens: G) -> SMA<'a,E,T,V,G> where G: Fn(&mut T) -> &mut V + Clone {
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
        RefMut::map(r,#[inline] move |v| f.lens_mut(v) ) //TODO more flexible Fn(&mut T) -> V, requires custom RefMut
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
impl<'a,E,T,U,F> RefClonable for SMA<'a,E,T,U,F> where F: SMALens<T,U> {
    #[inline]
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

impl<'a,E,T,U,V,F> AtomState<E,V> for SMA<'a,E,T,U,F> where 
    E: Env,
    U: AtomState<E,V>,
    F: SMALens<T,U>
{
    #[inline]
    fn get_direct(&self) -> Result<V,()> {
        self.borrow_mut().get_direct()
    }
    #[inline]
    fn get(&self, c: &mut E::Context<'_>) -> V {
        self.borrow_mut().get(c)
    }
}
impl<'a,E,T,U,V,F> AtomStateMut<E,V> for SMA<'a,E,T,U,F> where 
    E: Env,
    U: AtomStateMut<E,V>,
    F: SMALens<T,U>
{
    #[inline]
    fn set_direct(&mut self, v: V) -> Result<(),()> {
        self.borrow_mut().set_direct(v)
    }
    #[inline]
    fn set(&mut self, v: V, c: &mut E::Context<'_>) {
        self.borrow_mut().set(v,c)
    }
}

impl<'w,'a,E,T,U,F> TextStor<E> for SMA<'a,E,T,U,F> where 
    E: Env,
    U: TextStor<E>+'w,
    F: SMALens<T,U>
{
    #[inline]
    fn caption(&self) -> std::borrow::Cow<str> {
        let g = self.borrow_mut();
        let c = g.caption();
        std::borrow::Cow::Owned( c.into_owned() )
    }
    #[inline]
    fn len(&self) -> usize {
        self.borrow_mut().len()
    }
    #[inline]
    fn chars(&self) -> usize {
        self.borrow_mut().chars()
    }
}
impl<'w,'a,E,T,U,F> TextStorMut<E> for SMA<'a,E,T,U,F> where 
    E: Env,
    U: TextStorMut<E>+'w,
    F: SMALens<T,U>
{
    #[inline]
    fn remove_chars(&mut self, range: std::ops::Range<usize>) {
        self.borrow_mut().remove_chars(range)
    }
    #[inline]
    fn remove_chars_old(&mut self, off: usize, n: usize) {
        self.borrow_mut().remove_chars_old(off,n)
    }
    #[inline]
    fn push_chars(&mut self, off: usize, chars: &str) {
        self.borrow_mut().push_chars(off,chars)
    }
    #[inline]
    fn replace(&mut self, s: &str) {
        self.borrow_mut().replace(s)
    }
}

impl<'w,'a,E,T,U,F> Validation<E> for SMA<'a,E,T,U,F> where 
    E: Env,
    U: Validation<E>+'w,
    F: SMALens<T,U>
{
    #[inline]
    fn valid(&self, v: &dyn Any) -> bool {
        self.borrow_mut().valid(v)
    }
    #[inline]
    fn validation(&self) -> Arc<dyn Any> {
        self.borrow_mut().validation()
    }
}
impl<'w,'a,E,T,U,F> ValidationMut<E> for SMA<'a,E,T,U,F> where 
    E: Env,
    U: ValidationMut<E>+'w,
    F: SMALens<T,U>
{
    #[inline]
    fn validate(&mut self) -> Arc<dyn Any> {
        self.borrow_mut().validate()
    }
}
