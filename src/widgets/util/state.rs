//! Traits for state types
use super::*;
use std::borrow::Cow;
use std::cell::Cell;
use std::cell::RefCell;
use std::marker::PhantomData;

/// Simple atomic type state
pub trait AtomState<E,T> where E: Env {
    #[inline]
    fn get(&self, _: &mut E::Context) -> T {
        self.get_direct().unwrap()
    }
    fn get_direct(&self) -> Result<T,()>;

    #[inline]
    fn on_set<F>(self, f: F) -> AtomStateOnSet<E,Self,F,T> where Self: Sized, F: FnMut(T) {
        AtomStateOnSet(f,PhantomData,self)
    }
}
/// Simple atomic type state
pub trait AtomStateMut<E,T>: AtomState<E,T> where E: Env {
    #[inline]
    fn set(&mut self, v: T, _: &mut E::Context) {
        self.set_direct(v).unwrap()
    }
    fn set_direct(&mut self, v: T) -> Result<(),()>;
}

impl<E,T> AtomState<E,T> for T where T: Clone, E: Env {
    #[inline]
    fn get_direct(&self) -> Result<T,()> {
        Ok(self.clone())
    }
}
impl<E,T> AtomState<E,T> for &T where T: Clone, E: Env {
    #[inline]
    fn get_direct(&self) -> Result<T,()> {
        Ok((**self).clone())
    }
}
impl<E,T> AtomState<E,T> for &mut T where T: Clone, E: Env {
    #[inline]
    fn get_direct(&self) -> Result<T,()> {
        Ok((**self).clone())
    }
}
impl<E,T> AtomStateMut<E,T> for &mut T where T: Clone, E: Env {
    #[inline]
    fn set_direct(&mut self, v: T) -> Result<(),()> {
        **self = v;
        Ok(())
    }
}
impl<E,T> AtomStateMut<E,T> for T where T: Clone, E: Env {
    #[inline]
    fn set_direct(&mut self, v: T) -> Result<(),()> {
        *self = v;
        Ok(())
    }
}

impl<E,T> AtomState<E,T> for Cow<'_,T> where T: Clone, E: Env {
    #[inline]
    fn get_direct(&self) -> Result<T,()> {
        Ok((*self.as_ref()).clone())
    }
}
impl<E,T> AtomStateMut<E,T> for Cow<'_,T> where T: Clone, E: Env {
    #[inline]
    fn set_direct(&mut self, v: T) -> Result<(),()> {
        *self.to_mut() = v;
        Ok(())
    }
}

impl<E,T> AtomState<E,T> for RefCell<T> where T: Clone, E: Env {
    fn get_direct(&self) -> Result<T,()> {
        Ok(self.borrow().clone())
    }
}
impl<E,T> AtomStateMut<E,T> for RefCell<T> where T: Clone, E: Env {
    fn set_direct(&mut self, v: T) -> Result<(),()> {
        *self.get_mut() = v;
        Ok(())
    }
}

impl<E,T> AtomState<E,T> for &RefCell<T> where T: Clone, E: Env {
    fn get_direct(&self) -> Result<T,()> {
        Ok(self.borrow().clone())
    }
}
impl<E,T> AtomStateMut<E,T> for &RefCell<T> where T: Clone, E: Env {
    fn set_direct(&mut self, v: T) -> Result<(),()> {
        *self.borrow_mut() = v;
        Ok(())
    }
}

impl<E,T> AtomState<E,T> for RefCell<&mut T> where T: Clone, E: Env {
    fn get_direct(&self) -> Result<T,()> {
        Ok(self.borrow().clone())
    }
}
impl<E,T> AtomStateMut<E,T> for RefCell<&mut T> where T: Clone, E: Env {
    fn set_direct(&mut self, v: T) -> Result<(),()> {
        **self.get_mut() = v;
        Ok(())
    }
}

impl<E,T> AtomState<E,T> for &RefCell<&mut T> where T: Clone, E: Env {
    fn get_direct(&self) -> Result<T,()> {
        Ok(self.borrow().clone())
    }
}
impl<E,T> AtomStateMut<E,T> for &RefCell<&mut T> where T: Clone, E: Env {
    fn set_direct(&mut self, v: T) -> Result<(),()> {
        **self.borrow_mut() = v;
        Ok(())
    }
}

impl<E,T> AtomState<E,T> for Cell<T> where T: Copy, E: Env {
    fn get_direct(&self) -> Result<T,()> {
        Ok(Cell::get(self))
    }
}
impl<E,T> AtomStateMut<E,T> for Cell<T> where T: Copy, E: Env {
    fn set_direct(&mut self, v: T) -> Result<(),()> {
        Cell::set(self,v);
        Ok(())
    }
}

impl<E,T> AtomState<E,T> for &Cell<T> where T: Copy, E: Env {
    fn get_direct(&self) -> Result<T,()> {
        Ok(Cell::get(self))
    }
}
impl<E,T> AtomStateMut<E,T> for &Cell<T> where T: Copy, E: Env {
    fn set_direct(&mut self, v: T) -> Result<(),()> {
        Cell::set(self,v);
        Ok(())
    }
}

pub struct AtomStateOnSet<E,A: ?Sized,F,T>(F,PhantomData<(T,E)>,A) where E: Env, A: AtomState<E,T>, F: FnMut(T);

impl<E,A,F,T> AtomState<E,T> for AtomStateOnSet<E,A,F,T> where E: Env, A: AtomState<E,T>, F: FnMut(T) {
    #[inline]
    fn get_direct(&self) -> Result<T,()> {
        self.2.get_direct()
    }
    #[inline]
    fn get(&self, c: &mut E::Context) -> T {
        self.2.get(c)
    }
}
impl<E,A,F,T> AtomStateMut<E,T> for AtomStateOnSet<E,A,F,T> where E: Env, A: AtomState<E,T>, F: FnMut(T) {
    #[inline]
    fn set_direct(&mut self, v: T) -> Result<(),()> {
        self.0(v);
        Ok(())
    }
}

impl<E,A,F,T> AtomState<E,T> for &mut AtomStateOnSet<E,A,F,T> where E: Env, A: AtomState<E,T>, F: FnMut(T) {
    #[inline]
    fn get_direct(&self) -> Result<T,()> {
        self.2.get_direct()
    }
    #[inline]
    fn get(&self, c: &mut E::Context) -> T {
        self.2.get(c)
    }
}
impl<E,A,F,T> AtomStateMut<E,T> for &mut AtomStateOnSet<E,A,F,T> where E: Env, A: AtomState<E,T>, F: FnMut(T) {
    #[inline]
    fn set_direct(&mut self, v: T) -> Result<(),()> {
        self.0(v);
        Ok(())
    }
}

impl<E,A,F,T> AtomState<E,T> for &AtomStateOnSet<E,A,F,T> where E: Env, A: AtomState<E,T>, F: FnMut(T) {
    #[inline]
    fn get_direct(&self) -> Result<T,()> {
        self.2.get_direct()
    }
    #[inline]
    fn get(&self, c: &mut E::Context) -> T {
        self.2.get(c)
    }
}

unsafe impl<T,E> Statize<E> for dyn AtomState<E,T> where T: 'static, E: Env {
    type Statur = dyn AtomState<E,T>;
}
unsafe impl<T,E> Statize<E> for dyn AtomStateMut<E,T> where T: 'static, E: Env {
    type Statur = dyn AtomStateMut<E,T>;
}

unsafe impl<'w,T,E> Traitcast<dyn AtomState<E,T>+'w,E> for dyn Widget<E>+'w where E: Env, T: 'static {
    type DestTypeID = dyn AtomState<E,T>;
}
unsafe impl<'w,T,E> TraitcastMut<dyn AtomState<E,T>+'w,E> for dyn WidgetMut<E>+'w where E: Env, T: 'static {
    type DestTypeID = dyn AtomState<E,T>;
}
unsafe impl<'w,T,E> TraitcastMut<dyn AtomStateMut<E,T>+'w,E> for dyn WidgetMut<E>+'w where E: Env, T: 'static {
    type DestTypeID = dyn AtomStateMut<E,T>;
}
