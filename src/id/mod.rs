//! Unique identifier for widgets
use super::*;
use std::fmt::Debug;
use std::hash::Hash;

pub mod standard;

/// Unique ID for [`Widgets`](Widget::id)
/// 
/// WidgetID shall be easily clonable
pub trait WidgetID: Clone + PartialEq + Sized + Debug + 'static { //should also implement Eq + Hash + Send
    #[inline]
    fn id_eq<I: WidgetID + 'static>(&self, o: &I) -> bool where Self: 'static {
        Any::downcast_ref::<Self>(o)
            .map_or(false, #[inline] |r| self.eq(r) )
    }
}

pub trait WidgetIDAlloc: WidgetID {
    fn new_id() -> Self where Self: Sized;
}

/*impl WidgetID for Vec<Box<dyn Any>> {
    
}*/
