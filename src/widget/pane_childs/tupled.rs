use std::ops::{Range, Deref, DerefMut};

use crate::env::Env;
use crate::newpath::{PathFragment, PathResolvusDyn, PathResolvus};
use crate::root::RootRef;
use crate::widget::Widget;

use super::{PaneChilds, PaneChildWidget, PaneChildsDyn, ChildWidgetDynResult, ChildWidgetDynResultMut};
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

impl<E,I,T> PaneChilds<E> for Tupled<Vec<(I,PaneChildWidget<T,E>)>> where T: Widget<E>, E: Env, I: PathFragment<E> + Clone + PartialEq + 'static {
    type Caches = Vec<T::Cache>;

    // #[inline]
    // fn by_index<F,R>(&self, idx: usize, callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    // where
    //     F: AsWidgetsDispatch<Self::ChildID,R,E>
    // {
    //     match self.0.get(idx as usize) {
    //         Some((id,inner)) => {
    //             callback.call(AsWidgetsResult::from_some(idx,id.clone(),inner), root, ctx)
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
    //         Some((id,inner)) => {
    //             callback.call(AsWidgetsResultMut::from_some(idx,id.clone(),inner), root, ctx)
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
    //         Some((id,inner)) => {
    //             let cache_slot = unsafe { cache.get_unchecked_mut(idx as usize) };
    //             callback.call(AsWidgetsCResult::from_some(idx,id.clone(),inner, cache_slot), root, ctx)
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
    //         Some((id,inner)) => {
    //             let cache_slot = unsafe { cache.get_unchecked_mut(idx as usize) };
    //             callback.call(AsWidgetsCResultMut::from_some(idx,id.clone(),inner, cache_slot), root, ctx)
    //         },
    //         None => callback.call_none(root,ctx),
    //     }
    // }

    // #[inline]
    // fn iter_ids(&self) -> Self::IdIdxIter {
    //     let h = self.0.iter().enumerate().map(#[inline] |(i,(id,_))| (i,id.clone()) ).collect::<Vec<_>>();
    //     h.into_iter()
    // }

    // #[inline]
    // fn idx_range<F>(&self, range: Range<usize>, mut callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    // where
    //     F: AsWidgetsIndexedDispatch<Self::ChildID,E>
    // {
    //     for (i,(id,v)) in self.0[range.start as usize .. range.end as usize].iter().enumerate() {
    //         callback.call(i, id.clone(), v, root.fork(), ctx)
    //     }
    // }

    // #[inline]
    // fn idx_range_mut<F>(&mut self, range: Range<usize>, mut callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    // where
    //     F: AsWidgetsIndexedDispatchMut<Self::ChildID,E>
    // {
    //     for (i,(id,v)) in self.0[range.start as usize .. range.end as usize].iter_mut().enumerate() {
    //         callback.call(i, id.clone(), v, root.fork(), ctx)
    //     }
    // }

    // #[inline]
    // fn idx_range_c<F>(&self, range: Range<usize>, mut callback: F, cache: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    // where
    //     F: AsWidgetsIndexedCDispatch<Self::ChildID,E>
    // {
    //     if self.0.len() != cache.len() {
    //         cache.resize_with(self.0.len(), Default::default);
    //     }

    //     for (i,(id,v)) in self.0[range.start as usize .. range.end as usize].iter().enumerate() {
    //         let cache_slot = unsafe { cache.get_unchecked_mut(i) };
    //         callback.call(i, id.clone(), v, cache_slot, root.fork(), ctx)
    //     }
    // }

    // #[inline]
    // fn idx_range_c_mut<F>(&mut self, range: Range<usize>, mut callback: F, cache: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    // where
    //     F: AsWidgetsIndexedCDispatchMut<Self::ChildID,E>
    // {
    //     if self.0.len() != cache.len() {
    //         cache.resize_with(self.0.len(), Default::default);
    //     }

    //     for (i,(id,v)) in self.0[range.start as usize .. range.end as usize].iter_mut().enumerate() {
    //         let cache_slot = unsafe { cache.get_unchecked_mut(i) };
    //         callback.call(i, id.clone(), v, cache_slot, root.fork(), ctx)
    //     }
    // }

    // fn resolve<F,R>(&self, path: &(dyn PathResolvusDyn<E>+'_), callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    // where
    //     F: AsWidgetsResolveDispatch<Self::ChildID,R,E>
    // {
    //     if let Some(v) = path.try_fragment::<Self::ChildID>() {
    //         let res = self.0.iter().enumerate()
    //             .find(#[inline] |(_,(i,_))| *i == *v);

    //         if let Some((idx,(id,inner))) = res {
    //             return callback.call(AsWidgetsResolveResult::from_some(idx,id.clone(),path.inner().unwrap(),inner), root, ctx);
    //         }
    //     }

    //     callback.call_none(root,ctx)
    // }

    // fn resolve_mut<F,R>(&mut self, path: &(dyn PathResolvusDyn<E>+'_), callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    // where
    //     F: AsWidgetsResolveDispatchMut<Self::ChildID,R,E>
    // {
    //     if let Some(v) = path.try_fragment::<Self::ChildID>() {
    //         let res = self.0.iter_mut().enumerate()
    //             .find(#[inline] |(_,(i,_))| *i == *v);

    //         if let Some((idx,(id,inner))) = res {
    //             return callback.call(AsWidgetsResolveResultMut::from_some(idx,id.clone(),path.inner().unwrap(),inner), root, ctx);
    //         }
    //     }

    //     callback.call_none(root,ctx)
    // }

    // fn resolve_c<F,R>(&self, path: &(dyn PathResolvusDyn<E>+'_), callback: F, cache: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    // where
    //     F: AsWidgetsResolveCDispatch<Self::ChildID,R,E>
    // {
    //     if self.0.len() != cache.len() {
    //         cache.resize_with(self.0.len(), Default::default);
    //     }

    //     if let Some(v) = path.try_fragment::<Self::ChildID>() {
    //         let res = self.0.iter().enumerate()
    //             .find(#[inline] |(_,(i,_))| *i == *v);

    //         if let Some((idx,(id,inner))) = res {
    //             let cache_slot = unsafe { cache.get_unchecked_mut(idx) };
    //             return callback.call(AsWidgetsResolveCResult::from_some(idx,id.clone(),path.inner().unwrap(),inner, cache_slot), root, ctx);
    //         }
    //     }

    //     callback.call_none(root,ctx)
    // }

    // fn resolve_c_mut<F,R>(&mut self, path: &(dyn PathResolvusDyn<E>+'_), callback: F, cache: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    // where
    //     F: AsWidgetsResolveCDispatchMut<Self::ChildID,R,E>
    // {
    //     if self.0.len() != cache.len() {
    //         cache.resize_with(self.0.len(), Default::default);
    //     }

    //     if let Some(v) = path.try_fragment::<Self::ChildID>() {
    //         let res = self.0.iter_mut().enumerate()
    //             .find(#[inline] |(_,(i,_))| *i == *v);

    //         if let Some((idx,(id,inner))) = res {
    //             let cache_slot = unsafe { cache.get_unchecked_mut(idx) };
    //             return callback.call(AsWidgetsResolveCResultMut::from_some(idx,id.clone(),path.inner().unwrap(),inner, cache_slot), root, ctx);
    //         }
    //     }

    //     callback.call_none(root,ctx)
    // }
}

impl<E,I,T,const N: usize> PaneChilds<E> for Tupled<[(I,PaneChildWidget<T,E>);N]> where T: Widget<E>, E: Env, I: PathFragment<E> + Clone + PartialEq + 'static {
    type Caches = Vec<T::Cache>;

    // #[inline]
    // fn by_index<F,R>(&self, idx: usize, callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    // where
    //     F: AsWidgetsDispatch<Self::ChildID,R,E>
    // {
    //     match self.0.get(idx as usize) {
    //         Some((id,inner)) => {
    //             callback.call(AsWidgetsResult::from_some(idx,id.clone(),inner), root, ctx)
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
    //         Some((id,inner)) => {
    //             callback.call(AsWidgetsResultMut::from_some(idx,id.clone(),inner), root, ctx)
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
    //         Some((id,inner)) => {
    //             let cache_slot = unsafe { cache.get_unchecked_mut(idx as usize) };
    //             callback.call(AsWidgetsCResult::from_some(idx,id.clone(),inner, cache_slot), root, ctx)
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
    //         Some((id,inner)) => {
    //             let cache_slot = unsafe { cache.get_unchecked_mut(idx as usize) };
    //             callback.call(AsWidgetsCResultMut::from_some(idx,id.clone(),inner, cache_slot), root, ctx)
    //         },
    //         None => callback.call_none(root,ctx),
    //     }
    // }

    // #[inline]
    // fn iter_ids(&self) -> Self::IdIdxIter {
    //     let h = self.0.iter().enumerate().map(#[inline] |(i,(id,_))| (i,id.clone()) ).collect::<Vec<_>>();
    //     h.into_iter()
    // }

    // #[inline]
    // fn idx_range<F>(&self, range: Range<usize>, mut callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    // where
    //     F: AsWidgetsIndexedDispatch<Self::ChildID,E>
    // {
    //     for (i,(id,v)) in self.0[range.start as usize .. range.end as usize].iter().enumerate() {
    //         callback.call(i, id.clone(), v, root.fork(), ctx)
    //     }
    // }

    // #[inline]
    // fn idx_range_mut<F>(&mut self, range: Range<usize>, mut callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    // where
    //     F: AsWidgetsIndexedDispatchMut<Self::ChildID,E>
    // {
    //     for (i,(id,v)) in self.0[range.start as usize .. range.end as usize].iter_mut().enumerate() {
    //         callback.call(i, id.clone(), v, root.fork(), ctx)
    //     }
    // }

    // #[inline]
    // fn idx_range_c<F>(&self, range: Range<usize>, mut callback: F, cache: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    // where
    //     F: AsWidgetsIndexedCDispatch<Self::ChildID,E>
    // {
    //     if self.0.len() != cache.len() {
    //         cache.resize_with(self.0.len(), Default::default);
    //     }

    //     for (i,(id,v)) in self.0[range.start as usize .. range.end as usize].iter().enumerate() {
    //         let cache_slot = unsafe { cache.get_unchecked_mut(i) };
    //         callback.call(i, id.clone(), v, cache_slot, root.fork(), ctx)
    //     }
    // }

    // #[inline]
    // fn idx_range_c_mut<F>(&mut self, range: Range<usize>, mut callback: F, cache: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    // where
    //     F: AsWidgetsIndexedCDispatchMut<Self::ChildID,E>
    // {
    //     if self.0.len() != cache.len() {
    //         cache.resize_with(self.0.len(), Default::default);
    //     }

    //     for (i,(id,v)) in self.0[range.start as usize .. range.end as usize].iter_mut().enumerate() {
    //         let cache_slot = unsafe { cache.get_unchecked_mut(i) };
    //         callback.call(i, id.clone(), v, cache_slot, root.fork(), ctx)
    //     }
    // }

    // fn resolve<F,R>(&self, path: &(dyn PathResolvusDyn<E>+'_), callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    // where
    //     F: AsWidgetsResolveDispatch<Self::ChildID,R,E>
    // {
    //     if let Some(v) = path.try_fragment::<Self::ChildID>() {
    //         let res = self.0.iter().enumerate()
    //             .find(#[inline] |(_,(i,_))| *i == *v);

    //         if let Some((idx,(id,inner))) = res {
    //             return callback.call(AsWidgetsResolveResult::from_some(idx,id.clone(),path.inner().unwrap(),inner), root, ctx);
    //         }
    //     }

    //     callback.call_none(root,ctx)
    // }

    // fn resolve_mut<F,R>(&mut self, path: &(dyn PathResolvusDyn<E>+'_), callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    // where
    //     F: AsWidgetsResolveDispatchMut<Self::ChildID,R,E>
    // {
    //     if let Some(v) = path.try_fragment::<Self::ChildID>() {
    //         let res = self.0.iter_mut().enumerate()
    //             .find(#[inline] |(_,(i,_))| *i == *v);

    //         if let Some((idx,(id,inner))) = res {
    //             return callback.call(AsWidgetsResolveResultMut::from_some(idx,id.clone(),path.inner().unwrap(),inner), root, ctx);
    //         }
    //     }

    //     callback.call_none(root,ctx)
    // }

    // fn resolve_c<F,R>(&self, path: &(dyn PathResolvusDyn<E>+'_), callback: F, cache: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    // where
    //     F: AsWidgetsResolveCDispatch<Self::ChildID,R,E>
    // {
    //     if self.0.len() != cache.len() {
    //         cache.resize_with(self.0.len(), Default::default);
    //     }

    //     if let Some(v) = path.try_fragment::<Self::ChildID>() {
    //         let res = self.0.iter().enumerate()
    //             .find(#[inline] |(_,(i,_))| *i == *v);

    //         if let Some((idx,(id,inner))) = res {
    //             let cache_slot = unsafe { cache.get_unchecked_mut(idx) };
    //             return callback.call(AsWidgetsResolveCResult::from_some(idx,id.clone(),path.inner().unwrap(),inner, cache_slot), root, ctx);
    //         }
    //     }

    //     callback.call_none(root,ctx)
    // }

    // fn resolve_c_mut<F,R>(&mut self, path: &(dyn PathResolvusDyn<E>+'_), callback: F, cache: &mut Self::Caches, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    // where
    //     F: AsWidgetsResolveCDispatchMut<Self::ChildID,R,E>
    // {
    //     if self.0.len() != cache.len() {
    //         cache.resize_with(self.0.len(), Default::default);
    //     }

    //     if let Some(v) = path.try_fragment::<Self::ChildID>() {
    //         let res = self.0.iter_mut().enumerate()
    //             .find(#[inline] |(_,(i,_))| *i == *v);

    //         if let Some((idx,(id,inner))) = res {
    //             let cache_slot = unsafe { cache.get_unchecked_mut(idx) };
    //             return callback.call(AsWidgetsResolveCResultMut::from_some(idx,id.clone(),path.inner().unwrap(),inner, cache_slot), root, ctx);
    //         }
    //     }

    //     callback.call_none(root,ctx)
    // }
}

impl<E,I,T> PaneChildsDyn<E> for Tupled<Vec<(I,PaneChildWidget<T,E>)>> where T: Widget<E>, E: Env, I: PathFragment<E> + Clone + PartialEq + 'static {
    type ChildID = I;

    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    fn by_index_dyn(&self, idx: usize) -> Option<ChildWidgetDynResult<'_,Self::ChildID,E>> {
        self.0.get(idx as usize).map(|(id,inner)| ChildWidgetDynResult {
            widget: &inner.widget,
            widget_id: inner.widget.id(),
            child_id: id.clone(),
            idx,
        })
    }

    #[inline]
    fn by_index_dyn_mut(&mut self, idx: usize) -> Option<super::ChildWidgetDynResultMut<'_,Self::ChildID,E>> {
        self.0.get_mut(idx as usize).map(|(id,inner)| ChildWidgetDynResultMut {
            widget_id: inner.widget.id(),
            widget: &mut inner.widget,
            child_id: id.clone(),
            idx,
        })
    }

    #[inline]
    fn idx_range_dyn<'a>(&'a self, range: Range<usize>, callback: &mut (dyn FnMut(ChildWidgetDynResult<'a,Self::ChildID,E>) + '_) ) {
        for (i,(id,v)) in self.0[range.start as usize .. range.end as usize].iter().enumerate() {
            callback(ChildWidgetDynResult {
                widget: &v.widget,
                widget_id: v.widget.id(),
                child_id: id.clone(),
                idx: i,
            })
        }
    }

    #[inline]
    fn idx_range_dyn_mut<'a>(&'a mut self, range: Range<usize>, callback: &mut (dyn FnMut(super::ChildWidgetDynResultMut<'a,Self::ChildID,E>) + '_) ) {
        for (i,(id,v)) in self.0[range.start as usize .. range.end as usize].iter_mut().enumerate() {
            callback(ChildWidgetDynResultMut {
                widget_id: v.widget.id(),
                widget: &mut v.widget,
                child_id: id.clone(),
                idx: i,
            })
        }
    }

    // fn resolve_dyn<'a,'b>(&'a self, path: &'b (dyn PathResolvusDyn<E>+'b)) -> Option<ChildWidgetDynResolveResult<'a,'b,Self::ChildID,E>> {
    //     if let Some(v) = path.try_fragment::<Self::ChildID>() {
    //         let res = self.0.iter().enumerate()
    //             .find(#[inline] |(_,(i,_))| *i == *v);

    //         if let Some((idx,(id,inner))) = res {
    //             return Some(ChildWidgetDynResolveResult {
    //                 widget: inner,
    //                 widget_id: inner.id(),
    //                 child_id: id.clone(),
    //                 idx: idx,
    //                 resolvus: path.inner().unwrap(),
    //             });
    //         }
    //     }

    //     None
    // }

    // fn resolve_dyn_mut<'a,'b>(&'a mut self, path: &'b (dyn PathResolvusDyn<E>+'b)) -> Option<ChildWidgetDynResolveResultMut<'a,'b,Self::ChildID,E>> {
    //     if let Some(v) = path.try_fragment::<Self::ChildID>() {
    //         let res = self.0.iter_mut().enumerate()
    //             .find(#[inline] |(_,(i,_))| *i == *v);

    //         if let Some((idx,(id,inner))) = res {
    //             return Some(ChildWidgetDynResolveResultMut {
    //                 widget_id: inner.id(),
    //                 widget: inner,
    //                 child_id: id.clone(),
    //                 idx: idx,
    //                 resolvus: path.inner().unwrap(),
    //             });
    //         }
    //     }

    //     None
    // }
}

impl<E,I,T,const N: usize> PaneChildsDyn<E> for Tupled<[(I,PaneChildWidget<T,E>);N]> where T: Widget<E>, E: Env, I: PathFragment<E> + Clone + PartialEq + 'static {
    type ChildID = I;

    #[inline]
    fn len(&self) -> usize {
        N
    }

    #[inline]
    fn by_index_dyn(&self, idx: usize) -> Option<ChildWidgetDynResult<'_,Self::ChildID,E>> {
        self.0.get(idx as usize).map(|(id,inner)| ChildWidgetDynResult {
            widget: &inner.widget,
            widget_id: inner.widget.id(),
            child_id: id.clone(),
            idx,
        })
    }

    #[inline]
    fn by_index_dyn_mut(&mut self, idx: usize) -> Option<super::ChildWidgetDynResultMut<'_,Self::ChildID,E>> {
        self.0.get_mut(idx as usize).map(|(id,inner)| ChildWidgetDynResultMut {
            widget_id: inner.widget.id(),
            widget: &mut inner.widget,
            child_id: id.clone(),
            idx,
        })
    }

    #[inline]
    fn idx_range_dyn<'a>(&'a self, range: Range<usize>, callback: &mut (dyn FnMut(ChildWidgetDynResult<'a,Self::ChildID,E>) + '_) ) {
        for (i,(id,v)) in self.0[range.start as usize .. range.end as usize].iter().enumerate() {
            callback(ChildWidgetDynResult {
                widget: &v.widget,
                widget_id: v.widget.id(),
                child_id: id.clone(),
                idx: i,
            })
        }
    }

    #[inline]
    fn idx_range_dyn_mut<'a>(&'a mut self, range: Range<usize>, callback: &mut (dyn FnMut(super::ChildWidgetDynResultMut<'a,Self::ChildID,E>) + '_) ) {
        for (i,(id,v)) in self.0[range.start as usize .. range.end as usize].iter_mut().enumerate() {
            callback(ChildWidgetDynResultMut {
                widget_id: v.widget.id(),
                widget: &mut v.widget,
                child_id: id.clone(),
                idx: i,
            })
        }
    }

    // fn resolve_dyn<'a,'b>(&'a self, path: &'b (dyn PathResolvusDyn<E>+'b)) -> Option<ChildWidgetDynResolveResult<'a,'b,Self::ChildID,E>> {
    //     if let Some(v) = path.try_fragment::<Self::ChildID>() {
    //         let res = self.0.iter().enumerate()
    //             .find(#[inline] |(_,(i,_))| *i == *v);

    //         if let Some((idx,(id,inner))) = res {
    //             return Some(ChildWidgetDynResolveResult {
    //                 widget: inner,
    //                 widget_id: inner.id(),
    //                 child_id: id.clone(),
    //                 idx: idx,
    //                 resolvus: path.inner().unwrap(),
    //             });
    //         }
    //     }

    //     None
    // }

    // fn resolve_dyn_mut<'a,'b>(&'a mut self, path: &'b (dyn PathResolvusDyn<E>+'b)) -> Option<ChildWidgetDynResolveResultMut<'a,'b,Self::ChildID,E>> {
    //     if let Some(v) = path.try_fragment::<Self::ChildID>() {
    //         let res = self.0.iter_mut().enumerate()
    //             .find(#[inline] |(_,(i,_))| *i == *v);

    //         if let Some((idx,(id,inner))) = res {
    //             return Some(ChildWidgetDynResolveResultMut {
    //                 widget_id: inner.id(),
    //                 widget: inner,
    //                 child_id: id.clone(),
    //                 idx: idx,
    //                 resolvus: path.inner().unwrap(),
    //             });
    //         }
    //     }

    //     None
    // }
}
