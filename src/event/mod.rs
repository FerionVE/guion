//! [`Event`] container and variants

use std::fmt::Debug;

use crate::env::Env;
use crate::util::bounds::Bounds;

use self::key::Key;
use self::variant::{Variant, VariantSupport};

pub mod imp;
pub mod key;
pub mod key_combo;
pub mod standard;
pub mod variant;

/// an Event holds one of the support [`Variant`] and can be downcasted to a specific Variant
pub trait Event<E>: Sized + Clone + Debug where E: Env {
    type Dest: Destination;
    type Key: Key;
    
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

    #[inline]
    fn in_bounds(&self, _: &Bounds) -> bool {
        true
    }

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

pub type EventResp = bool;
