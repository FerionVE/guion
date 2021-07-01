use crate::util::translate::immu::Immutable;

use super::*;
use std::marker::PhantomData;
use std::sync::Arc;

pub mod imp;
pub mod validated;
pub mod noop;
pub mod mirrored;

/// Trait for querying/updating revision on trait data
pub trait Validator<T,E> where T: ?Sized {
    type Cache: Sized + 'static;

    // As validation is generally used for caching (e.g. generated glyphs), this type is cached next to the cached generated data
    //type Cached: Clone + Sized + 'static;

    fn valid(value: &T, cache: &Self::Cache) -> bool;
    fn validation(value: &T) -> Self::Cache;
}
