use std::ops::Range;

use crate::dispatchor::{AsWidgetsDispatch, AsWidgetsIndexedDispatch, AsWidgetsResolveDispatch, AsWidgetsCDispatch, AsWidgetsIndexedCDispatch, AsWidgetsResolveCDispatch, AsWidgetsDispatchMut, AsWidgetsCDispatchMut, AsWidgetsIndexedDispatchMut, AsWidgetsIndexedCDispatchMut, AsWidgetsResolveDispatchMut, AsWidgetsResolveCDispatchMut};
use crate::env::Env;
use crate::newpath::{PathFragment, PathResolvusDyn};

use super::dyn_tunnel::WidgetDyn;
use super::id::WidgetID;

pub mod fixed_idx;
pub mod tupled;

pub trait AsWidgets<E>: AsWidgetsDyn<E> where E: Env {
    type Caches: Default + Sized + 'static;
    type IdIdxIter: Iterator<Item=(isize,Self::ChildID)>;
    type IndexedSideData<T>: Clone + Default + AsRef<[T]> + AsMut<[T]> where T: Clone + Default;
    type IndexedSideData2<T>: Clone + Default + AsRef<[T]> + AsMut<[T]> + 'static where T: Clone + Default + 'static;

    fn by_index<F,R>(&self, idx: isize, callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsDispatch<Self::ChildID,R,E>;

    fn by_index_mut<F,R>(&mut self, idx: isize, callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsDispatchMut<Self::ChildID,R,E>;

    fn by_index_c<F,R>(&self, idx: isize, callback: F, caches: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsCDispatch<Self::ChildID,R,E>;

    fn by_index_c_mut<F,R>(&mut self, idx: isize, callback: F, caches: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsCDispatchMut<Self::ChildID,R,E>;

    fn iter_ids(&self) -> Self::IdIdxIter;

    fn idx_range<F>(&self, range: Range<isize>, callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedDispatch<Self::ChildID,E>;

    fn idx_range_mut<F>(&mut self, range: Range<isize>, callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedDispatchMut<Self::ChildID,E>;

    fn idx_range_c<F>(&self, range: Range<isize>, callback: F, caches: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedCDispatch<Self::ChildID,E>;

    fn idx_range_c_mut<F>(&mut self, range: Range<isize>, callback: F, caches: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedCDispatchMut<Self::ChildID,E>;

    fn resolve<F,R>(&self, path: &(dyn PathResolvusDyn<E>+'_), callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsResolveDispatch<Self::ChildID,R,E>;

    fn resolve_mut<F,R>(&mut self, path: &(dyn PathResolvusDyn<E>+'_), callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsResolveDispatchMut<Self::ChildID,R,E>;

    fn resolve_c<F,R>(&self, path: &(dyn PathResolvusDyn<E>+'_), callback: F, caches: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsResolveCDispatch<Self::ChildID,R,E>;

    fn resolve_c_mut<F,R>(&mut self, path: &(dyn PathResolvusDyn<E>+'_), callback: F, caches: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsResolveCDispatchMut<Self::ChildID,R,E>;
}

pub trait AsWidgetsDyn<E> where E: Env {
    type ChildID: PathFragment<E> + Clone + 'static;

    fn by_index_dyn(&self, idx: isize) -> Option<AsWidgetsDynResult<'_,Self::ChildID,E>>;

    fn by_index_dyn_mut(&mut self, idx: isize) -> Option<AsWidgetsDynResultMut<'_,Self::ChildID,E>>;

    fn range(&self) -> Range<isize>;

    fn idx_range_dyn<'a>(&'a self, range: Range<isize>, callback: &mut (dyn FnMut(AsWidgetsDynResult<'a,Self::ChildID,E>) + '_) );

    fn idx_range_dyn_mut<'a>(&'a mut self, range: Range<isize>, callback: &mut (dyn FnMut(AsWidgetsDynResultMut<'a,Self::ChildID,E>) + '_) );

    fn resolve_dyn<'a,'b>(&'a self, path: &'b (dyn PathResolvusDyn<E>+'b)) -> Option<AsWidgetsDynResolveResult<'a,'b,Self::ChildID,E>>;

    fn resolve_dyn_mut<'a,'b>(&'a mut self, path: &'b (dyn PathResolvusDyn<E>+'b)) -> Option<AsWidgetsDynResolveResultMut<'a,'b,Self::ChildID,E>>;

    fn collect_range(&self, range: Range<isize>) -> Vec<AsWidgetsDynResult<'_,Self::ChildID,E>> {
        let mut dest = Vec::with_capacity(range.len());
        self.idx_range_dyn(range, &mut |result| dest.push(result) );
        dest
    }

    fn collect_range_mut(&mut self, range: Range<isize>) -> Vec<AsWidgetsDynResultMut<'_,Self::ChildID,E>> {
        let mut dest = Vec::with_capacity(range.len());
        self.idx_range_dyn_mut(range, &mut |result| dest.push(result) );
        dest
    }
}

pub struct AsWidgetsDynResult<'a,CID,E> where CID: PathFragment<E> + Clone + 'static {
    pub widget: &'a (dyn WidgetDyn<E>+'a),
    pub widget_id: WidgetID,
    pub child_id: CID,
    pub idx: isize,
}
pub struct AsWidgetsDynResolveResult<'a,'b,CID,E> where CID: PathFragment<E> + Clone + 'static {
    pub widget: &'a (dyn WidgetDyn<E>+'a),
    pub widget_id: WidgetID,
    pub child_id: CID,
    pub idx: isize,
    pub resolvus: &'b (dyn PathResolvusDyn<E>+'b),
}

pub struct AsWidgetsDynResultMut<'a,CID,E> where CID: PathFragment<E> + Clone + 'static {
    pub widget: &'a mut (dyn WidgetDyn<E>+'a),
    pub widget_id: WidgetID,
    pub child_id: CID,
    pub idx: isize,
}
pub struct AsWidgetsDynResolveResultMut<'a,'b,CID,E> where CID: PathFragment<E> + Clone + 'static {
    pub widget: &'a mut (dyn WidgetDyn<E>+'a),
    pub widget_id: WidgetID,
    pub child_id: CID,
    pub idx: isize,
    pub resolvus: &'b (dyn PathResolvusDyn<E>+'b),
}

impl<'a,'b,CID,E> From<AsWidgetsDynResolveResult<'a,'b,CID,E>> for AsWidgetsDynResult<'a,CID,E> where CID: PathFragment<E> + Clone + 'static {
    #[inline]
    fn from(v: AsWidgetsDynResolveResult<'a,'b,CID,E>) -> Self {
        Self {
            idx: v.idx,
            widget: v.widget,
            widget_id: v.widget_id,
            child_id: v.child_id,
        }
    }
}
impl<'a,'b,CID,E> From<AsWidgetsDynResolveResultMut<'a,'b,CID,E>> for AsWidgetsDynResultMut<'a,CID,E> where CID: PathFragment<E> + Clone + 'static {
    #[inline]
    fn from(v: AsWidgetsDynResolveResultMut<'a,'b,CID,E>) -> Self {
        Self {
            idx: v.idx,
            widget: v.widget,
            widget_id: v.widget_id,
            child_id: v.child_id,
        }
    }
}
