use std::borrow::Cow;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut, Range};

use crate::text::stor::{TextStor, TextStorMut};
use crate::widgets::util::state::AtomState;
use crate::*;
use crate::widgets::util::state::AtomStateMut;

/// Implements TextStorMut/ValidationMut for immutable and discards mutation
#[repr(transparent)]
pub struct Immutable<E,T: ?Sized,Z: ?Sized>(pub PhantomData<(E,Z)>,pub T);

impl<E,T,Z> Deref for Immutable<E,T,Z> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.1
    }
}
impl<E,T,Z> DerefMut for Immutable<E,T,Z> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.1
    }
}

//TODO AtomState/AtomStateMut

impl<E,A,Z> TextStor<E> for Immutable<E,A,Z> where A: TextStor<E> {
    #[inline]
    fn caption<'s>(&'s self) -> Cow<'s,str> {
        (**self).caption()
    }
    #[inline]
    fn chars(&self) -> usize {
        (**self).chars()
    }
    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }
}
impl<E,A,Z> TextStorMut<E> for Immutable<E,A,Z> where A: TextStor<E> {
    #[inline]
    fn remove_chars(&mut self, range: Range<usize>) {}
    #[inline]
    fn push_chars(&mut self, off: usize, chars: &str) {}
    #[inline]
    fn remove_chars_old(&mut self, off: usize, n: usize) {}
    #[inline]
    fn replace(&mut self, s: &str) {}
}
impl<E,A,Z> TextStorMut<E> for &Immutable<E,A,Z> where A: TextStor<E>, E: Env {
    #[inline]
    fn remove_chars(&mut self, range: Range<usize>) {}
    #[inline]
    fn push_chars(&mut self, off: usize, chars: &str) {}
    #[inline]
    fn remove_chars_old(&mut self, off: usize, n: usize) {}
    #[inline]
    fn replace(&mut self, s: &str) {}
}

impl<E,A,T> AtomState<E,T> for Immutable<E,A,T> where E: Env, A: AtomState<E,T> {
    #[inline]
    fn get_direct(&self) -> Result<T,()> {
        (**self).get_direct()
    }
    #[inline]
    fn get(&self, c: &mut E::Context) -> T {
        (**self).get(c)
    }
}
impl<E,A,T> AtomStateMut<E,T> for Immutable<E,A,T> where E: Env, A: AtomState<E,T> {
    #[inline]
    fn set_direct(&mut self, _: T) -> Result<(),()> {
        Ok(()) //TODO discard or fail?
    }
}

impl<E,A,T> AtomState<E,T> for &mut Immutable<E,A,T> where E: Env, A: AtomState<E,T> {
    #[inline]
    fn get_direct(&self) -> Result<T,()> {
        (**self).get_direct()
    }
    #[inline]
    fn get(&self, c: &mut E::Context) -> T {
        (**self).get(c)
    }
}
impl<E,A,T> AtomStateMut<E,T> for &mut Immutable<E,A,T> where E: Env, A: AtomState<E,T> {
    #[inline]
    fn set_direct(&mut self, _: T) -> Result<(),()> {
        Ok(()) //TODO discard or fail?
    }
}

impl<E,A,T> AtomState<E,T> for &Immutable<E,A,T> where E: Env, A: AtomState<E,T> {
    #[inline]
    fn get_direct(&self) -> Result<T,()> {
        (**self).get_direct()
    }
    #[inline]
    fn get(&self, c: &mut E::Context) -> T {
        (**self).get(c)
    }
}
impl<E,A,T> AtomStateMut<E,T> for &Immutable<E,A,T> where E: Env, A: AtomState<E,T> {
    #[inline]
    fn set_direct(&mut self, _: T) -> Result<(),()> {
        Ok(()) //TODO discard or fail?
    }
}
