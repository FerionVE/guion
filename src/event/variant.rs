use std::fmt::Debug;

use crate::aliases::EEDest;
use crate::env::Env;
use crate::util::bounds::Bounds;

use super::{Event, Destination};
use super::imp::VariantDerive;

pub trait VariantSupport<V,E>: Event<E> where E: Env, V: Variant<E> {
    fn from_variant(v: V) -> Self;
    fn to_variant(&self) -> Option<V>;
}

pub trait Variant<E>: VariantDerive<E> + Debug where E: Env { //TODO rename to EventVariant
    #[inline]
    fn in_bounds(&self, _: &Bounds) -> bool {
        true
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
