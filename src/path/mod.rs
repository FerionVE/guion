//! A path contains information, like widget id, which can denote a location of a widget
use std::{slice::SliceIndex, ops::RangeBounds, fmt::Debug};
use qwutils::RefClonable;
use super::*;

pub mod sub;
pub use sub::*;

pub mod standard;

/// A WidgetPath contains information to resolve to a specific Widget in a widget tree
pub trait WidgetPath<E>:
    //AsWidget<'static,E> + //TODO fix the generic AsWidget impl for StdPath
    Into<E::WidgetPath> +
    From<E::WidgetPath> +
    RefClonable +
    Clone +
    Sized +
    Debug +
    Send +
    Sync +
    'static
where E: Env {


    // TODO rename to attach_path
    fn attach_subpath(&mut self, sub: &Self);
    #[inline]
    fn attached_subpath(mut self, sub: &Self) -> Self {
        self.attach_subpath(sub);
        self
    }

    /// IMPL  
    /// Does the sub path from the parent path resolve to or through the specific child widget of the parent widget?  
    // returns None only of sub_path wouldn't resolve to or through the given child widget
    /// [`parent_path`]: Absolute path of the current parent widget  
    /// [`child`]: Child widget of parent widget to which the sub path probably resolves to/through  
    /// [`sub_path`]: Relative sub path to which widget should be attempted to resolve  
    fn resolves_thru_child_id(child: E::WidgetID, sub_path: &Self) -> Option<ResolvesThruResult<E>>;
    fn resolves_thru_child_path(child_path: &Self, sub_path: &Self) -> Option<ResolvesThruResult<E>>;

    fn for_child_widget_id(&self, child: E::WidgetID) -> Self;
    fn for_child_widget_path(&self, child_path: &Self) -> Self;


    // fn tip(&self) -> Option<&Self::SubPath>;
    /// returns the targeted widget ID if available
    /// NOTE this function is implemented optionally, so it may never return ID, even if possible
    fn _dest_widget(&self) -> Option<E::WidgetID> {
        None
    }
    #[deprecated]
    fn exact_eq(&self, o: &Self) -> bool;

    /// If self and o would resolve to the same widget
    fn dest_eq(&self, o: &Self) -> bool;

    /// the path which would resolve to the parent widget
    /// returns None only if the path is empty
    fn parent(&self) -> Option<Self>;

    /// if the path is empty e.g. doesn't resolve further
    fn is_empty(&self) -> bool;

    fn empty() -> Self;

    #[inline]
    fn with_env<F: Env<WidgetPath=E::WidgetPath>>(self) -> Self where E::WidgetPath: WidgetPath<F> {
        self
    }
}

pub struct ResolvesThruResult<E> where E: Env {
    /// the sub path inside the current child widget which resolves further
    pub sub_path: E::WidgetPath,
}

#[inline]
pub fn rc_path_with_env<E: Env, F: Env<WidgetPath=E::WidgetPath>>(e: E::WidgetPath) -> F::WidgetPath where E::WidgetPath: WidgetPath<F> {
    e
}
