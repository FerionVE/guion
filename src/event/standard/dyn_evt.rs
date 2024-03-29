use std::fmt::Debug;
use std::marker::PhantomData;

use crate::backend::Backend;
use crate::env::Env;
use crate::event::{Destination, Event};
use crate::event::key::Key;
use crate::event::variant::{Variant, VariantSupport};
use crate::util::bounds::Bounds;

/// Dynamic [`Event`] container. Supports all variants.
pub struct DynEvent<E,K,D> where E: Env, E::Backend: Backend<E,Event=Self>, D: Destination, K: Key {
    pub event: Box<dyn Variant<E>>,
    _m: PhantomData<(K,D)>,
}

impl<E,K,D> Clone for DynEvent<E,K,D> where E: Env, E::Backend: Backend<E,Event=Self>, D: Destination, K: Key {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            event: self.event.clone(),
            _m: PhantomData,
        }
    }
}

impl<E,K,D> Event<E> for DynEvent<E,K,D> where E: Env, E::Backend: Backend<E,Event=Self>, D: Destination, K: Key {
    type Dest = D;
    type Key = K;

    #[inline]
    fn consuming(&self) -> bool {
        self.event.consuming()
    }
    #[inline]
    fn destination(&self) -> Self::Dest {
        self.event.destination()
    }
    #[inline]
    fn in_bounds(&self, b: &Bounds) -> bool {
        self.event.in_bounds(b)
    }
    #[inline]
    fn _root_only(&self) -> bool {
        self.event._root_only()
    }

    fn _debug_type_name(&self) {
        self.event._debug_type_name();
    }
}

impl<V,E,K,D> VariantSupport<V,E> for DynEvent<E,K,D> where V: Variant<E>, E: Env, E::Backend: Backend<E,Event=Self>, D: Destination, K: Key {
    #[inline]
    fn from_variant(v: V) -> Self {
        Self {
            event: Box::new(v),
            _m: PhantomData,
        }
    }
    #[inline]
    fn to_variant(&self) -> Option<V> {
        self.event._as_any().downcast_ref()
            .map(#[inline] |e: &V| e.clon() )
    }
}

impl<E,K,D> Debug for DynEvent<E,K,D> where E: Env, E::Backend: Backend<E,Event=Self>, D: Destination, K: Key {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.event.fmt(f)
    }
}
