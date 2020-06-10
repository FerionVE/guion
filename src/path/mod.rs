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
    
    fn attach(&mut self, sub: Self::SubPath);
    fn attached(mut self, sub: Self::SubPath) -> Self {
        self.attach(sub);
        self
    }

    fn attach_subpath(&mut self, sub: &Self);
    fn attached_subpath(mut self, sub: &Self) -> Self {
        self.attach_subpath(sub);
        self
    }

    fn tip(&self) -> Option<&Self::SubPath>;
    fn exact_eq(&self, o: &Self) -> bool;

    fn parent(&self) -> Option<Self>;

    fn is_empty(&self) -> bool;

    fn slice<T>(&self, range: T) -> Self where T: RangeBounds<usize>;
    fn index<T>(&self, i: T) -> &Self::SubPath where T: SliceIndex<[Self::SubPath],Output=Self::SubPath>;

    fn empty() -> Self;

    #[inline]
    fn with_env<F: Env<WidgetPath=E::WidgetPath>>(self) -> Self where E::WidgetPath: WidgetPath<F> {
        self
    }
}

#[inline]
pub fn rc_path_with_env<E: Env, F: Env<WidgetPath=E::WidgetPath>>(e: E::WidgetPath) -> F::WidgetPath where E::WidgetPath: WidgetPath<F> {
    e
}