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