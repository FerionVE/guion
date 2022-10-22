use std::ops::Range;

use crate::dispatchor::{AsWidgetsDispatch, AsWidgetsIndexedDispatch, AsWidgetsResolveDispatch, AsWidgetsAllClosure, AsWidgetsResolveClosure, AsWidgetsClosure, AsWidgetsResult, AsWidgetsResolveResult};
use crate::env::Env;
use crate::newpath::{PathFragment, PathResolvusDyn};

use super::Widget;
use super::cache::WidgetCache;

pub mod fixed_idx;
pub mod tupled;

pub trait AsWidgets<E> where E: Env {
    type Widget<'v>: Widget<E,Cache=Self::WidgetCache> + ?Sized + 'v where Self: 'v;
    type WidgetCache: WidgetCache<E>;
    type ChildID: PathFragment<E> + Clone + 'static; // + AppendToPathResolvor
    type IdIdxIter: Iterator<Item=(usize,Self::ChildID)>;

    fn by_index<R>(&self, idx: usize, callback: &mut (dyn AsWidgetsDispatch<Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R;

    fn by_id<R>(&self, id: &Self::ChildID, callback: &mut (dyn AsWidgetsDispatch<Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R;

    fn iter_ids(&self) -> Self::IdIdxIter;

    //fn sliced

    fn len(&self) -> usize;

    fn idx_range(&self, range: Range<usize>, callback: &mut (dyn AsWidgetsIndexedDispatch<Self,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) {
        self.idx_range_filtered(range, #[inline] |_, _| true, callback, root, ctx)
    }

    fn idx_range_filtered(&self, range: Range<usize>, filter: impl for<'a> FnMut(usize,&'a Self::ChildID) -> bool, callback: &mut (dyn AsWidgetsIndexedDispatch<Self,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>);

    fn resolve<R>(&self, path: &(dyn PathResolvusDyn<E>+'_), callback: &mut (dyn AsWidgetsResolveDispatch<Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R;

    fn covar_ref<'s,'ll,'ss>(w: &'s Self::Widget<'ll>) -> &'s Self::Widget<'ss> where 'll: 'ss, 'ss: 's, Self: 'll;
}

impl<E,T> AsWidgets<E> for &'_ T where T: AsWidgets<E> + ?Sized, E: Env {
    type Widget<'v> = T::Widget<'v> where Self: 'v;
    type WidgetCache = T::WidgetCache;
    type ChildID = T::ChildID;
    type IdIdxIter = T::IdIdxIter;

    #[inline]
    fn by_index<R>(&self, idx: usize, callback: &mut (dyn AsWidgetsDispatch<Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R {
        let mut callback = AsWidgetsClosure::<_,T,R,E>::new(#[inline] |result,root,ctx| {
            let result = match result {
                Some(r) => Some(AsWidgetsResult {
                    idx: r.idx,
                    child_id: r.child_id,
                    widget: T::covar_ref(r.widget),
                }),
                None => None,
            };
            callback.call(result, root, ctx)
        });
        (**self).by_index(idx, &mut callback, root, ctx)
    }

    #[inline]
    fn by_id<R>(&self, id: &Self::ChildID, callback: &mut (dyn AsWidgetsDispatch<Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R {
        let mut callback = AsWidgetsClosure::new(#[inline] |result,root,ctx| {
            let result = match result {
                Some(r) => Some(AsWidgetsResult {
                    idx: r.idx,
                    child_id: r.child_id,
                    widget: T::covar_ref(r.widget),
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
    fn idx_range(&self, range: Range<usize>, callback: &mut (dyn AsWidgetsIndexedDispatch<Self,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) {
        let mut callback = AsWidgetsAllClosure::new(#[inline] |idx,child_id,widget,root,ctx| {
            callback.call(idx, child_id, T::covar_ref(widget), root, ctx)
        });
        (**self).idx_range(range, &mut callback, root, ctx)
    }

    #[inline]
    fn idx_range_filtered(&self, range: Range<usize>, filter: impl for<'a> FnMut(usize,&'a Self::ChildID) -> bool, callback: &mut (dyn AsWidgetsIndexedDispatch<Self,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) {
        let mut callback = AsWidgetsAllClosure::new(#[inline] |idx,child_id,widget,root,ctx| {
            callback.call(idx, child_id, T::covar_ref(widget), root, ctx)
        });
        (**self).idx_range_filtered(range, filter, &mut callback, root, ctx)
    }

    #[inline]
    fn resolve<R>(&self, path: &(dyn PathResolvusDyn<E>+'_), callback: &mut (dyn AsWidgetsResolveDispatch<Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R {
        let mut callback = AsWidgetsResolveClosure::new(#[inline] |result,root,ctx| {
            let result = match result {
                Some(r) => Some(AsWidgetsResolveResult {
                    idx: r.idx,
                    child_id: r.child_id,
                    resolvus: r.resolvus,
                    widget: T::covar_ref(r.widget),
                }),
                None => None,
            };
            callback.call(result, root, ctx)
        });
        (**self).resolve(path, &mut callback, root, ctx)
    }

    #[inline]
    fn covar_ref<'s,'ll,'ss>(w: &'s Self::Widget<'ll>) -> &'s Self::Widget<'ss> where 'll: 'ss, 'ss: 's, Self: 'll {
        T::covar_ref(w)
    }
}
