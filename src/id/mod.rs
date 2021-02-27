//! Unique indentifier for widgets
use super::*;
use std::fmt::Debug;
use std::hash::Hash;

pub mod standard;


pub trait WidgetID: Clone + PartialEq + Sized + Debug + 'static { //should also implement Eq + Hash + Send
    #[inline]
    fn id_eq<I: WidgetID + 'static>(&self, o: &I) -> bool where Self: 'static {
        Any::downcast_ref::<Self>(o)
            .map_or(false, #[inline] |r| self.eq(r) )
    }

    #[inline]
    fn is_hovered<E: Env<WidgetID=Self>>(&self, c: &E::Context) -> bool where E::Context: CtxStdState<E>, EPressedKey<E>: PressedKey<E> {
        c.state().is_hovered(self)
    }
    #[inline]
    fn is_focused<E: Env<WidgetID=Self>>(&self, c: &E::Context) -> bool where E::Context: CtxStdState<E>, EPressedKey<E>: PressedKey<E> {
        c.state().is_focused(self)
    }
}

pub trait WidgetIDAlloc: WidgetID {
    fn new_id() -> Self where Self: Sized;
}

/*impl WidgetID for Vec<Box<dyn Any>> {
    
}*/
