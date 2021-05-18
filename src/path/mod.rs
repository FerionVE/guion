//! A [`Path`](WidgetPath) contains information, like [`WidgetID`], which can denote a location of a [`Widget`]
use std::{slice::SliceIndex, ops::RangeBounds, fmt::Debug};
use qwutils::RefClonable;
use super::*;

pub mod sub;
pub use sub::*;

pub mod standard;

/// A WidgetPath contains information to [resolve](Widget::TODOGuionBookResolveGuide) to a specific [`Widget`] in a [widget tree](Widgets)
pub trait WidgetPath<E>:
    //AsWidget<'static,E> + //TODO fix the generic AsWidget impl for StdPath
    Into<E::WidgetPath> +
    From<E::WidgetPath> +
    RefClonable +
    Clone +
    Sized +
    Debug +
    'static
where E: Env {


    // TODO rename to attach_path
    fn attach_subpath(&mut self, sub: &Self);
    #[inline]
    fn attached_subpath(mut self, sub: &Self) -> Self {
        self.attach_subpath(sub);
        self
    }

    // Returns the subpath which when attached to the prefix yields a path with identical target widget and resolve route
    fn strip_prefix(&self, prefix: &Self) -> Result<Self,()>; //TODO GuionError

    /// Does the sub path from the parent path resolve to or through the specific child widget of the parent widget?
    // returns None only of sub_path wouldn't resolve to or through the given child widget
    /// 
    /// `parent_path`: Absolute path of the current parent widget  
    /// `child`: Child widget of parent widget to which the sub path probably resolves to/through  
    /// `sub_path`: Relative sub path to which widget should be attempted to resolve  
    fn resolves_thru_child_id(child: E::WidgetID, sub_path: &Self) -> Option<ResolvesThruResult<E>>;
    fn resolves_thru_child_path(child_path: &Self, sub_path: &Self) -> Option<ResolvesThruResult<E>>;

    fn for_child_widget_id(&self, child: E::WidgetID) -> Self;
    /// `reduce': as 'child_path' is a absolute path and can resolve to the widget itself, the path with the child can be "reduced" to just returning 'child_path'. This reduction should only be done if 'reduce'=true
    fn for_child_widget_path(&self, child_path: &Self, reduce: bool) -> Self;


    // fn tip(&self) -> Option<&Self::SubPath>;
    /// Returns the targeted widget ID if available
    /// 
    /// NOTE this function is implemented optionally, so it may never return ID, even if possible
    fn _dest_widget(&self) -> Option<E::WidgetID> {
        None
    }
    #[deprecated]
    fn exact_eq(&self, o: &Self) -> bool;

    /// If self and o would resolve to the same widget
    fn dest_eq(&self, o: &Self) -> bool;

    /// The path which would resolve to the parent widget
    /// 
    /// Returns None only if the path is empty
    fn parent(&self) -> Option<Self>;

    /// If the path is empty e.g. doesn't resolve further
    fn is_empty(&self) -> bool;

    fn empty() -> Self;
}

pub struct ResolvesThruResult<E> where E: Env {
    /// The sub path inside the current child widget which resolves further
    pub sub_path: E::WidgetPath,
}
