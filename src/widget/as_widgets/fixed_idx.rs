use std::mem::MaybeUninit;
use std::ops::{Range, Deref, DerefMut};

use crate::dispatchor::*;
use crate::env::Env;
use crate::newpath::{PathResolvusDyn, FixedIdx, PathResolvus};
use crate::root::RootRef;
use crate::widget::{WidgetDyn, Widget};

use super::{AsWidgets, AsWidgetsDyn, AsWidgetsDynResult, AsWidgetsDynResultMut, AsWidgetsDynResolveResult, AsWidgetsDynResolveResultMut};

mod impl_tuple;

#[repr(transparent)]
pub struct WidgetsFixedIdx<T>(pub T) where T: ?Sized;

impl<T> Deref for WidgetsFixedIdx<T> where T: ?Sized {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for WidgetsFixedIdx<T> where T: ?Sized {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<E,T> AsWidgets<E> for WidgetsFixedIdx<Vec<T>> where T: Widget<E>, E: Env {
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
            Some(inner) => {
                callback.call(AsWidgetsResult::from_some(idx,FixedIdx(idx),inner), root, ctx)
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
            Some(inner) => {
                callback.call(AsWidgetsResultMut::from_some(idx,FixedIdx(idx),inner), root, ctx)
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
            Some(inner) => {
                let cache_slot = unsafe { cache.get_unchecked_mut(idx as usize) };
                callback.call(AsWidgetsCResult::from_some(idx,FixedIdx(idx),inner, cache_slot), root, ctx)
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
            Some(inner) => {
                let cache_slot = unsafe { cache.get_unchecked_mut(idx as usize) };
                callback.call(AsWidgetsCResultMut::from_some(idx,FixedIdx(idx),inner, cache_slot), root, ctx)
            },
            None => callback.call_none(root,ctx),
        }
    }

    #[inline]
    fn iter_ids(&self) -> Self::IdIdxIter {
        (0..self.len()).map(#[inline] |i| (i as isize,FixedIdx(i as isize)))
    }

    #[inline]
    fn idx_range<F>(&self, idx_range: Range<isize>, mut callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedDispatch<Self::ChildID,E>
    {
        for (i,v) in self.0[idx_range.start as usize .. idx_range.end as usize].iter().enumerate() {
            callback.call(i as isize, FixedIdx(i as isize), v, root.fork(), ctx)
        }
    }

    #[inline]
    fn idx_range_mut<F>(&mut self, idx_range: Range<isize>, mut callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedDispatchMut<Self::ChildID,E>
    {
        for (i,v) in self.0[idx_range.start as usize .. idx_range.end as usize].iter_mut().enumerate() {
            callback.call(i as isize, FixedIdx(i as isize), v, root.fork(), ctx)
        }
    }

    #[inline]
    fn idx_range_c<F>(&self, idx_range: Range<isize>, mut callback: F, cache: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedCDispatch<Self::ChildID,E>
    {
        if self.0.len() != cache.len() {
            cache.resize_with(self.0.len(), Default::default);
        }

        for (i,v) in self.0[idx_range.start as usize .. idx_range.end as usize].iter().enumerate() {
            let cache_slot = unsafe { cache.get_unchecked_mut(i) };
            callback.call(i as isize, FixedIdx(i as isize), v, cache_slot, root.fork(), ctx)
        }
    }

    #[inline]
    fn idx_range_c_mut<F>(&mut self, idx_range: Range<isize>, mut callback: F, cache: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedCDispatchMut<Self::ChildID,E>
    {
        if self.0.len() != cache.len() {
            cache.resize_with(self.0.len(), Default::default);
        }

        for (i,v) in self.0[idx_range.start as usize .. idx_range.end as usize].iter_mut().enumerate() {
            let cache_slot = unsafe { cache.get_unchecked_mut(i) };
            callback.call(i as isize, FixedIdx(i as isize), v, cache_slot, root.fork(), ctx)
        }
    }

    #[inline]
    fn resolve<F,R>(&self, path: &(dyn PathResolvusDyn<E>+'_), callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsResolveDispatch<Self::ChildID,R,E>
    {
        if let Some(v) = path.try_fragment::<Self::ChildID>() {
            let idx = v.0;
            if let Some(inner) = self.0.get(idx as usize) {
                return callback.call(AsWidgetsResolveResult::from_some(idx,FixedIdx(idx),path.inner().unwrap(),inner), root, ctx);
            }
        }

        callback.call_none(root,ctx)
    }

    #[inline]
    fn resolve_mut<F,R>(&mut self, path: &(dyn PathResolvusDyn<E>+'_), callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsResolveDispatchMut<Self::ChildID,R,E>
    {
        if let Some(v) = path.try_fragment::<Self::ChildID>() {
            let idx = v.0;
            if let Some(inner) = self.0.get_mut(idx as usize) {
                return callback.call(AsWidgetsResolveResultMut::from_some(idx,FixedIdx(idx),path.inner().unwrap(),inner), root, ctx);
            }
        }

        callback.call_none(root,ctx)
    }

    #[inline]
    fn resolve_c<F,R>(&self, path: &(dyn PathResolvusDyn<E>+'_), callback: F, cache: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsResolveCDispatch<Self::ChildID,R,E>
    {
        if self.0.len() != cache.len() {
            cache.resize_with(self.0.len(), Default::default);
        }

        if let Some(v) = path.try_fragment::<Self::ChildID>() {
            let idx = v.0;
            if let Some(inner) = self.0.get(idx as usize) {
                let cache_slot = unsafe { cache.get_unchecked_mut(idx as usize) };
                return callback.call(AsWidgetsResolveCResult::from_some(idx,FixedIdx(idx),path.inner().unwrap(),inner, cache_slot), root, ctx);
            }
        }

        callback.call_none(root,ctx)
    }

    #[inline]
    fn resolve_c_mut<F,R>(&mut self, path: &(dyn PathResolvusDyn<E>+'_), callback: F, cache: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsResolveCDispatchMut<Self::ChildID,R,E>
    {
        if self.0.len() != cache.len() {
            cache.resize_with(self.0.len(), Default::default);
        }

        if let Some(v) = path.try_fragment::<Self::ChildID>() {
            let idx = v.0;
            if let Some(inner) = self.0.get_mut(idx as usize) {
                let cache_slot = unsafe { cache.get_unchecked_mut(idx as usize) };
                return callback.call(AsWidgetsResolveCResultMut::from_some(idx,FixedIdx(idx),path.inner().unwrap(),inner, cache_slot), root, ctx);
            }
        }

        callback.call_none(root,ctx)
    }
}

impl<E,T,const N: usize> AsWidgets<E> for WidgetsFixedIdx<[T;N]> where T: Widget<E>, E: Env {
    type Caches = DefaultHack<[T::Cache;N]>;
    type IdIdxIter = impl Iterator<Item=(isize,Self::ChildID)>;
    type IndexedSideData<TT> = DefaultHack<[TT;N]> where TT: Clone + Default;
    type IndexedSideData2<TT> = Self::IndexedSideData<TT> where TT: Clone + Default + 'static;

    #[inline]
    fn by_index<F,R>(&self, idx: isize, callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsDispatch<Self::ChildID,R,E>
    {
        match self.0.get(idx as usize) {
            Some(inner) => {
                callback.call(AsWidgetsResult::from_some(idx,FixedIdx(idx),inner ), root, ctx)
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
            Some(inner) => {
                callback.call(AsWidgetsResultMut::from_some(idx,FixedIdx(idx),inner), root, ctx)
            },
            None => callback.call_none(root,ctx),
        }
    }

    #[inline]
    fn by_index_c<F,R>(&self, idx: isize, callback: F, cache: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsCDispatch<Self::ChildID,R,E>
    {
        match self.0.get(idx as usize) {
            Some(inner) => {
                let cache_slot = unsafe { cache.0.get_unchecked_mut(idx as usize) };
                callback.call(AsWidgetsCResult::from_some(idx,FixedIdx(idx),inner, cache_slot), root, ctx)
            },
            None => callback.call_none(root,ctx),
        }
    }

    #[inline]
    fn by_index_c_mut<F,R>(&mut self, idx: isize, callback: F, cache: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsCDispatchMut<Self::ChildID,R,E>
    {
        match self.0.get_mut(idx as usize) {
            Some(inner) => {
                let cache_slot = unsafe { cache.0.get_unchecked_mut(idx as usize) };
                callback.call(AsWidgetsCResultMut::from_some(idx,FixedIdx(idx),inner, cache_slot), root, ctx)
            },
            None => callback.call_none(root,ctx),
        }
    }

    #[inline]
    fn iter_ids(&self) -> Self::IdIdxIter {
        (0..N).map(#[inline] |i| (i as isize,FixedIdx(i as isize)))
    }

    #[inline]
    fn idx_range<F>(&self, idx_range: Range<isize>, mut callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedDispatch<Self::ChildID,E>
    {
        for (i,v) in self.0[idx_range.start as usize .. idx_range.end as usize].iter().enumerate() {
            callback.call(i as isize, FixedIdx(i as isize), v, root.fork(), ctx)
        }
    }

    #[inline]
    fn idx_range_mut<F>(&mut self, idx_range: Range<isize>, mut callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedDispatchMut<Self::ChildID,E>
    {
        for (i,v) in self.0[idx_range.start as usize .. idx_range.end as usize].iter_mut().enumerate() {
            callback.call(i as isize, FixedIdx(i as isize), v, root.fork(), ctx)
        }
    }

    #[inline]
    fn idx_range_c<F>(&self, idx_range: Range<isize>, mut callback: F, cache: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedCDispatch<Self::ChildID,E>
    {
        for (i,v) in self.0[idx_range.start as usize .. idx_range.end as usize].iter().enumerate() {
            let cache_slot = unsafe { cache.0.get_unchecked_mut(i) };
            callback.call(i as isize, FixedIdx(i as isize), v, cache_slot, root.fork(), ctx)
        }
    }

    #[inline]
    fn idx_range_c_mut<F>(&mut self, idx_range: Range<isize>, mut callback: F, cache: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedCDispatchMut<Self::ChildID,E>
    {
        for (i,v) in self.0[idx_range.start as usize .. idx_range.end as usize].iter_mut().enumerate() {
            let cache_slot = unsafe { cache.0.get_unchecked_mut(i) };
            callback.call(i as isize, FixedIdx(i as isize), v, cache_slot, root.fork(), ctx)
        }
    }

    fn resolve<F,R>(&self, path: &(dyn PathResolvusDyn<E>+'_), callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsResolveDispatch<Self::ChildID,R,E>
    {
        if let Some(v) = path.try_fragment::<Self::ChildID>() {
            let idx = v.0;
            if let Some(inner) = self.0.get(idx as usize) {
                return callback.call(AsWidgetsResolveResult::from_some(idx,FixedIdx(idx),path.inner().unwrap(),inner), root, ctx);
            }
        }

        callback.call_none(root,ctx)
    }

    #[inline]
    fn resolve_mut<F,R>(&mut self, path: &(dyn PathResolvusDyn<E>+'_), callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsResolveDispatchMut<Self::ChildID,R,E>
    {
        if let Some(v) = path.try_fragment::<Self::ChildID>() {
            let idx = v.0;
            if let Some(inner) = self.0.get_mut(idx as usize) {
                return callback.call(AsWidgetsResolveResultMut::from_some(idx,FixedIdx(idx),path.inner().unwrap(),inner), root, ctx);
            }
        }

        callback.call_none(root,ctx)
    }

    fn resolve_c<F,R>(&self, path: &(dyn PathResolvusDyn<E>+'_), callback: F, cache: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsResolveCDispatch<Self::ChildID,R,E>
    {
        if let Some(v) = path.try_fragment::<Self::ChildID>() {
            let idx = v.0;
            if let Some(inner) = self.0.get(idx as usize) {
                let cache_slot = unsafe { cache.0.get_unchecked_mut(idx as usize) };
                return callback.call(AsWidgetsResolveCResult::from_some(idx,FixedIdx(idx),path.inner().unwrap(),inner, cache_slot), root, ctx);
            }
        }

        callback.call_none(root,ctx)
    }

    #[inline]
    fn resolve_c_mut<F,R>(&mut self, path: &(dyn PathResolvusDyn<E>+'_), callback: F, cache: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsResolveCDispatchMut<Self::ChildID,R,E>
    {
        if let Some(v) = path.try_fragment::<Self::ChildID>() {
            let idx = v.0;
            if let Some(inner) = self.0.get_mut(idx as usize) {
                let cache_slot = unsafe { cache.0.get_unchecked_mut(idx as usize) };
                return callback.call(AsWidgetsResolveCResultMut::from_some(idx,FixedIdx(idx),path.inner().unwrap(),inner, cache_slot), root, ctx);
            }
        }

        callback.call_none(root,ctx)
    }
}

impl<E,T> AsWidgetsDyn<E> for WidgetsFixedIdx<Vec<T>> where T: Widget<E>, E: Env {
    type ChildID = FixedIdx;

    fn by_index_dyn(&self, idx: isize) -> Option<AsWidgetsDynResult<'_,Self::ChildID,E>> {
        self.0.get(idx as usize).map(|widget| AsWidgetsDynResult {
            widget,
            widget_id: widget.id(),
            child_id: FixedIdx(idx),
            idx,
        })
    }

    fn by_index_dyn_mut(&mut self, idx: isize) -> Option<AsWidgetsDynResultMut<'_,Self::ChildID,E>> {
        self.0.get_mut(idx as usize).map(|widget| AsWidgetsDynResultMut {
            widget_id: widget.id(),
            widget,
            child_id: FixedIdx(idx),
            idx,
        })
    }

    fn range(&self) -> Range<isize> {
        0 .. self.0.len() as isize
    }

    fn idx_range_dyn<'a>(&'a self, range: Range<isize>, callback: &mut (dyn FnMut(AsWidgetsDynResult<'a,Self::ChildID,E>) + '_) ) {
        for (idx,widget) in self.0[range.start as usize .. range.end as usize].iter().enumerate() {
            callback(AsWidgetsDynResult {
                widget,
                widget_id: widget.id(),
                child_id: FixedIdx(idx as isize),
                idx: idx as isize,
            })
        }
    }

    fn idx_range_dyn_mut<'a>(&'a mut self, range: Range<isize>, callback: &mut (dyn FnMut(AsWidgetsDynResultMut<'a,Self::ChildID,E>) + '_) ) {
        for (idx,widget) in self.0[range.start as usize .. range.end as usize].iter_mut().enumerate() {
            callback(AsWidgetsDynResultMut {
                widget_id: widget.id(),
                widget,
                child_id: FixedIdx(idx as isize),
                idx: idx as isize,
            })
        }
    }

    fn resolve_dyn<'a,'b>(&'a self, path: &'b (dyn PathResolvusDyn<E>+'b)) -> Option<AsWidgetsDynResolveResult<'a,'b,Self::ChildID,E>> {
        if let Some(v) = path.try_fragment::<Self::ChildID>() {
            let idx = v.0;
            if let Some(widget) = self.0.get(idx as usize) {
                return Some(AsWidgetsDynResolveResult {
                    widget_id: widget.id(),
                    widget,
                    child_id: FixedIdx(idx),
                    idx,
                    resolvus: path.inner().unwrap(),
                });
            }
        }

        None
    }

    fn resolve_dyn_mut<'a,'b>(&'a mut self, path: &'b (dyn PathResolvusDyn<E>+'b)) -> Option<AsWidgetsDynResolveResultMut<'a,'b,Self::ChildID,E>> {
        if let Some(v) = path.try_fragment::<Self::ChildID>() {
            let idx = v.0;
            if let Some(widget) = self.0.get_mut(idx as usize) {
                return Some(AsWidgetsDynResolveResultMut {
                    widget_id: widget.id(),
                    widget,
                    child_id: FixedIdx(idx),
                    idx,
                    resolvus: path.inner().unwrap(),
                });
            }
        }

        None
    }
}

impl<E,T,const N: usize> AsWidgetsDyn<E> for WidgetsFixedIdx<[T;N]> where T: Widget<E>, E: Env {
    type ChildID = FixedIdx;

    fn by_index_dyn(&self, idx: isize) -> Option<AsWidgetsDynResult<'_,Self::ChildID,E>> {
        self.0.get(idx as usize).map(|widget| AsWidgetsDynResult {
            widget,
            widget_id: widget.id(),
            child_id: FixedIdx(idx),
            idx,
        })
    }

    fn by_index_dyn_mut(&mut self, idx: isize) -> Option<AsWidgetsDynResultMut<'_,Self::ChildID,E>> {
        self.0.get_mut(idx as usize).map(|widget| AsWidgetsDynResultMut {
            widget_id: widget.id(),
            widget,
            child_id: FixedIdx(idx),
            idx,
        })
    }

    fn range(&self) -> Range<isize> {
        0 .. N as isize
    }

    fn idx_range_dyn<'a>(&'a self, range: Range<isize>, callback: &mut (dyn FnMut(AsWidgetsDynResult<'a,Self::ChildID,E>) + '_) ) {
        for (idx,widget) in self.0[range.start as usize .. range.end as usize].iter().enumerate() {
            callback(AsWidgetsDynResult {
                widget,
                widget_id: widget.id(),
                child_id: FixedIdx(idx as isize),
                idx: idx as isize,
            })
        }
    }

    fn idx_range_dyn_mut<'a>(&'a mut self, range: Range<isize>, callback: &mut (dyn FnMut(AsWidgetsDynResultMut<'a,Self::ChildID,E>) + '_) ) {
        for (idx,widget) in self.0[range.start as usize .. range.end as usize].iter_mut().enumerate() {
            callback(AsWidgetsDynResultMut {
                widget_id: widget.id(),
                widget,
                child_id: FixedIdx(idx as isize),
                idx: idx as isize,
            })
        }
    }

    fn resolve_dyn<'a,'b>(&'a self, path: &'b (dyn PathResolvusDyn<E>+'b)) -> Option<AsWidgetsDynResolveResult<'a,'b,Self::ChildID,E>> {
        if let Some(v) = path.try_fragment::<Self::ChildID>() {
            let idx = v.0;
            if let Some(widget) = self.0.get(idx as usize) {
                return Some(AsWidgetsDynResolveResult {
                    widget,
                    widget_id: widget.id(),
                    child_id: FixedIdx(idx),
                    idx,
                    resolvus: path.inner().unwrap(),
                });
            }
        }

        None
    }

    fn resolve_dyn_mut<'a,'b>(&'a mut self, path: &'b (dyn PathResolvusDyn<E>+'b)) -> Option<AsWidgetsDynResolveResultMut<'a,'b,Self::ChildID,E>> {
        if let Some(v) = path.try_fragment::<Self::ChildID>() {
            let idx = v.0;
            if let Some(widget) = self.0.get_mut(idx as usize) {
                return Some(AsWidgetsDynResolveResultMut {
                    widget_id: widget.id(),
                    widget,
                    child_id: FixedIdx(idx),
                    idx,
                    resolvus: path.inner().unwrap(),
                });
            }
        }

        None
    }
}

pub struct DefaultHack<T>(T);

impl<T,const N: usize> Default for DefaultHack<[T;N]> where T: Default {
    #[inline]
    fn default() -> Self {
        unsafe { 
            let mut dest: MaybeUninit<[T;N]> = MaybeUninit::uninit();
            for entry in &mut *(dest.as_mut_ptr() as *mut [MaybeUninit<T>;N]) {
                entry.write(T::default());
            }
            Self(dest.assume_init())
        }
    }
}

impl<T,const N: usize> AsRef<[T]> for DefaultHack<[T;N]> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        &self.0[..]
    }
}
impl<T,const N: usize> AsMut<[T]> for DefaultHack<[T;N]> {
    #[inline]
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.0[..]
    }
}

impl<T,const N: usize> Clone for DefaultHack<[T;N]> where T: Clone {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
