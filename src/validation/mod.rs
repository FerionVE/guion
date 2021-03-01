use super::*;
use std::sync::Arc;

pub mod imp;
pub mod validated;

/// Trait for querying/updating revision on trait data
pub trait Validation<E> {
    // As validation is generally used for caching (e.g. generated glyphs), this type is cached next to the cached generated data
    //type Cached: Clone + Sized + 'static;

    fn valid(&self, v: &dyn Any) -> bool;
}

pub trait ValidationMut<E>: Validation<E> {
    /// Called if e.g. dependent data is just generated, marks this data as valid, and returns side data to cache along with the dependend generated data.
    /// Impl note: this could be called more than once on the same data
    fn validate(&mut self) -> Arc<dyn Any>;
}

unsafe impl<E> Statize<E> for dyn Validation<E> where E: 'static {
    type Statur = dyn Validation<E>;
}
unsafe impl<E> Statize<E> for dyn ValidationMut<E> where E: 'static {
    type Statur = dyn ValidationMut<E>;
}

traitcast_for!(Validation<E>;ValidationMut<E>);
