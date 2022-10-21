mod imp;
mod direct;
mod to_owned;
mod ptr_eq;
mod mut_cell;
mod cachor_or;

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

use std::num::NonZeroU64;

pub use direct::*;
pub use to_owned::*;
pub use ptr_eq::{CachorPtrEq,CachorPtrEqRef,CachorPtrEqV,CachorPtrEqVRef};
pub use mut_cell::*;
pub use cachor_or::{CachorOr,CachorOrRef};

#[derive(Clone,Copy,PartialEq)]
#[repr(transparent)]
pub struct MutCounter {
    mut_counter: NonZeroU64,
}

impl MutCounter {
    #[inline]
    pub fn invalidate(&mut self) {
        self.mut_counter = self.mut_counter.checked_add(1).unwrap();
    }

    #[inline]
    pub fn new() -> Self {
        Default::default()
    }
}

impl Default for MutCounter {
    #[inline]
    fn default() -> Self {
        Self { mut_counter: unsafe { NonZeroU64::new_unchecked(1) } }
    }
}

impl<E> AsCachor<E> for MutCounter {
    type Cachor = NonZeroU64;

    #[inline]
    fn cachor(&self) -> Self::Cachor {
        self.mut_counter
    }
}
