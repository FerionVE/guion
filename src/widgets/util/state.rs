//! Traits for state types
use super::*;
use std::borrow::Cow;

/// Simple atomic type state
pub trait AtomState<E,T> where E: Env {
    #[inline]
    fn get(&self, _: &mut E::Context) -> T {
        self.get_direct().unwrap()
    }
    fn get_direct(&self) -> Result<T,()>;
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

unsafe impl<T,E> Statize<E> for dyn AtomState<E,T> where T: StatizeSized<E>, E: Env {
    type Statur = dyn AtomState<E,T::StaturSized>;
}
unsafe impl<T,E> Statize<E> for dyn AtomStateMut<E,T> where T: StatizeSized<E>, E: Env {
    type Statur = dyn AtomStateMut<E,T::StaturSized>;
}
