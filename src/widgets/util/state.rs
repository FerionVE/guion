//! Traits for state types
use super::*;
use std::borrow::Cow;

/// Simple atomic type state
pub trait AtomState<T> {
    fn get(&self) -> T;
}
/// Simple atomic type state
pub trait AtomStateMut<T>: AtomState<T> {
    fn set(&mut self, v: T);
}

impl<T> AtomState<T> for T where T: Copy {
    fn get(&self) -> T {
        *self
    }
}
impl<T> AtomState<T> for &T where T: Copy {
    fn get(&self) -> T {
        **self
    }
}
impl<T> AtomState<T> for &mut T where T: Copy {
    fn get(&self) -> T {
        **self
    }
}
impl<T> AtomStateMut<T> for &mut T where T: Copy {
    fn set(&mut self, v: T) {
        **self = v;
    }
}
impl<T> AtomStateMut<T> for T where T: Copy {
    fn set(&mut self, v: T) {
        *self = v;
    }
}

impl<T> AtomState<T> for Cow<'_,T> where T: Copy {
    fn get(&self) -> T {
        *self.as_ref()
    }
}
impl<T> AtomStateMut<T> for Cow<'_,T> where T: Copy {
    fn set(&mut self, v: T) {
        *self.to_mut() = v;
    }
}

unsafe impl<T> Statize for dyn AtomState<T> where T: Statize {
    type Statur = dyn AtomState<T::Statur>;
}
unsafe impl<T> Statize for dyn AtomStateMut<T> where T: Statize {
    type Statur = dyn AtomStateMut<T::Statur>;
}

// TODO find a name for this extended AtomState variant
/// Extended AtomState which can access the Context (side data)
pub trait AtomStateX<E,T> where E: Env {
    fn get(&self, c: &mut E::Context) -> T;
}
pub trait AtomStateXMut<E,T>: AtomStateX<E,T> where E: Env {
    fn set(&mut self, v: T, c: &mut E::Context);
}

// TODO make it less error-prone as you probably forget the X in the traitcast fns
impl<E,T,I> AtomStateX<E,T> for I where I: AtomState<T>, T: Copy, E: Env { 
    fn get(&self, _: &mut E::Context) -> T {
        self.get()
    }
}
impl<E,T,I> AtomStateXMut<E,T> for I where I: AtomStateMut<T>, T: Copy, E: Env {
    fn set(&mut self, v: T, _: &mut E::Context) {
        self.set(v)
    }
}

unsafe impl<E,T> Statize for dyn AtomStateX<E,T> where T: Statize, E: Env {
    type Statur = dyn AtomStateX<E,T::Statur>;
}
unsafe impl<E,T> Statize for dyn AtomStateXMut<E,T> where T: Statize, E: Env {
    type Statur = dyn AtomStateXMut<E,T::Statur>;
}