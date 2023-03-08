use std::mem::MaybeUninit;
use std::ops::{Range, Deref, DerefMut};

use crate::env::Env;
use crate::newpath::{PathResolvusDyn, FixedIdx, PathResolvus};
use crate::root::RootRef;
use crate::widget::{WidgetDyn, Widget};

use super::{PaneChildWidget, PaneChilds, PaneChildsDyn, ChildWidgetDynResult, ChildWidgetDynResultMut};

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

impl<E,T> PaneChilds<E> for WidgetsFixedIdx<Vec<PaneChildWidget<T,E>>> where T: Widget<E>, E: Env {
    type Caches = Vec<T::Cache>;
    
    // #[inline]
    // fn by_index<F,R>(&self, idx: usize, callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    // where
    //     F: AsWidgetsDispatch<Self::ChildID,R,E>
    // {
    //     match self.0.get(idx as usize) {
    //         Some(inner) => {
    //             callback.call(AsWidgetsResult::from_some(idx,FixedIdx(idx as isize),inner), root, ctx)
    //         },
    //         None => callback.call_none(root,ctx),
    //     }
    // }

    // #[inline]
    // fn by_index_mut<F,R>(&mut self, idx: usize, callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    // where
    //     F: AsWidgetsDispatchMut<Self::ChildID,R,E>
    // {
    //     match self.0.get_mut(idx as usize) {
    //         Some(inner) => {
    //             callback.call(AsWidgetsResultMut::from_some(idx,FixedIdx(idx as isize),inner), root, ctx)
    //         },
    //         None => callback.call_none(root,ctx),
    //     }
    // }

    // #[inline]
    // fn by_index_c<F,R>(&self, idx: usize, callback: F, cache: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    // where
    //     F: AsWidgetsCDispatch<Self::ChildID,R,E>
    // {
    //     if self.0.len() != cache.len() {
    //         cache.resize_with(self.0.len(), Default::default);
    //     }

    //     match self.0.get(idx as usize) {
    //         Some(inner) => {
    //             let cache_slot = unsafe { cache.get_unchecked_mut(idx as usize) };
    //             callback.call(AsWidgetsCResult::from_some(idx,FixedIdx(idx as isize),inner, cache_slot), root, ctx)
    //         },
    //         None => callback.call_none(root,ctx),
    //     }
    // }

    // #[inline]
    // fn by_index_c_mut<F,R>(&mut self, idx: usize, callback: F, cache: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    // where
    //     F: AsWidgetsCDispatchMut<Self::ChildID,R,E>
    // {
    //     if self.0.len() != cache.len() {
    //         cache.resize_with(self.0.len(), Default::default);
    //     }

    //     match self.0.get_mut(idx as usize) {
    //         Some(inner) => {
    //             let cache_slot = unsafe { cache.get_unchecked_mut(idx as usize) };
    //             callback.call(AsWidgetsCResultMut::from_some(idx,FixedIdx(idx as isize),inner, cache_slot), root, ctx)
    //         },
    //         None => callback.call_none(root,ctx),
    //     }
    // }

    // #[inline]
    // fn iter_ids(&self) -> Self::IdIdxIter {
    //     (0..self.len()).map(#[inline] |i| (i,FixedIdx(i)))
    // }

    // #[inline]
    // fn idx_range<F>(&self, idx_range: Range<usize>, mut callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    // where
    //     F: AsWidgetsIndexedDispatch<Self::ChildID,E>
    // {
    //     for (i,v) in self.0[idx_range.start as usize .. idx_range.end as usize].iter().enumerate() {
    //         callback.call(i, FixedIdx(i), v, root.fork(), ctx)
    //     }
    // }

    // #[inline]
    // fn idx_range_mut<F>(&mut self, idx_range: Range<usize>, mut callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    // where
    //     F: AsWidgetsIndexedDispatchMut<Self::ChildID,E>
    // {
    //     for (i,v) in self.0[idx_range.start as usize .. idx_range.end as usize].iter_mut().enumerate() {
    //         callback.call(i, FixedIdx(i), v, root.fork(), ctx)
    //     }
    // }

    // #[inline]
    // fn idx_range_c<F>(&self, idx_range: Range<usize>, mut callback: F, cache: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    // where
    //     F: AsWidgetsIndexedCDispatch<Self::ChildID,E>
    // {
    //     if self.0.len() != cache.len() {
    //         cache.resize_with(self.0.len(), Default::default);
    //     }

    //     for (i,v) in self.0[idx_range.start as usize .. idx_range.end as usize].iter().enumerate() {
    //         let cache_slot = unsafe { cache.get_unchecked_mut(i) };
    //         callback.call(i, FixedIdx(i), v, cache_slot, root.fork(), ctx)
    //     }
    // }

    // #[inline]
    // fn idx_range_c_mut<F>(&mut self, idx_range: Range<usize>, mut callback: F, cache: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    // where
    //     F: AsWidgetsIndexedCDispatchMut<Self::ChildID,E>
    // {
    //     if self.0.len() != cache.len() {
    //         cache.resize_with(self.0.len(), Default::default);
    //     }

    //     for (i,v) in self.0[idx_range.start as usize .. idx_range.end as usize].iter_mut().enumerate() {
    //         let cache_slot = unsafe { cache.get_unchecked_mut(i) };
    //         callback.call(i, FixedIdx(i), v, cache_slot, root.fork(), ctx)
    //     }
    // }

    // #[inline]
    // fn resolve<F,R>(&self, path: &(dyn PathResolvusDyn<E>+'_), callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    // where
    //     F: AsWidgetsResolveDispatch<Self::ChildID,R,E>
    // {
    //     if let Some(v) = path.try_fragment::<Self::ChildID>() {
    //         let idx = v.0;
    //         if let Some(inner) = self.0.get(idx as usize) {
    //             return callback.call(AsWidgetsResolveResult::from_some(idx,FixedIdx(idx as isize),path.inner().unwrap(),inner), root, ctx);
    //         }
    //     }

    //     callback.call_none(root,ctx)
    // }

    // #[inline]
    // fn resolve_mut<F,R>(&mut self, path: &(dyn PathResolvusDyn<E>+'_), callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    // where
    //     F: AsWidgetsResolveDispatchMut<Self::ChildID,R,E>
    // {
    //     if let Some(v) = path.try_fragment::<Self::ChildID>() {
    //         let idx = v.0;
    //         if let Some(inner) = self.0.get_mut(idx as usize) {
    //             return callback.call(AsWidgetsResolveResultMut::from_some(idx,FixedIdx(idx as isize),path.inner().unwrap(),inner), root, ctx);
    //         }
    //     }

    //     callback.call_none(root,ctx)
    // }

    // #[inline]
    // fn resolve_c<F,R>(&self, path: &(dyn PathResolvusDyn<E>+'_), callback: F, cache: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    // where
    //     F: AsWidgetsResolveCDispatch<Self::ChildID,R,E>
    // {
    //     if self.0.len() != cache.len() {
    //         cache.resize_with(self.0.len(), Default::default);
    //     }

    //     if let Some(v) = path.try_fragment::<Self::ChildID>() {
    //         let idx = v.0;
    //         if let Some(inner) = self.0.get(idx as usize) {
    //             let cache_slot = unsafe { cache.get_unchecked_mut(idx as usize) };
    //             return callback.call(AsWidgetsResolveCResult::from_some(idx,FixedIdx(idx as isize),path.inner().unwrap(),inner, cache_slot), root, ctx);
    //         }
    //     }

    //     callback.call_none(root,ctx)
    // }

    // #[inline]
    // fn resolve_c_mut<F,R>(&mut self, path: &(dyn PathResolvusDyn<E>+'_), callback: F, cache: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    // where
    //     F: AsWidgetsResolveCDispatchMut<Self::ChildID,R,E>
    // {
    //     if self.0.len() != cache.len() {
    //         cache.resize_with(self.0.len(), Default::default);
    //     }

    //     if let Some(v) = path.try_fragment::<Self::ChildID>() {
    //         let idx = v.0;
    //         if let Some(inner) = self.0.get_mut(idx as usize) {
    //             let cache_slot = unsafe { cache.get_unchecked_mut(idx as usize) };
    //             return callback.call(AsWidgetsResolveCResultMut::from_some(idx,FixedIdx(idx as isize),path.inner().unwrap(),inner, cache_slot), root, ctx);
    //         }
    //     }

    //     callback.call_none(root,ctx)
    // }
}

impl<E,T,const N: usize> PaneChilds<E> for WidgetsFixedIdx<[PaneChildWidget<T,E>;N]> where T: Widget<E>, E: Env {
    type Caches = DefaultHack<[T::Cache;N]>;

    // #[inline]
    // fn by_index<F,R>(&self, idx: usize, callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    // where
    //     F: AsWidgetsDispatch<Self::ChildID,R,E>
    // {
    //     match self.0.get(idx as usize) {
    //         Some(inner) => {
    //             callback.call(AsWidgetsResult::from_some(idx,FixedIdx(idx as isize),inner ), root, ctx)
    //         },
    //         None => callback.call_none(root,ctx),
    //     }
    // }

    // #[inline]
    // fn by_index_mut<F,R>(&mut self, idx: usize, callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    // where
    //     F: AsWidgetsDispatchMut<Self::ChildID,R,E>
    // {
    //     match self.0.get_mut(idx as usize) {
    //         Some(inner) => {
    //             callback.call(AsWidgetsResultMut::from_some(idx,FixedIdx(idx as isize),inner), root, ctx)
    //         },
    //         None => callback.call_none(root,ctx),
    //     }
    // }

    // #[inline]
    // fn by_index_c<F,R>(&self, idx: usize, callback: F, cache: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    // where
    //     F: AsWidgetsCDispatch<Self::ChildID,R,E>
    // {
    //     match self.0.get(idx as usize) {
    //         Some(inner) => {
    //             let cache_slot = unsafe { cache.0.get_unchecked_mut(idx as usize) };
    //             callback.call(AsWidgetsCResult::from_some(idx,FixedIdx(idx as isize),inner, cache_slot), root, ctx)
    //         },
    //         None => callback.call_none(root,ctx),
    //     }
    // }

    // #[inline]
    // fn by_index_c_mut<F,R>(&mut self, idx: usize, callback: F, cache: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    // where
    //     F: AsWidgetsCDispatchMut<Self::ChildID,R,E>
    // {
    //     match self.0.get_mut(idx as usize) {
    //         Some(inner) => {
    //             let cache_slot = unsafe { cache.0.get_unchecked_mut(idx as usize) };
    //             callback.call(AsWidgetsCResultMut::from_some(idx,FixedIdx(idx as isize),inner, cache_slot), root, ctx)
    //         },
    //         None => callback.call_none(root,ctx),
    //     }
    // }

    // #[inline]
    // fn iter_ids(&self) -> Self::IdIdxIter {
    //     (0..N).map(#[inline] |i| (i,FixedIdx(i)))
    // }

    // #[inline]
    // fn idx_range<F>(&self, idx_range: Range<usize>, mut callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    // where
    //     F: AsWidgetsIndexedDispatch<Self::ChildID,E>
    // {
    //     for (i,v) in self.0[idx_range.start as usize .. idx_range.end as usize].iter().enumerate() {
    //         callback.call(i, FixedIdx(i), v, root.fork(), ctx)
    //     }
    // }

    // #[inline]
    // fn idx_range_mut<F>(&mut self, idx_range: Range<usize>, mut callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    // where
    //     F: AsWidgetsIndexedDispatchMut<Self::ChildID,E>
    // {
    //     for (i,v) in self.0[idx_range.start as usize .. idx_range.end as usize].iter_mut().enumerate() {
    //         callback.call(i, FixedIdx(i), v, root.fork(), ctx)
    //     }
    // }

    // #[inline]
    // fn idx_range_c<F>(&self, idx_range: Range<usize>, mut callback: F, cache: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    // where
    //     F: AsWidgetsIndexedCDispatch<Self::ChildID,E>
    // {
    //     for (i,v) in self.0[idx_range.start as usize .. idx_range.end as usize].iter().enumerate() {
    //         let cache_slot = unsafe { cache.0.get_unchecked_mut(i) };
    //         callback.call(i, FixedIdx(i), v, cache_slot, root.fork(), ctx)
    //     }
    // }

    // #[inline]
    // fn idx_range_c_mut<F>(&mut self, idx_range: Range<usize>, mut callback: F, cache: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    // where
    //     F: AsWidgetsIndexedCDispatchMut<Self::ChildID,E>
    // {
    //     for (i,v) in self.0[idx_range.start as usize .. idx_range.end as usize].iter_mut().enumerate() {
    //         let cache_slot = unsafe { cache.0.get_unchecked_mut(i) };
    //         callback.call(i, FixedIdx(i), v, cache_slot, root.fork(), ctx)
    //     }
    // }

    // fn resolve<F,R>(&self, path: &(dyn PathResolvusDyn<E>+'_), callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    // where
    //     F: AsWidgetsResolveDispatch<Self::ChildID,R,E>
    // {
    //     if let Some(v) = path.try_fragment::<Self::ChildID>() {
    //         let idx = v.0;
    //         if let Some(inner) = self.0.get(idx as usize) {
    //             return callback.call(AsWidgetsResolveResult::from_some(idx,FixedIdx(idx as isize),path.inner().unwrap(),inner), root, ctx);
    //         }
    //     }

    //     callback.call_none(root,ctx)
    // }

    // #[inline]
    // fn resolve_mut<F,R>(&mut self, path: &(dyn PathResolvusDyn<E>+'_), callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    // where
    //     F: AsWidgetsResolveDispatchMut<Self::ChildID,R,E>
    // {
    //     if let Some(v) = path.try_fragment::<Self::ChildID>() {
    //         let idx = v.0;
    //         if let Some(inner) = self.0.get_mut(idx as usize) {
    //             return callback.call(AsWidgetsResolveResultMut::from_some(idx,FixedIdx(idx as isize),path.inner().unwrap(),inner), root, ctx);
    //         }
    //     }

    //     callback.call_none(root,ctx)
    // }

    // fn resolve_c<F,R>(&self, path: &(dyn PathResolvusDyn<E>+'_), callback: F, cache: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    // where
    //     F: AsWidgetsResolveCDispatch<Self::ChildID,R,E>
    // {
    //     if let Some(v) = path.try_fragment::<Self::ChildID>() {
    //         let idx = v.0;
    //         if let Some(inner) = self.0.get(idx as usize) {
    //             let cache_slot = unsafe { cache.0.get_unchecked_mut(idx as usize) };
    //             return callback.call(AsWidgetsResolveCResult::from_some(idx,FixedIdx(idx as isize),path.inner().unwrap(),inner, cache_slot), root, ctx);
    //         }
    //     }

    //     callback.call_none(root,ctx)
    // }

    // #[inline]
    // fn resolve_c_mut<F,R>(&mut self, path: &(dyn PathResolvusDyn<E>+'_), callback: F, cache: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    // where
    //     F: AsWidgetsResolveCDispatchMut<Self::ChildID,R,E>
    // {
    //     if let Some(v) = path.try_fragment::<Self::ChildID>() {
    //         let idx = v.0;
    //         if let Some(inner) = self.0.get_mut(idx as usize) {
    //             let cache_slot = unsafe { cache.0.get_unchecked_mut(idx as usize) };
    //             return callback.call(AsWidgetsResolveCResultMut::from_some(idx,FixedIdx(idx as isize),path.inner().unwrap(),inner, cache_slot), root, ctx);
    //         }
    //     }

    //     callback.call_none(root,ctx)
    // }
}

impl<E,T> PaneChildsDyn<E> for WidgetsFixedIdx<Vec<PaneChildWidget<T,E>>> where T: Widget<E>, E: Env {
    type ChildID = FixedIdx;

    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }

    fn by_index_dyn(&self, idx: usize) -> Option<ChildWidgetDynResult<'_,Self::ChildID,E>> {
        self.0.get(idx as usize).map(|widget| ChildWidgetDynResult {
            widget: &widget.widget,
            widget_id: widget.widget.id(),
            child_id: FixedIdx(idx as isize),
            idx,
        })
    }

    fn by_index_dyn_mut(&mut self, idx: usize) -> Option<ChildWidgetDynResultMut<'_,Self::ChildID,E>> {
        self.0.get_mut(idx as usize).map(|widget| ChildWidgetDynResultMut {
            widget_id: widget.widget.id(),
            widget: &mut widget.widget,
            child_id: FixedIdx(idx as isize),
            idx,
        })
    }

    fn idx_range_dyn<'a>(&'a self, range: Range<usize>, callback: &mut (dyn FnMut(ChildWidgetDynResult<'a,Self::ChildID,E>) + '_) ) {
        for (idx,widget) in self.0[range.start as usize .. range.end as usize].iter().enumerate() {
            callback(ChildWidgetDynResult {
                widget: &widget.widget,
                widget_id: widget.widget.id(),
                child_id: FixedIdx(idx as isize),
                idx: idx,
            })
        }
    }

    fn idx_range_dyn_mut<'a>(&'a mut self, range: Range<usize>, callback: &mut (dyn FnMut(ChildWidgetDynResultMut<'a,Self::ChildID,E>) + '_) ) {
        for (idx,widget) in self.0[range.start as usize .. range.end as usize].iter_mut().enumerate() {
            callback(ChildWidgetDynResultMut {
                widget_id: widget.widget.id(),
                widget: &mut widget.widget,
                child_id: FixedIdx(idx as isize),
                idx: idx,
            })
        }
    }

    // fn resolve_dyn<'a,'b>(&'a self, path: &'b (dyn PathResolvusDyn<E>+'b)) -> Option<ChildWidgetDynResolveResult<'a,'b,Self::ChildID,E>> {
    //     if let Some(v) = path.try_fragment::<Self::ChildID>() {
    //         let idx = v.0;
    //         if let Some(widget) = self.0.get(idx as usize) {
    //             return Some(ChildWidgetDynResolveResult {
    //                 widget_id: widget.id(),
    //                 widget,
    //                 child_id: FixedIdx(idx as isize),
    //                 idx,
    //                 resolvus: path.inner().unwrap(),
    //             });
    //         }
    //     }

    //     None
    // }

    // fn resolve_dyn_mut<'a,'b>(&'a mut self, path: &'b (dyn PathResolvusDyn<E>+'b)) -> Option<ChildWidgetDynResolveResultMut<'a,'b,Self::ChildID,E>> {
    //     if let Some(v) = path.try_fragment::<Self::ChildID>() {
    //         let idx = v.0;
    //         if let Some(widget) = self.0.get_mut(idx as usize) {
    //             return Some(ChildWidgetDynResolveResultMut {
    //                 widget_id: widget.id(),
    //                 widget,
    //                 child_id: FixedIdx(idx as isize),
    //                 idx,
    //                 resolvus: path.inner().unwrap(),
    //             });
    //         }
    //     }

    //     None
    // }
}

impl<E,T,const N: usize> PaneChildsDyn<E> for WidgetsFixedIdx<[PaneChildWidget<T,E>;N]> where T: Widget<E>, E: Env {
    type ChildID = FixedIdx;

    #[inline]
    fn len(&self) -> usize {
        N
    }

    fn by_index_dyn(&self, idx: usize) -> Option<ChildWidgetDynResult<'_,Self::ChildID,E>> {
        self.0.get(idx as usize).map(|widget| ChildWidgetDynResult {
            widget: &widget.widget,
            widget_id: widget.widget.id(),
            child_id: FixedIdx(idx as isize),
            idx,
        })
    }

    fn by_index_dyn_mut(&mut self, idx: usize) -> Option<ChildWidgetDynResultMut<'_,Self::ChildID,E>> {
        self.0.get_mut(idx as usize).map(|widget| ChildWidgetDynResultMut {
            widget_id: widget.widget.id(),
            widget: &mut widget.widget,
            child_id: FixedIdx(idx as isize),
            idx,
        })
    }

    fn idx_range_dyn<'a>(&'a self, range: Range<usize>, callback: &mut (dyn FnMut(ChildWidgetDynResult<'a,Self::ChildID,E>) + '_) ) {
        for (idx,widget) in self.0[range.start as usize .. range.end as usize].iter().enumerate() {
            callback(ChildWidgetDynResult {
                widget: &widget.widget,
                widget_id: widget.widget.id(),
                child_id: FixedIdx(idx as isize),
                idx: idx,
            })
        }
    }

    fn idx_range_dyn_mut<'a>(&'a mut self, range: Range<usize>, callback: &mut (dyn FnMut(ChildWidgetDynResultMut<'a,Self::ChildID,E>) + '_) ) {
        for (idx,widget) in self.0[range.start as usize .. range.end as usize].iter_mut().enumerate() {
            callback(ChildWidgetDynResultMut {
                widget_id: widget.widget.id(),
                widget: &mut widget.widget,
                child_id: FixedIdx(idx as isize),
                idx: idx,
            })
        }
    }

    // fn resolve_dyn<'a,'b>(&'a self, path: &'b (dyn PathResolvusDyn<E>+'b)) -> Option<ChildWidgetDynResolveResult<'a,'b,Self::ChildID,E>> {
    //     if let Some(v) = path.try_fragment::<Self::ChildID>() {
    //         let idx = v.0;
    //         if let Some(widget) = self.0.get(idx as usize) {
    //             return Some(ChildWidgetDynResolveResult {
    //                 widget,
    //                 widget_id: widget.id(),
    //                 child_id: FixedIdx(idx as isize),
    //                 idx,
    //                 resolvus: path.inner().unwrap(),
    //             });
    //         }
    //     }

    //     None
    // }

    // fn resolve_dyn_mut<'a,'b>(&'a mut self, path: &'b (dyn PathResolvusDyn<E>+'b)) -> Option<ChildWidgetDynResolveResultMut<'a,'b,Self::ChildID,E>> {
    //     if let Some(v) = path.try_fragment::<Self::ChildID>() {
    //         let idx = v.0;
    //         if let Some(widget) = self.0.get_mut(idx as usize) {
    //             return Some(ChildWidgetDynResolveResultMut {
    //                 widget_id: widget.id(),
    //                 widget,
    //                 child_id: FixedIdx(idx as isize),
    //                 idx,
    //                 resolvus: path.inner().unwrap(),
    //             });
    //         }
    //     }

    //     None
    // }
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
