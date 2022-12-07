use std::ops::Range;

use crate::dispatchor::{AsWidgetsDispatch, AsWidgetsIndexedDispatch, AsWidgetsResolveDispatch, AsWidgetsResult, AsWidgetsResolveResult};
use crate::env::Env;
use crate::newpath::{PathFragment, PathResolvusDyn};

use super::Widget;
use super::cache::WidgetCache;

pub mod fixed_idx;
pub mod tupled;

pub trait AsWidgets<E> where E: Env {
    type WidgetCache: WidgetCache<E>;
    type ChildID: PathFragment<E> + Clone + 'static; // + AppendToPathResolvor
    type IdIdxIter: Iterator<Item=(usize,Self::ChildID)>;

    fn by_index<F,R>(&self, idx: usize, callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsDispatch<Self::ChildID,R,E>;

    fn by_id<F,R>(&self, id: &Self::ChildID, callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsDispatch<Self::ChildID,R,E>;

    fn iter_ids(&self) -> Self::IdIdxIter;

    //fn sliced

    fn len(&self) -> usize;

    fn idx_range<F>(&self, range: Range<usize>, callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedDispatch<Self::ChildID,E>
    {
        self.idx_range_filtered(range, #[inline] |_, _| true, callback, root, ctx)
    }

    fn idx_range_filtered<F>(&self, range: Range<usize>, filter: impl for<'a> FnMut(usize,&'a Self::ChildID) -> bool, callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedDispatch<Self::ChildID,E>;

    fn resolve<F,R>(&self, path: &(dyn PathResolvusDyn<E>+'_), callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsResolveDispatch<Self::ChildID,R,E>;
}

impl<E,T> AsWidgets<E> for &'_ T where T: AsWidgets<E> + ?Sized, E: Env {
    type WidgetCache = T::WidgetCache;
    type ChildID = T::ChildID;
    type IdIdxIter = T::IdIdxIter;

    #[inline]
    fn by_index<F,R>(&self, idx: usize, callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsDispatch<Self::ChildID,R,E>
    {
        (**self).by_index(idx, callback, root, ctx)
    }

    #[inline]
    fn by_id<F,R>(&self, id: &Self::ChildID, callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsDispatch<Self::ChildID,R,E>
    {
        (**self).by_id(id, callback, root, ctx)
    }

    #[inline]
    fn iter_ids(&self) -> Self::IdIdxIter {
        (**self).iter_ids()
    }

    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }

    #[inline]
    fn idx_range<F>(&self, range: Range<usize>, callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedDispatch<Self::ChildID,E>
    {
        (**self).idx_range(range, callback, root, ctx)
    }

    #[inline]
    fn idx_range_filtered<F>(&self, range: Range<usize>, filter: impl for<'a> FnMut(usize,&'a Self::ChildID) -> bool, callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedDispatch<Self::ChildID,E>
    {
        (**self).idx_range_filtered(range, filter, callback, root, ctx)
    }

    #[inline]
    fn resolve<F,R>(&self, path: &(dyn PathResolvusDyn<E>+'_), callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsResolveDispatch<Self::ChildID,R,E>
    {
        (**self).resolve(path, callback, root, ctx)
    }
}
