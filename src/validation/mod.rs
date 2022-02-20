use crate::util::translate::immu::Immutable;

use super::*;
use std::marker::PhantomData;
use std::sync::Arc;

pub mod imp;
pub mod validated;

/// Trait for querying/updating revision on trait data
pub trait Validation<E> {
    // As validation is generally used for caching (e.g. generated glyphs), this type is cached next to the cached generated data
    //type Cached: Clone + Sized + 'static;

    fn valid(&self, v: &dyn Any) -> bool;
    fn validation(&self) -> Arc<dyn Any>;

    #[inline]
    fn immutable(self) -> Immutable<E,Self,()> where Self: Sized {
        Immutable(PhantomData,self)
    }
}

pub trait ValidationMut<E>: Validation<E> {
    /// Called if e.g. dependent data is just generated, marks this data as valid, and returns side data to cache along with the dependent generated data.
    /// Impl note: this could be called more than once on the same data
    fn validate(&mut self) -> Arc<dyn Any>;
}

traitcast_for!(Validation<E>;ValidationMut<E>);
