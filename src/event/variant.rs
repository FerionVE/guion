use super::*;

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
    fn filter(&self, bounds: &Bounds) -> bool {
        self.position().map_or(true, |p| p.is_inside(bounds) )
    }
    // both own_bounds and subbounds are absolute

    #[inline]
    fn consuming(&self) -> bool {
        false
    }
    #[inline]
    fn destination(&self) -> EEDest<E> {
        Destination::default()
    }
    #[inline]
    fn _root_only(&self) -> bool {
        false
    }

    fn _debug_type_name(&self) {
        eprintln!("Evention {}",std::any::type_name::<Self>());
    }
}