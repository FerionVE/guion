use crate::core::event::key::Key;
use crate::core::*;
use util::bounds::Bounds;
use ctx::*;

pub mod variants;
pub mod key;

/// Use is() for querying as a specific variant
pub trait Event<E>: Sized + Clone where E: Env<Event=Self> {
    type K: Key;
    ///split Self into some known cases to handle
    //fn variant(&self) -> EventVariant;

    fn filter(self, subbounds: &Bounds) -> Option<Self>;
    fn filter_cloned(&self, subbounds: &Bounds) -> Option<Self>; 
    /// Should this be send only to one widget if it should be sent to multiple  
    /// This is relevant for widget impls  
    fn consuming(&self) -> bool;
    /// Should this event be send to all child widgets  
    /// This is only relevant for ContextHandler  
    #[doc(hidden)]
    fn _broadcast(&self) -> bool;
    /// Create a empty event
    fn empty() -> Self;
    /// Create the event from a variant. returns empty event if variant is not supported
    fn from<V: Variant<E>>(v: V) -> Self;
    /// Only relevant for ContextHandler
    #[doc(hidden)]
    fn _is_root_event(&self) -> bool;
    /// Only relevant for ContextHandler
    #[doc(hidden)]
    fn _set_root_event(&mut self) -> bool;
    /// Only relevant for ContextHandler
    #[doc(hidden)]
    fn _with_root_event(&self, b: bool) -> Self;
    /// Try to cast the Event as a specific variant.  
    /// Use this for filtering and reading events  
    fn is<V: Variant<E>>(&self) -> Option<V>;
    /// Insert or remove a variant.
    /// Adding a variant may replaces previous and removes other variants.  
    /// Removing a variant doesn't affect other contained variants.  
    /// Fails if the event doesn't support the variant.  
    fn set<V: Variant<E>>(&mut self, v: Option<V>) -> Result<(),Option<V>>;
    /// Clone the event with a specific variant included if supported.  
    /// Adding a variant may replaces previous and removes other variants.  
    fn with<V: Variant<E>>(&self, v: V) -> Result<Self,()>;
    /// Clone the event and remove a specific variant.  
    /// Removing a variant doesn't affect other contained variants.  
    fn without<V: Variant<E>>(&self, v: V) -> Result<Self,()>;

    fn soft_from<V: Variant<E>>(v: V) -> Result<Self,V>;

    fn soft_is<V: Variant<E>>(&self) -> Option<V>;
    fn soft_set<V: Variant<E>>(&mut self, v: Option<V>);
    fn soft_with<V: Variant<E>>(&self, v: V) -> Self;
    fn soft_without<V: Variant<E>>(&self, v: V) -> Self;
}

pub trait Variant<E>: Clone + 'static {
    
}

/*
 *  pub struct EventImpl {
 *      pub inner: SDLEvent,
 *      pub root: bool,
 *  }
 */