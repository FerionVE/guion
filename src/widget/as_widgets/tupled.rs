use std::ops::{Range, Deref, DerefMut};

use crate::dispatchor::*;
use crate::env::Env;
use crate::newpath::{PathFragment, PathResolvusDyn, PathResolvus};
use crate::root::RootRef;
use crate::widget::Widget;

use super::{AsWidgets, AsWidgetsDyn, AsWidgetsDynResult, AsWidgetsDynResolveResult, AsWidgetsDynResolveResultMut, AsWidgetsDynResultMut};
use super::fixed_idx::DefaultHack;

#[repr(transparent)]
pub struct Tupled<T>(pub T) where T: ?Sized;

impl<T> Deref for Tupled<T> where T: ?Sized {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for Tupled<T> where T: ?Sized {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<E,I,T> AsWidgets<E> for Tupled<Vec<(I,T)>> where T: Widget<E>, E: Env, I: PathFragment<E> + Clone + PartialEq + 'static {
    type Caches = Vec<T::Cache>;
    type IdIdxIter = impl Iterator<Item=(isize,Self::ChildID)>;
    type IndexedSideData<TT> = Vec<TT> where TT: Clone + Default;
    type IndexedSideData2<TT> = Self::IndexedSideData<TT> where TT: Clone + Default + 'static;

    #[inline]
    fn by_index<F,R>(&self, idx: isize, callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsDispatch<Self::ChildID,R,E>
    {
        match self.0.get(idx as usize) {
            Some((id,inner)) => {
                callback.call(AsWidgetsResult::from_some(idx,id.clone(),inner), root, ctx)
            },
            None => callback.call_none(root,ctx),
        }
    }

    #[inline]
    fn by_index_mut<F,R>(&mut self, idx: isize, callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsDispatchMut<Self::ChildID,R,E>
    {
        match self.0.get_mut(idx as usize) {
            Some((id,inner)) => {
                callback.call(AsWidgetsResultMut::from_some(idx,id.clone(),inner), root, ctx)
            },
            None => callback.call_none(root,ctx),
        }
    }

    #[inline]
    fn by_index_c<F,R>(&self, idx: isize, callback: F, cache: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsCDispatch<Self::ChildID,R,E>
    {
        if self.0.len() != cache.len() {
            cache.resize_with(self.0.len(), Default::default);
        }

        match self.0.get(idx as usize) {
            Some((id,inner)) => {
                let cache_slot = unsafe { cache.get_unchecked_mut(idx as usize) };
                callback.call(AsWidgetsCResult::from_some(idx,id.clone(),inner, cache_slot), root, ctx)
            },
            None => callback.call_none(root,ctx),
        }
    }

    #[inline]
    fn by_index_c_mut<F,R>(&mut self, idx: isize, callback: F, cache: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsCDispatchMut<Self::ChildID,R,E>
    {
        if self.0.len() != cache.len() {
            cache.resize_with(self.0.len(), Default::default);
        }

        match self.0.get_mut(idx as usize) {
            Some((id,inner)) => {
                let cache_slot = unsafe { cache.get_unchecked_mut(idx as usize) };
                callback.call(AsWidgetsCResultMut::from_some(idx,id.clone(),inner, cache_slot), root, ctx)
            },
            None => callback.call_none(root,ctx),
        }
    }

    #[inline]
    fn iter_ids(&self) -> Self::IdIdxIter {
        let h = self.0.iter().enumerate().map(#[inline] |(i,(id,_))| (i as isize,id.clone()) ).collect::<Vec<_>>();
        h.into_iter()
    }

    #[inline]
    fn idx_range<F>(&self, range: Range<isize>, mut callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedDispatch<Self::ChildID,E>
    {
        for (i,(id,v)) in self.0[range.start as usize .. range.end as usize].iter().enumerate() {
            callback.call(i as isize, id.clone(), v, root.fork(), ctx)
        }
    }

    #[inline]
    fn idx_range_mut<F>(&mut self, range: Range<isize>, mut callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedDispatchMut<Self::ChildID,E>
    {
        for (i,(id,v)) in self.0[range.start as usize .. range.end as usize].iter_mut().enumerate() {
            callback.call(i as isize, id.clone(), v, root.fork(), ctx)
        }
    }

    #[inline]
    fn idx_range_c<F>(&self, range: Range<isize>, mut callback: F, cache: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedCDispatch<Self::ChildID,E>
    {
        if self.0.len() != cache.len() {
            cache.resize_with(self.0.len(), Default::default);
        }

        for (i,(id,v)) in self.0[range.start as usize .. range.end as usize].iter().enumerate() {
            let cache_slot = unsafe { cache.get_unchecked_mut(i) };
            callback.call(i as isize, id.clone(), v, cache_slot, root.fork(), ctx)
        }
    }

    #[inline]
    fn idx_range_c_mut<F>(&mut self, range: Range<isize>, mut callback: F, cache: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedCDispatchMut<Self::ChildID,E>
    {
        if self.0.len() != cache.len() {
            cache.resize_with(self.0.len(), Default::default);
        }

        for (i,(id,v)) in self.0[range.start as usize .. range.end as usize].iter_mut().enumerate() {
            let cache_slot = unsafe { cache.get_unchecked_mut(i) };
            callback.call(i as isize, id.clone(), v, cache_slot, root.fork(), ctx)
        }
    }

    fn resolve<F,R>(&self, path: &(dyn PathResolvusDyn<E>+'_), callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsResolveDispatch<Self::ChildID,R,E>
    {
        if let Some(v) = path.try_fragment::<Self::ChildID>() {
            let res = self.0.iter().enumerate()
                .find(#[inline] |(_,(i,_))| *i == *v);

            if let Some((idx,(id,inner))) = res {
                return callback.call(AsWidgetsResolveResult::from_some(idx as isize,id.clone(),path.inner().unwrap(),inner), root, ctx);
            }
        }

        callback.call_none(root,ctx)
    }

    fn resolve_mut<F,R>(&mut self, path: &(dyn PathResolvusDyn<E>+'_), callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsResolveDispatchMut<Self::ChildID,R,E>
    {
        if let Some(v) = path.try_fragment::<Self::ChildID>() {
            let res = self.0.iter_mut().enumerate()
                .find(#[inline] |(_,(i,_))| *i == *v);

            if let Some((idx,(id,inner))) = res {
                return callback.call(AsWidgetsResolveResultMut::from_some(idx as isize,id.clone(),path.inner().unwrap(),inner), root, ctx);
            }
        }

        callback.call_none(root,ctx)
    }

    fn resolve_c<F,R>(&self, path: &(dyn PathResolvusDyn<E>+'_), callback: F, cache: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsResolveCDispatch<Self::ChildID,R,E>
    {
        if self.0.len() != cache.len() {
            cache.resize_with(self.0.len(), Default::default);
        }

        if let Some(v) = path.try_fragment::<Self::ChildID>() {
            let res = self.0.iter().enumerate()
                .find(#[inline] |(_,(i,_))| *i == *v);

            if let Some((idx,(id,inner))) = res {
                let cache_slot = unsafe { cache.get_unchecked_mut(idx) };
                return callback.call(AsWidgetsResolveCResult::from_some(idx as isize,id.clone(),path.inner().unwrap(),inner, cache_slot), root, ctx);
            }
        }

        callback.call_none(root,ctx)
    }

    fn resolve_c_mut<F,R>(&mut self, path: &(dyn PathResolvusDyn<E>+'_), callback: F, cache: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsResolveCDispatchMut<Self::ChildID,R,E>
    {
        if self.0.len() != cache.len() {
            cache.resize_with(self.0.len(), Default::default);
        }

        if let Some(v) = path.try_fragment::<Self::ChildID>() {
            let res = self.0.iter_mut().enumerate()
                .find(#[inline] |(_,(i,_))| *i == *v);

            if let Some((idx,(id,inner))) = res {
                let cache_slot = unsafe { cache.get_unchecked_mut(idx) };
                return callback.call(AsWidgetsResolveCResultMut::from_some(idx as isize,id.clone(),path.inner().unwrap(),inner, cache_slot), root, ctx);
            }
        }

        callback.call_none(root,ctx)
    }
}

impl<E,I,T,const N: usize> AsWidgets<E> for Tupled<[(I,T);N]> where T: Widget<E>, E: Env, I: PathFragment<E> + Clone + PartialEq + 'static {
    type Caches = Vec<T::Cache>;
    type IdIdxIter = impl Iterator<Item=(isize,Self::ChildID)>;
    type IndexedSideData<TT> = DefaultHack<[TT;N]> where TT: Clone + Default;
    type IndexedSideData2<TT> = Self::IndexedSideData<TT> where TT: Clone + Default + 'static;

    #[inline]
    fn by_index<F,R>(&self, idx: isize, callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsDispatch<Self::ChildID,R,E>
    {
        match self.0.get(idx as usize) {
            Some((id,inner)) => {
                callback.call(AsWidgetsResult::from_some(idx,id.clone(),inner), root, ctx)
            },
            None => callback.call_none(root,ctx),
        }
    }

    #[inline]
    fn by_index_mut<F,R>(&mut self, idx: isize, callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsDispatchMut<Self::ChildID,R,E>
    {
        match self.0.get_mut(idx as usize) {
            Some((id,inner)) => {
                callback.call(AsWidgetsResultMut::from_some(idx,id.clone(),inner), root, ctx)
            },
            None => callback.call_none(root,ctx),
        }
    }

    #[inline]
    fn by_index_c<F,R>(&self, idx: isize, callback: F, cache: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsCDispatch<Self::ChildID,R,E>
    {
        if self.0.len() != cache.len() {
            cache.resize_with(self.0.len(), Default::default);
        }

        match self.0.get(idx as usize) {
            Some((id,inner)) => {
                let cache_slot = unsafe { cache.get_unchecked_mut(idx as usize) };
                callback.call(AsWidgetsCResult::from_some(idx,id.clone(),inner, cache_slot), root, ctx)
            },
            None => callback.call_none(root,ctx),
        }
    }

    #[inline]
    fn by_index_c_mut<F,R>(&mut self, idx: isize, callback: F, cache: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsCDispatchMut<Self::ChildID,R,E>
    {
        if self.0.len() != cache.len() {
            cache.resize_with(self.0.len(), Default::default);
        }

        match self.0.get_mut(idx as usize) {
            Some((id,inner)) => {
                let cache_slot = unsafe { cache.get_unchecked_mut(idx as usize) };
                callback.call(AsWidgetsCResultMut::from_some(idx,id.clone(),inner, cache_slot), root, ctx)
            },
            None => callback.call_none(root,ctx),
        }
    }

    #[inline]
    fn iter_ids(&self) -> Self::IdIdxIter {
        let h = self.0.iter().enumerate().map(#[inline] |(i,(id,_))| (i as isize,id.clone()) ).collect::<Vec<_>>();
        h.into_iter()
    }

    #[inline]
    fn idx_range<F>(&self, range: Range<isize>, mut callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedDispatch<Self::ChildID,E>
    {
        for (i,(id,v)) in self.0[range.start as usize .. range.end as usize].iter().enumerate() {
            callback.call(i as isize, id.clone(), v, root.fork(), ctx)
        }
    }

    #[inline]
    fn idx_range_mut<F>(&mut self, range: Range<isize>, mut callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedDispatchMut<Self::ChildID,E>
    {
        for (i,(id,v)) in self.0[range.start as usize .. range.end as usize].iter_mut().enumerate() {
            callback.call(i as isize, id.clone(), v, root.fork(), ctx)
        }
    }

    #[inline]
    fn idx_range_c<F>(&self, range: Range<isize>, mut callback: F, cache: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedCDispatch<Self::ChildID,E>
    {
        if self.0.len() != cache.len() {
            cache.resize_with(self.0.len(), Default::default);
        }

        for (i,(id,v)) in self.0[range.start as usize .. range.end as usize].iter().enumerate() {
            let cache_slot = unsafe { cache.get_unchecked_mut(i) };
            callback.call(i as isize, id.clone(), v, cache_slot, root.fork(), ctx)
        }
    }

    #[inline]
    fn idx_range_c_mut<F>(&mut self, range: Range<isize>, mut callback: F, cache: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedCDispatchMut<Self::ChildID,E>
    {
        if self.0.len() != cache.len() {
            cache.resize_with(self.0.len(), Default::default);
        }

        for (i,(id,v)) in self.0[range.start as usize .. range.end as usize].iter_mut().enumerate() {
            let cache_slot = unsafe { cache.get_unchecked_mut(i) };
            callback.call(i as isize, id.clone(), v, cache_slot, root.fork(), ctx)
        }
    }

    fn resolve<F,R>(&self, path: &(dyn PathResolvusDyn<E>+'_), callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsResolveDispatch<Self::ChildID,R,E>
    {
        if let Some(v) = path.try_fragment::<Self::ChildID>() {
            let res = self.0.iter().enumerate()
                .find(#[inline] |(_,(i,_))| *i == *v);

            if let Some((idx,(id,inner))) = res {
                return callback.call(AsWidgetsResolveResult::from_some(idx as isize,id.clone(),path.inner().unwrap(),inner), root, ctx);
            }
        }

        callback.call_none(root,ctx)
    }

    fn resolve_mut<F,R>(&mut self, path: &(dyn PathResolvusDyn<E>+'_), callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsResolveDispatchMut<Self::ChildID,R,E>
    {
        if let Some(v) = path.try_fragment::<Self::ChildID>() {
            let res = self.0.iter_mut().enumerate()
                .find(#[inline] |(_,(i,_))| *i == *v);

            if let Some((idx,(id,inner))) = res {
                return callback.call(AsWidgetsResolveResultMut::from_some(idx as isize,id.clone(),path.inner().unwrap(),inner), root, ctx);
            }
        }

        callback.call_none(root,ctx)
    }

    fn resolve_c<F,R>(&self, path: &(dyn PathResolvusDyn<E>+'_), callback: F, cache: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsResolveCDispatch<Self::ChildID,R,E>
    {
        if self.0.len() != cache.len() {
            cache.resize_with(self.0.len(), Default::default);
        }

        if let Some(v) = path.try_fragment::<Self::ChildID>() {
            let res = self.0.iter().enumerate()
                .find(#[inline] |(_,(i,_))| *i == *v);

            if let Some((idx,(id,inner))) = res {
                let cache_slot = unsafe { cache.get_unchecked_mut(idx) };
                return callback.call(AsWidgetsResolveCResult::from_some(idx as isize,id.clone(),path.inner().unwrap(),inner, cache_slot), root, ctx);
            }
        }

        callback.call_none(root,ctx)
    }

    fn resolve_c_mut<F,R>(&mut self, path: &(dyn PathResolvusDyn<E>+'_), callback: F, cache: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsResolveCDispatchMut<Self::ChildID,R,E>
    {
        if self.0.len() != cache.len() {
            cache.resize_with(self.0.len(), Default::default);
        }

        if let Some(v) = path.try_fragment::<Self::ChildID>() {
            let res = self.0.iter_mut().enumerate()
                .find(#[inline] |(_,(i,_))| *i == *v);

            if let Some((idx,(id,inner))) = res {
                let cache_slot = unsafe { cache.get_unchecked_mut(idx) };
                return callback.call(AsWidgetsResolveCResultMut::from_some(idx as isize,id.clone(),path.inner().unwrap(),inner, cache_slot), root, ctx);
            }
        }

        callback.call_none(root,ctx)
    }
}

impl<E,I,T> AsWidgetsDyn<E> for Tupled<Vec<(I,T)>> where T: Widget<E>, E: Env, I: PathFragment<E> + Clone + PartialEq + 'static {
    type ChildID = I;

    #[inline]
    fn by_index_dyn(&self, idx: isize) -> Option<AsWidgetsDynResult<'_,Self::ChildID,E>> {
        self.0.get(idx as usize).map(|(id,inner)| AsWidgetsDynResult {
            widget: inner,
            widget_id: inner.id(),
            child_id: id.clone(),
            idx,
        })
    }

    #[inline]
    fn by_index_dyn_mut(&mut self, idx: isize) -> Option<super::AsWidgetsDynResultMut<'_,Self::ChildID,E>> {
        self.0.get_mut(idx as usize).map(|(id,inner)| AsWidgetsDynResultMut {
            widget_id: inner.id(),
            widget: inner,
            child_id: id.clone(),
            idx,
        })
    }

    #[inline]
    fn range(&self) -> Range<isize> {
        0 .. self.0.len() as isize
    }

    #[inline]
    fn idx_range_dyn<'a>(&'a self, range: Range<isize>, callback: &mut (dyn FnMut(AsWidgetsDynResult<'a,Self::ChildID,E>) + '_) ) {
        for (i,(id,v)) in self.0[range.start as usize .. range.end as usize].iter().enumerate() {
            callback(AsWidgetsDynResult {
                widget: v,
                widget_id: v.id(),
                child_id: id.clone(),
                idx: i as isize,
            })
        }
    }

    #[inline]
    fn idx_range_dyn_mut<'a>(&'a mut self, range: Range<isize>, callback: &mut (dyn FnMut(super::AsWidgetsDynResultMut<'a,Self::ChildID,E>) + '_) ) {
        for (i,(id,v)) in self.0[range.start as usize .. range.end as usize].iter_mut().enumerate() {
            callback(AsWidgetsDynResultMut {
                widget_id: v.id(),
                widget: v,
                child_id: id.clone(),
                idx: i as isize,
            })
        }
    }

    fn resolve_dyn<'a,'b>(&'a self, path: &'b (dyn PathResolvusDyn<E>+'b)) -> Option<AsWidgetsDynResolveResult<'a,'b,Self::ChildID,E>> {
        if let Some(v) = path.try_fragment::<Self::ChildID>() {
            let res = self.0.iter().enumerate()
                .find(#[inline] |(_,(i,_))| *i == *v);

            if let Some((idx,(id,inner))) = res {
                return Some(AsWidgetsDynResolveResult {
                    widget: inner,
                    widget_id: inner.id(),
                    child_id: id.clone(),
                    idx: idx as isize,
                    resolvus: path.inner().unwrap(),
                });
            }
        }

        None
    }

    fn resolve_dyn_mut<'a,'b>(&'a mut self, path: &'b (dyn PathResolvusDyn<E>+'b)) -> Option<AsWidgetsDynResolveResultMut<'a,'b,Self::ChildID,E>> {
        if let Some(v) = path.try_fragment::<Self::ChildID>() {
            let res = self.0.iter_mut().enumerate()
                .find(#[inline] |(_,(i,_))| *i == *v);

            if let Some((idx,(id,inner))) = res {
                return Some(AsWidgetsDynResolveResultMut {
                    widget_id: inner.id(),
                    widget: inner,
                    child_id: id.clone(),
                    idx: idx as isize,
                    resolvus: path.inner().unwrap(),
                });
            }
        }

        None
    }
}

impl<E,I,T,const N: usize> AsWidgetsDyn<E> for Tupled<[(I,T);N]> where T: Widget<E>, E: Env, I: PathFragment<E> + Clone + PartialEq + 'static {
    type ChildID = I;

    #[inline]
    fn by_index_dyn(&self, idx: isize) -> Option<AsWidgetsDynResult<'_,Self::ChildID,E>> {
        self.0.get(idx as usize).map(|(id,inner)| AsWidgetsDynResult {
            widget: inner,
            widget_id: inner.id(),
            child_id: id.clone(),
            idx,
        })
    }

    #[inline]
    fn by_index_dyn_mut(&mut self, idx: isize) -> Option<super::AsWidgetsDynResultMut<'_,Self::ChildID,E>> {
        self.0.get_mut(idx as usize).map(|(id,inner)| AsWidgetsDynResultMut {
            widget_id: inner.id(),
            widget: inner,
            child_id: id.clone(),
            idx,
        })
    }

    #[inline]
    fn range(&self) -> Range<isize> {
        0 .. N as isize
    }

    #[inline]
    fn idx_range_dyn<'a>(&'a self, range: Range<isize>, callback: &mut (dyn FnMut(AsWidgetsDynResult<'a,Self::ChildID,E>) + '_) ) {
        for (i,(id,v)) in self.0[range.start as usize .. range.end as usize].iter().enumerate() {
            callback(AsWidgetsDynResult {
                widget: v,
                widget_id: v.id(),
                child_id: id.clone(),
                idx: i as isize,
            })
        }
    }

    #[inline]
    fn idx_range_dyn_mut<'a>(&'a mut self, range: Range<isize>, callback: &mut (dyn FnMut(super::AsWidgetsDynResultMut<'a,Self::ChildID,E>) + '_) ) {
        for (i,(id,v)) in self.0[range.start as usize .. range.end as usize].iter_mut().enumerate() {
            callback(AsWidgetsDynResultMut {
                widget_id: v.id(),
                widget: v,
                child_id: id.clone(),
                idx: i as isize,
            })
        }
    }

    fn resolve_dyn<'a,'b>(&'a self, path: &'b (dyn PathResolvusDyn<E>+'b)) -> Option<AsWidgetsDynResolveResult<'a,'b,Self::ChildID,E>> {
        if let Some(v) = path.try_fragment::<Self::ChildID>() {
            let res = self.0.iter().enumerate()
                .find(#[inline] |(_,(i,_))| *i == *v);

            if let Some((idx,(id,inner))) = res {
                return Some(AsWidgetsDynResolveResult {
                    widget: inner,
                    widget_id: inner.id(),
                    child_id: id.clone(),
                    idx: idx as isize,
                    resolvus: path.inner().unwrap(),
                });
            }
        }

        None
    }

    fn resolve_dyn_mut<'a,'b>(&'a mut self, path: &'b (dyn PathResolvusDyn<E>+'b)) -> Option<AsWidgetsDynResolveResultMut<'a,'b,Self::ChildID,E>> {
        if let Some(v) = path.try_fragment::<Self::ChildID>() {
            let res = self.0.iter_mut().enumerate()
                .find(#[inline] |(_,(i,_))| *i == *v);

            if let Some((idx,(id,inner))) = res {
                return Some(AsWidgetsDynResolveResultMut {
                    widget_id: inner.id(),
                    widget: inner,
                    child_id: id.clone(),
                    idx: idx as isize,
                    resolvus: path.inner().unwrap(),
                });
            }
        }

        None
    }
}
