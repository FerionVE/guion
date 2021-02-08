use selectag::StyleSelectag;

use super::*;

pub mod imp;

pub trait StyleSelector<E>: Clone + Default {
    fn and(&self, s: &Self) -> Self;
}

pub trait StyleSelectorAppend<S,E>: StyleSelector<E> where S: StyleSelectag<E> {
    #[inline]
    fn with(&self, selectag: S) -> Self where Self: Sized {
        let mut s = self.clone();
        s.append(selectag);
        s
    }
    fn append(&mut self, selectag: S);
    #[inline]
    fn from(selectag: S) -> Self where Self: Sized {
        let mut s: Self = Default::default();
        s.append(selectag);
        s
    }
}
