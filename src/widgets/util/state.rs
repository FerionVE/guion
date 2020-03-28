use super::*;
use std::borrow::Cow;

pub trait AtomState<T> {
    fn get(&self) -> T;
}
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

pub trait Mutize<T> {
    type Mutur: AtomStateMut<T> + 'static;
}

impl<T> Mutize<T> for T where T: Statize, T::Statur: AtomStateMut<T>+Sized {
    type Mutur = T::Statur;
}
impl<T> Mutize<T> for &T where T: Statize, &'static mut T::Statur: AtomStateMut<T> {
    type Mutur = &'static mut T::Statur;
}
impl<T> Mutize<T> for &mut T where T: Statize, &'static mut T::Statur: AtomStateMut<T> {
    type Mutur = &'static mut T::Statur;
}
impl<T> Mutize<T> for Cow<'_,T> where T: Statize+Clone, T::Statur: Clone, Cow<'static,T::Statur>: AtomStateMut<T>+Sized {
    type Mutur = Cow<'static,T::Statur>;
}