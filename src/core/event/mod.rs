use crate::core::util::bounds::Bounds;

pub mod variants;

pub trait Event: Sized + Clone {
    ///split Self into some known cases to handle
    //fn variant(&self) -> EventVariant;

    fn filter(self, subbounds: &Bounds) -> Option<Self>;
    fn filter_cloned(&self, subbounds: &Bounds) -> Option<Self>; 

    fn consuming(&self) -> bool;

    fn empty() -> Self;
    fn from<V: Variant>(v: V) -> Self;

    fn is_root_event(&self) -> bool;
    fn set_root_event(&mut self) -> bool;
    fn with_root_event(&self, b: bool) -> Self;

    fn is<V: Variant>(&self) -> Option<V>;
    fn set<V: Variant>(&mut self, v: Option<V>);
    fn with<V: Variant>(&self, v: V) -> Self;
    fn without<V: Variant>(&self, v: V) -> Self;

    fn try_from<V: Variant>(v: V) -> Result<Self,V>;

    fn try_is<V: Variant>(&self) -> Option<V>;
    fn try_set<V: Variant>(&mut self, v: Option<V>) -> Result<(),V>;
    fn try_with<V: Variant>(&self, v: V) -> Result<Self,()>;
    fn try_without<V: Variant>(&self, v: V) -> Result<Self,()>;
}

pub trait Variant: Clone + 'static {
    
}

/*
 *  pub struct EventImpl {
 *      pub inner: SDLEvent,
 *      pub root: bool,
 *  }
 */