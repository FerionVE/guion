use std::ops::{Range, Mul, Div};

use crate::dispatchor::*;
use crate::env::Env;
use crate::newpath::{PathFragment, PathResolvusDyn, FixedIdx};
use crate::root::RootRef;
use crate::widget::cache::DynWidgetCache;

use super::*;
use super::as_widget::AsWidget;

pub mod fixed_idx;
pub mod tupled;

pub trait AsWidgets<E> where E: Env {
    type Widget<'v,'z>: Widget<E,Cache=Self::WidgetCache> + ?Sized + 'v where 'z: 'v, Self: 'z;
    type WidgetCache: WidgetCache<E>;
    type ChildID: PathFragment<E> + Clone + 'static; // + AppendToPathResolvor
    type IdIdxIter: Iterator<Item=(usize,Self::ChildID)>;

    fn by_index<'w,R>(&self, idx: usize, callback: &mut (dyn AsWidgetsDispatch<'w,Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        Self: 'w;

    fn by_id<'w,R>(&self, id: &Self::ChildID, callback: &mut (dyn AsWidgetsDispatch<'w,Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        Self: 'w;

    fn iter_ids(&self) -> Self::IdIdxIter;

    //fn sliced

    fn len(&self) -> usize;

    fn idx_range<'w>(&self, range: Range<usize>, callback: &mut (dyn AsWidgetsIndexedDispatch<'w,Self,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        Self: 'w
    {
        self.idx_range_filtered(range, #[inline] |_, _| true, callback, root, ctx)
    }

    fn idx_range_filtered<'w>(&self, range: Range<usize>, filter: impl for<'a> FnMut(usize,&'a Self::ChildID) -> bool, callback: &mut (dyn AsWidgetsIndexedDispatch<'w,Self,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        Self: 'w;

    fn resolve<'w,R>(&self, path: &(dyn PathResolvusDyn<E>+'_), callback: &mut (dyn AsWidgetsResolveDispatch<'w,Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        Self: 'w;
}

impl<E,T> AsWidgets<E> for &'_ T where T: AsWidgets<E> + ?Sized, E: Env {
    type Widget<'v,'z> = T::Widget<'v,'z> where 'z: 'v, Self: 'z;
    type WidgetCache = T::WidgetCache;
    type ChildID = T::ChildID;
    type IdIdxIter = T::IdIdxIter;

    #[inline]
    fn by_index<'w,R>(&self, idx: usize, callback: &mut (dyn AsWidgetsDispatch<'w,Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        Self: 'w
    {
        let mut callback = AsWidgetsClosure::<'_,_,T,R,E>::new(#[inline] |result,root,ctx| {
            let result = match result {
                Some(r) => Some(AsWidgetsResult {
                    idx: r.idx,
                    child_id: r.child_id,
                    widget: r.widget,
                }),
                None => None,
            };
            callback.call(result, root, ctx)
        });
        (**self).by_index(idx, &mut callback, root, ctx)
    }

    #[inline]
    fn by_id<'w,R>(&self, id: &Self::ChildID, callback: &mut (dyn AsWidgetsDispatch<'w,Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        Self: 'w
    {
        let mut callback = AsWidgetsClosure::new(#[inline] |result,root,ctx| {
            let result = match result {
                Some(r) => Some(AsWidgetsResult {
                    idx: r.idx,
                    child_id: r.child_id,
                    widget: r.widget,
                }),
                None => None,
            };
            callback.call(result, root, ctx)
        });
        (**self).by_id(id, &mut callback, root, ctx)
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
    fn idx_range<'w>(&self, range: Range<usize>, mut callback: &mut (dyn AsWidgetsIndexedDispatch<'w,Self,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        Self: 'w
    {
        let mut callback = AsWidgetsAllClosure::new(#[inline] |idx,child_id,widget,root,ctx| {
            callback.call(idx, child_id, widget, root, ctx)
        });
        (**self).idx_range(range, &mut callback, root, ctx)
    }

    #[inline]
    fn idx_range_filtered<'w>(&self, range: Range<usize>, filter: impl for<'a> FnMut(usize,&'a Self::ChildID) -> bool, mut callback: &mut (dyn AsWidgetsIndexedDispatch<'w,Self,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        Self: 'w
    {
        let mut callback = AsWidgetsAllClosure::new(#[inline] |idx,child_id,widget,root,ctx| {
            callback.call(idx, child_id, widget, root, ctx)
        });
        (**self).idx_range_filtered(range, filter, &mut callback, root, ctx)
    }

    #[inline]
    fn resolve<'w,R>(&self, path: &(dyn PathResolvusDyn<E>+'_), callback: &mut (dyn AsWidgetsResolveDispatch<'w,Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        Self: 'w
    {
        let mut callback = AsWidgetsResolveClosure::new(#[inline] |result,root,ctx| {
            let result = match result {
                Some(r) => Some(AsWidgetsResolveResult {
                    idx: r.idx,
                    child_id: r.child_id,
                    resolvus: r.resolvus,
                    widget: r.widget,
                }),
                None => None,
            };
            callback.call(result, root, ctx)
        });
        (**self).resolve(path, &mut callback, root, ctx)
    }
}
