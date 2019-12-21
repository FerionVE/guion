use crate::core::event::imp::VariantDerive;
use std::any::Any;
use crate::core::util::bounds::Offset;
use crate::core::*;
use util::bounds::Bounds;
use ctx::*;

pub mod variants;
pub mod key;

pub mod imp;
pub mod dyn_evt;

/// Use is() for querying as a specific variant
pub trait Event<E>: Sized + Clone where E: Env<Event=Self> {
    fn filter(self, subbounds: &Bounds) -> Option<Self>;
    #[inline]
    fn filter_cloned(&self, subbounds: &Bounds) -> Option<Self> {
        self.clone().filter(subbounds)
    }
    /// True if container widgets should sent this to only one widget  
    fn consuming(&self) -> bool;
    /// Where there Event should be initially injected into the context
    fn destination(&self) -> E::EventDest;
    /// Create the event from a variant. returns empty event if variant is not supported
    fn from<V: Variant<E>>(v: V) -> Self where Self: VariantSupport<V,E>;
    /// Try to cast the Event as a specific variant.  
    /// Use this for filtering and reading events  
    fn is<V: Variant<E>>(&self) -> Option<V> where Self: VariantSupport<V,E>;
}

pub trait VariantSupport<V,E>: Event<E> where E: Env<Event=Self>, V: Variant<E> {
    fn from_variant(v: &V) -> Self;
    fn to_variant(&self) -> Self;
}

pub trait Variant<E>: VariantDerive<E> where E: Env {
    #[inline]
    fn pos(&self) -> Option<&Offset> {
        None
    }
    #[inline]
    fn filter(&self, subbounds: &Bounds) -> bool {
        self.pos().map_or(true, |p| p.is_inside(subbounds) )
    }
    /// true 
    #[inline]
    fn consuming(&self) -> bool {
        false
    }
    #[inline]
    fn destination(&self) -> E::EventDest {
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