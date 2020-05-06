//! Event container and variants
use crate::event::key::Key;
use super::*;
use std::any::Any;

pub mod variants;
pub mod key;

pub mod imp;
pub mod dyn_evt;

pub mod variant;

/// an Event holds one of the support Variant and can be downcasted to a specific Variant
pub trait Event<E>: Sized + Clone where E: Env, E::Backend: Backend<E,Event=Self> {
    type Dest: Destination;
    type Key: Key;

    fn filter(self, bounds: &Bounds) -> Option<Self>;
    #[inline]
    fn filter_cloned(&self, bounds: &Bounds) -> Option<Self> {
        self.clone().filter(bounds)
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

    fn _root_only(&self) -> bool;

    fn _debug_type_name(&self);
}

pub trait Destination: Clone + Sized {
    /// send the event to the root widget
    const ROOT: Self;
    /// send the widget to the currently focused widget
    const FOCUSED: Self;
    /// send the event to the currently hovered widget
    const HOVERED: Self;
    /// distribution of such event is invalid
    const INVALID: Self;

    #[inline]
    fn default() -> Self {
        Self::ROOT
    }
}
