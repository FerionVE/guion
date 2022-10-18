use std::borrow::Borrow;

use super::{AsCachorOf, AsCachor};

#[derive(Clone,Copy)]
pub struct CachorToOwned;
#[derive(Clone,Copy)]
pub struct CachorToOwnedRef;
#[derive(Clone,Copy)]
#[repr(transparent)]
pub struct CachorToOwnedV<T>(pub T) where T: ToOwned + PartialEq + ?Sized, T::Owned: Clone + PartialEq + 'static;
#[repr(transparent)]
pub struct CachorToOwnedVRef<'a,T>(pub &'a T) where T: ToOwned + PartialEq + ?Sized, T::Owned: Clone + PartialEq + 'static;

impl<'a,T> Clone for CachorToOwnedVRef<'a,T> where T: ToOwned + PartialEq + ?Sized, T::Owned: Clone + PartialEq + 'static {
    #[inline]
    fn clone(&self) -> Self {
        Self(&*self.0)
    }
}
impl<'a,T> Copy for CachorToOwnedVRef<'a,T> where T: ToOwned + PartialEq + ?Sized, T::Owned: Clone + PartialEq + 'static {}

impl<T,E> AsCachorOf<&T,E> for CachorToOwnedRef where CachorToOwned: AsCachorOf<T,E>, T: ToOwned + PartialEq + ?Sized, T::Owned: Clone + PartialEq + 'static {
    type Cachor = <CachorToOwned as AsCachorOf<T,E>>::Cachor;

    #[must_use]
    #[inline]
    fn cachor(&self, v: &&T) -> Self::Cachor {
        <CachorToOwned as AsCachorOf<T,E>>::cachor(&CachorToOwned, v)
    }

    #[must_use]
    #[inline]
    fn valid(&self, v: &&T, cachored: &Self::Cachor) -> bool {
        <CachorToOwned as AsCachorOf<T,E>>::valid(&CachorToOwned, v, cachored)
    }
}
impl<T,U,E> AsCachorOf<U,E> for CachorToOwnedV<T> where Self: AsCachor<E>, T: ToOwned + PartialEq + ?Sized, T::Owned: Clone + PartialEq + 'static {
    type Cachor = <Self as AsCachor<E>>::Cachor;

    #[must_use]
    #[inline]
    fn cachor(&self, _: &U) -> Self::Cachor {
        <Self as AsCachor<E>>::cachor(self)
    }

    #[must_use]
    #[inline]
    fn valid(&self, _: &U, cachored: &Self::Cachor) -> bool {
        <Self as AsCachor<E>>::valid(self, cachored)
    }
}
impl<T,U,E> AsCachorOf<U,E> for CachorToOwnedVRef<'_,T> where Self: AsCachor<E>, T: ToOwned + PartialEq + ?Sized, T::Owned: Clone + PartialEq + 'static {
    type Cachor = <Self as AsCachor<E>>::Cachor;

    #[must_use]
    #[inline]
    fn cachor(&self, _: &U) -> Self::Cachor {
        <Self as AsCachor<E>>::cachor(self)
    }

    #[must_use]
    #[inline]
    fn valid(&self, _: &U, cachored: &Self::Cachor) -> bool {
        <Self as AsCachor<E>>::valid(self, cachored)
    }
}

impl<T,E> AsCachorOf<T,E> for CachorToOwned where T: ToOwned + PartialEq + ?Sized, T::Owned: Clone + PartialEq + 'static {
    type Cachor = T::Owned;

    #[must_use]
    #[inline]
    fn cachor(&self, v: &T) -> Self::Cachor {
        v.to_owned()
    }

    #[must_use]
    #[inline]
    fn valid(&self, v: &T, cachored: &Self::Cachor) -> bool {
        v == cachored.borrow()
    }
}
impl<T,E> AsCachor<E> for CachorToOwnedV<T> where T: ToOwned + PartialEq + ?Sized, T::Owned: Clone + PartialEq + 'static {
    type Cachor = T::Owned;

    #[must_use]
    #[inline]
    fn cachor(&self) -> Self::Cachor {
        self.0.to_owned()
    }

    #[must_use]
    #[inline]
    fn valid(&self, cachored: &Self::Cachor) -> bool {
        self.0 == *cachored.borrow()
    }
}
impl<T,E> AsCachor<E> for CachorToOwnedVRef<'_,T> where T: ToOwned + PartialEq + ?Sized, T::Owned: Clone + PartialEq + 'static {
    type Cachor = T::Owned;

    #[must_use]
    #[inline]
    fn cachor(&self) -> Self::Cachor {
        self.0.to_owned()
    }

    #[must_use]
    #[inline]
    fn valid(&self, cachored: &Self::Cachor) -> bool {
        *self.0 == *cachored.borrow()
    }
}
