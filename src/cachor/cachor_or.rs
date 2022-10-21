use super::{AsCachorOf, AsCachor};

#[derive(Clone,Copy)]
#[repr(transparent)]
pub struct CachorOr<T>(pub T);
#[repr(transparent)]
pub struct CachorOrRef<'a,T>(pub &'a T) where T: ?Sized;

impl<'a,T> Clone for CachorOrRef<'a,T> where T: ?Sized {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<'a,T> Copy for CachorOrRef<'a,T> where T: ?Sized {}

impl<T,U,E> AsCachorOf<U,E> for CachorOr<T> where T: AsCachor<E>, U: AsCachor<E> + ?Sized {
    type Cachor = CachorOrCachor<T::Cachor,U::Cachor>;

    #[inline]
    fn cachor(&self, v: &U) -> Self::Cachor {
        CachorOrCachor(self.0.cachor(),v.cachor())
    }

    #[inline]
    fn valid(&self, v: &U, cachored: &Self::Cachor) -> bool {
        self.0.valid(&cachored.0) || v.valid(&cachored.1)
    }
}

impl<T,U,E> AsCachorOf<U,E> for CachorOrRef<'_,T> where T: AsCachor<E> + ?Sized, U: AsCachor<E> + ?Sized {
    type Cachor = CachorOrCachor<T::Cachor,U::Cachor>;

    #[inline]
    fn cachor(&self, v: &U) -> Self::Cachor {
        CachorOrCachor(self.0.cachor(),v.cachor())
    }

    #[inline]
    fn valid(&self, v: &U, cachored: &Self::Cachor) -> bool {
        self.0.valid(&cachored.0) || v.valid(&cachored.1)
    }
}

#[doc(hidden)]
#[derive(Copy,Clone)]
pub struct CachorOrCachor<A,B>(A,B) where A: PartialEq, B: PartialEq;

impl<A,B> PartialEq for CachorOrCachor<A,B> where A: PartialEq, B: PartialEq {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 || self.1 == other.1
    }
}
