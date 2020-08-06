use std::marker::PhantomData;
use super::*;
use std::any::Any;

/// Dynamic Event container. Supports all variants.
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
        Any::downcast_ref(self.event._as_any())
            .map(|e: &V| e.clon() )
    }
}

impl<E,K,D> Debug for DynEvent<E,K,D> where E: Env, E::Backend: Backend<E,Event=Self>, D: Destination, K: Key {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.event.fmt(f)
    }
}