mod imp;
mod direct;
mod to_owned;
mod ptr_eq;

pub trait AsCachor<E> {
    type Cachor: Clone + PartialEq + 'static;

    #[must_use]
    fn cachor(&self) -> Self::Cachor;

    #[must_use]
    #[inline]
    fn valid(&self, cachored: &Self::Cachor) -> bool {
        self.cachor() == *cachored
    }
}

/// proxy for AsCachor
pub trait AsCachorOf<T,E> where T: ?Sized {
    type Cachor: Clone + PartialEq + 'static;

    #[must_use]
    fn cachor(&self, v: &T) -> Self::Cachor;

    #[must_use]
    #[inline]
    fn valid(&self, v: &T, cachored: &Self::Cachor) -> bool {
        self.cachor(v) == *cachored
    }
}

#[derive(Clone,Copy)]
#[repr(transparent)]
pub struct CachorV<T>(pub T) where T: ?Sized;

impl<T,U,E> AsCachorOf<U,E> for CachorV<T> where T: AsCachor<E> + ?Sized, U: ?Sized {
    type Cachor = T::Cachor;

    #[must_use]
    #[inline]
    fn cachor(&self, _: &U) -> Self::Cachor {
        self.0.cachor()
    }

    #[must_use]
    #[inline]
    fn valid(&self, _: &U, cachored: &Self::Cachor) -> bool {
        self.0.valid(cachored)
    }
}

pub use direct::*;
pub use to_owned::*;
pub use ptr_eq::*;
