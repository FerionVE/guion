use std::borrow::Cow;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut, Range};

use crate::cachor::AsCachor;
use crate::env::Env;
use crate::text::stor::{TextStor, TextStorMut};
use crate::widgets::util::state::{AtomState, AtomStateMut};

/// Implements TextStorMut/ValidationMut for immutable and discards mutation
#[repr(transparent)]
pub struct Immutable<E,T: ?Sized,Z: ?Sized>(pub PhantomData<(E,fn()->Z)>,pub T);

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
    fn caption(&self) -> Cow<'_,str> {
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
    fn replace(&mut self, _: Range<usize>, _: &str) {}
}
impl<E,A,Z> TextStorMut<E> for &Immutable<E,A,Z> where A: TextStor<E>, E: Env {
    #[inline]
    fn replace(&mut self, _: Range<usize>, _: &str) {}
}

impl<E,A,Z> AsCachor<E> for Immutable<E,A,Z> where A: AsCachor<E> {
    type Cachor = A::Cachor;
    #[inline]
    fn cachor(&self) -> Self::Cachor {
        (**self).cachor()
    }
    #[inline]
    fn valid(&self, cachored: &Self::Cachor) -> bool {
        (**self).valid(cachored)
    }
}

impl<E,A,T> AtomState<E,T> for Immutable<E,A,T> where E: Env, A: AtomState<E,T> {
    #[inline]
    fn get_direct(&self) -> Result<T,()> {
        (**self).get_direct()
    }
    #[inline]
    fn get(&self, c: &mut E::Context<'_>) -> T {
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
    fn get(&self, c: &mut E::Context<'_>) -> T {
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
    fn get(&self, c: &mut E::Context<'_>) -> T {
        (**self).get(c)
    }
}
impl<E,A,T> AtomStateMut<E,T> for &Immutable<E,A,T> where E: Env, A: AtomState<E,T> {
    #[inline]
    fn set_direct(&mut self, _: T) -> Result<(),()> {
        Ok(()) //TODO discard or fail?
    }
}
