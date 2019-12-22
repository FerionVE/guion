use super::*;
use std::any::Any;

pub struct DynEvent<E> where E: Env<Event=Self> {
    pub event: Box<dyn Variant<E>>,
}

impl<E> Clone for DynEvent<E> where E: Env<Event=Self>{
    #[inline]
    fn clone(&self) -> Self {
        Self {
            event: self.event.clone(),
        }
    }
}

impl<E> Event<E> for DynEvent<E> where E: Env<Event=Self> {
    #[inline]
    fn filter(self, subbounds: &Bounds) -> Option<Self> {
        if self.event.filter(subbounds) {
            Some(self)
        }else{
            None
        }
    }
    #[inline]
    fn consuming(&self) -> bool {
        self.event.consuming()
    }
    #[inline]
    fn destination(&self) -> E::EventDest {
        self.event.destination()
    }
    #[inline]
    fn position(&self) -> Option<Offset> {
        self.event.position()
    }
}

impl<V,E> VariantSupport<V,E> for DynEvent<E> where V: Variant<E>, E: Env<Event=Self> {
    #[inline]
    fn from_variant(v: V) -> Self {
        Self {
            event: Box::new(v),
        }
    }
    #[inline]
    fn to_variant(&self) -> Option<V> {
        Any::downcast_ref(self.event._as_any())
            .map(|e: &V| e.clone() )
    }
}
