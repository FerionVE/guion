use crate::core::event::key::Key;
use super::*;
use std::any::Any;

pub mod variants;
pub mod key;

pub mod imp;
pub mod dyn_evt;

/// Use is() for querying as a specific variant
pub trait Event<E>: Sized + Clone where E: Env, E::Backend: Backend<E,Event=Self> {
    type Dest: Destination;
    type Key: Key;

    fn filter(self, subbounds: &Bounds) -> Option<Self>;
    #[inline]
    fn filter_cloned(&self, subbounds: &Bounds) -> Option<Self> {
        self.clone().filter(subbounds)
    }
    /// True if container widgets should sent this to only one widget  
    fn consuming(&self) -> bool;
    /// Where there Event should be initially injected into the context
    fn destination(&self) -> Self::Dest;
    /// Create the event from a variant
    #[inline]
    fn from<V: Variant<E>>(v: V) -> Self where Self: VariantSupport<V,E> {
        VariantSupport::<V,E>::from_variant(v)
    }
    /// Try to cast the Event as a specific variant.  
    /// Use this for filtering and reading events  
    #[inline]
    fn is<V: Variant<E>>(&self) -> Option<V> where Self: VariantSupport<V,E> {
        VariantSupport::<V,E>::to_variant(self)
    }

    fn position(&self) -> Option<Offset>;
}

pub trait VariantSupport<V,E>: Event<E> where E: Env, E::Backend: Backend<E,Event=Self>, V: Variant<E> {
    fn from_variant(v: V) -> Self;
    fn to_variant(&self) -> Option<V>;
}

pub trait Variant<E>: VariantDerive<E> where E: Env {
    #[inline]
    fn position(&self) -> Option<Offset> {
        None
    }
    #[inline]
    fn filter(&self, subbounds: &Bounds) -> bool {
        self.position().map_or(true, |p| p.is_inside(subbounds) )
    }

    #[inline]
    fn consuming(&self) -> bool {
        false
    }
    #[inline]
    fn destination(&self) -> EEDest<E> {
        Destination::default()
    }
}

pub trait Destination: Clone + Sized {
    /// send the event to the root widget
    const ROOT: Self;
    /// send the widget to the currently focused widget
    const SELECTED: Self;

    #[inline]
    fn default() -> Self {
        Self::ROOT
    }
}
/*
 *  pub struct EventImpl {
 *      pub inner: SDLEvent,
 *      pub root: bool,
 *  }
 */