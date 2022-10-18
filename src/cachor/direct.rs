use super::{AsCachorOf, AsCachor};

#[derive(Clone,Copy)]
pub struct CachorDirect;
#[derive(Clone,Copy)]
#[repr(transparent)]
pub struct CachorDirectV<T>(pub T) where T: Clone + PartialEq + 'static + ?Sized;
#[repr(transparent)]
pub struct CachorDirectVRef<'a,T>(pub &'a T) where T: Clone + PartialEq + 'static + ?Sized;

impl<'a,T> Clone for CachorDirectVRef<'a,T> where T: Clone + PartialEq + 'static + ?Sized {
    #[inline]
    fn clone(&self) -> Self {
        Self(&*self.0)
    }
}
impl<'a,T> Copy for CachorDirectVRef<'a,T> where T: Clone + PartialEq + 'static + ?Sized {}

impl<T,E> AsCachorOf<T,E> for CachorDirect where T: Clone + PartialEq + 'static + ?Sized {
    type Cachor = T;

    #[must_use]
    #[inline]
    fn cachor(&self, v: &T) -> Self::Cachor {
        v.clone()
    }

    #[must_use]
    #[inline]
    fn valid(&self, v: &T, cachored: &Self::Cachor) -> bool {
        *v == *cachored
    }
}
impl<T,U,E> AsCachorOf<U,E> for CachorDirectV<T> where T: Clone + PartialEq + 'static + ?Sized, U: ?Sized {
    type Cachor = T;

    #[must_use]
    #[inline]
    fn cachor(&self, _: &U) -> Self::Cachor {
        self.0.clone()
    }

    #[must_use]
    #[inline]
    fn valid(&self, _: &U, cachored: &Self::Cachor) -> bool {
        self.0 == *cachored
    }
}
impl<T,U,E> AsCachorOf<U,E> for CachorDirectVRef<'_,T> where T: Clone + PartialEq + 'static + ?Sized, U: ?Sized {
    type Cachor = T;

    #[must_use]
    #[inline]
    fn cachor(&self, _: &U) -> Self::Cachor {
        self.0.clone()
    }

    #[must_use]
    #[inline]
    fn valid(&self, _: &U, cachored: &Self::Cachor) -> bool {
        *self.0 == *cachored
    }
}
impl<T,E> AsCachor<E> for CachorDirectV<T> where T: Clone + PartialEq + 'static + ?Sized {
    type Cachor = T;

    #[must_use]
    #[inline]
    fn cachor(&self) -> Self::Cachor {
        self.0.clone()
    }

    #[must_use]
    #[inline]
    fn valid(&self, cachored: &Self::Cachor) -> bool {
        self.0 == *cachored
    }
}
impl<T,E> AsCachor<E> for CachorDirectVRef<'_,T> where T: Clone + PartialEq + 'static + ?Sized {
    type Cachor = T;

    #[must_use]
    #[inline]
    fn cachor(&self) -> Self::Cachor {
        self.0.clone()
    }

    #[must_use]
    #[inline]
    fn valid(&self, cachored: &Self::Cachor) -> bool {
        *self.0 == *cachored
    }
}
