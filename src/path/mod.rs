//! A path contains information, like widget id, which can denote a location of a widget
use std::{slice::SliceIndex, ops::RangeBounds};
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
    Send +
    Sync +
    'static
where E: Env {
    type SubPath: SubPath<E>;
    
    // TODO rename to attach_sub
    fn attach(&mut self, sub: Self::SubPath);
    #[inline]
    fn attached(mut self, sub: Self::SubPath) -> Self {
        self.attach(sub);
        self
    }

    // TODO rename to attach_path
    fn attach_path(&mut self, sub: &Self);
    #[inline]
    fn attached_path(mut self, sub: &Self) -> Self {
        self.attach_path(sub);
        self
    }

    /// IMPL
    /// does the sub path from the parent path resolve to or through the specific child widget of the parent widget?
    /// [`parent_path`]: absolute path of the current parent widget
    /// [`child`]: child widget of parent widget to which the sub path probably resolves to/through
    /// [`sub_path`]: relative sub path to which widget should be attempted to resolve
    fn _resolves_thru<W>(child: &W, sub_path: &Self) -> Option<ResolvesThruResult<E>> where W: Widget<E>+?Sized;

    fn for_child_widget<W>(&self, child: &W) -> Self where W: Widget<E>+?Sized;

    fn tip(&self) -> Option<&Self::SubPath>;
    fn exact_eq(&self, o: &Self) -> bool;

    fn parent(&self) -> Option<Self>;

    /// if the path is empty e.g. doesn't resolve further
    fn is_empty(&self) -> bool;

    fn slice<T>(&self, range: T) -> Self where T: RangeBounds<usize>;
    fn index<T>(&self, i: T) -> Option<&Self::SubPath> where T: SliceIndex<[Self::SubPath],Output=Self::SubPath>;

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
