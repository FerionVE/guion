use crate::core::event::key::Key;
use crate::core::*;
use util::bounds::Bounds;
use ctx::*;

pub mod variants;
pub mod key;

pub trait Event<E>: Sized + Clone where E: Env<Event=Self> {
    type K: Key;
    ///split Self into some known cases to handle
    //fn variant(&self) -> EventVariant;

    fn filter(self, subbounds: &Bounds) -> Option<Self>;
    fn filter_cloned(&self, subbounds: &Bounds) -> Option<Self>; 

    fn consuming(&self) -> bool;

    fn empty() -> Self;
    fn from<V: Variant<E>>(v: V) -> Self;

    fn is_root_event(&self) -> bool;
    fn set_root_event(&mut self) -> bool;
    fn with_root_event(&self, b: bool) -> Self;

    fn is<V: Variant<E>>(&self) -> Option<V>;
    fn set<V: Variant<E>>(&mut self, v: Option<V>);
    fn with<V: Variant<E>>(&self, v: V) -> Self;
    fn without<V: Variant<E>>(&self, v: V) -> Self;

    fn try_from<V: Variant<E>>(v: V) -> Result<Self,V>;

    fn try_is<V: Variant<E>>(&self) -> Option<V>;
    fn try_set<V: Variant<E>>(&mut self, v: Option<V>) -> Result<(),V>;
    fn try_with<V: Variant<E>>(&self, v: V) -> Result<Self,()>;
    fn try_without<V: Variant<E>>(&self, v: V) -> Result<Self,()>;
}

pub trait Variant<E>: Clone + 'static {
    
}

/*
 *  pub struct EventImpl {
 *      pub inner: SDLEvent,
 *      pub root: bool,
 *  }
 */