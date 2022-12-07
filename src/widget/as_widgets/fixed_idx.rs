use std::ops::{Range, Deref, DerefMut};

use crate::dispatchor::*;
use crate::env::Env;
use crate::newpath::{PathResolvusDyn, FixedIdx, PathResolvus};
use crate::root::RootRef;
use crate::widget::WidgetDyn;
use crate::widget::cache::DynWidgetCache;
use crate::widget::as_widget::AsWidget;

use super::AsWidgets;

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

#[inline]
fn bender<'a,'b,T>(v: &'a WidgetsFixedIdx<&'b T>) -> &'a WidgetsFixedIdx<T> where 'b: 'a, T: 'b + Sized {
    unsafe{std::mem::transmute(v)}
}

impl<E,T> AsWidgets<E> for WidgetsFixedIdx<&'_ T> where T: Sized, WidgetsFixedIdx<T>: AsWidgets<E>, E: Env {
    type WidgetCache = <WidgetsFixedIdx<T> as AsWidgets<E>>::WidgetCache;
    type ChildID = <WidgetsFixedIdx<T> as AsWidgets<E>>::ChildID;
    type IdIdxIter = <WidgetsFixedIdx<T> as AsWidgets<E>>::IdIdxIter;

    #[inline]
    fn by_index<F,R>(&self, idx: usize, callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsDispatch<Self::ChildID,R,E>
    {
        (*bender(self)).by_index(idx, callback, root, ctx)
    }

    #[inline]
    fn by_id<F,R>(&self, id: &Self::ChildID, callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsDispatch<Self::ChildID,R,E>
    {
        (*bender(self)).by_id(id, callback, root, ctx)
    }

    #[inline]
    fn iter_ids(&self) -> Self::IdIdxIter {
        (*bender(self)).iter_ids()
    }

    #[inline]
    fn len(&self) -> usize {
        (*bender(self)).len()
    }

    #[inline]
    fn idx_range<F>(&self, range: Range<usize>, callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedDispatch<Self::ChildID,E>
    {
        (*bender(self)).idx_range(range, callback, root, ctx)
    }

    #[inline]
    fn idx_range_filtered<F>(&self, range: Range<usize>, filter: impl for<'a> FnMut(usize,&'a Self::ChildID) -> bool, callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedDispatch<Self::ChildID,E>
    {
        (*bender(self)).idx_range_filtered(range, filter, callback, root, ctx)
    }

    #[inline]
    fn resolve<F,R>(&self, path: &(dyn PathResolvusDyn<E>+'_), callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsResolveDispatch<Self::ChildID,R,E>
    {
        (*bender(self)).resolve(path, callback, root, ctx)
    }
}

impl<E,T> AsWidgets<E> for WidgetsFixedIdx<[T]> where T: AsWidget<E>, E: Env {
    type WidgetCache = T::WidgetCache;
    type ChildID = FixedIdx;
    type IdIdxIter = impl Iterator<Item=(usize,Self::ChildID)>;

    #[inline]
    fn by_index<F,R>(&self, idx: usize, mut callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsDispatch<Self::ChildID,R,E>
    {
        match self.0.get(idx) {
            Some(inner) => {
                let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    callback.call(AsWidgetsResult::from_some(idx,FixedIdx(idx),widget), root, ctx)
                });
                inner.with_widget(&mut callback,root,ctx)
            },
            None => callback.call_none(root,ctx),
        }
    }

    #[inline]
    fn by_id<F,R>(&self, id: &Self::ChildID, callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsDispatch<Self::ChildID,R,E>
    {
        self.by_index(id.0, callback, root, ctx)
    }

    #[inline]
    fn iter_ids(&self) -> Self::IdIdxIter {
        (0..self.len()).map(#[inline] |i| (i,FixedIdx(i)))
    }

    #[inline]
    fn len(&self) -> usize {
        <[T]>::len(&self.0)
    }

    #[inline]
    fn idx_range_filtered<F>(&self, idx_range: Range<usize>, mut filter: impl for<'a> FnMut(usize,&'a Self::ChildID) -> bool, mut callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedDispatch<Self::ChildID,E>
    {
        for (i,v) in self.0[idx_range].iter().enumerate() {
            if (filter)(i,&FixedIdx(i)) {
                let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    callback.call(i, FixedIdx(i), widget, root, ctx)
                });
                v.with_widget(&mut callback,root.fork(),ctx)
            }
        }
    }

    fn resolve<F,R>(&self, path: &(dyn PathResolvusDyn<E>+'_), mut callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsResolveDispatch<Self::ChildID,R,E>
    {
        if let Some(v) = path.try_fragment::<Self::ChildID>() {
            let idx = v.0;
            if let Some(inner) = self.0.get(idx) {
                let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    callback.call(AsWidgetsResolveResult::from_some(idx,FixedIdx(idx),path.inner().unwrap(),widget), root, ctx)
                });
                return inner.with_widget(&mut callback,root,ctx);
            }
        }

        callback.call_none(root,ctx)
    }
}

impl<E,T,const N: usize> AsWidgets<E> for WidgetsFixedIdx<[T;N]> where T: AsWidget<E>, E: Env {
    type WidgetCache = T::WidgetCache;
    type ChildID = FixedIdx;
    type IdIdxIter = impl Iterator<Item=(usize,Self::ChildID)>;

    #[inline]
    fn by_index<F,R>(&self, idx: usize, mut callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsDispatch<Self::ChildID,R,E>
    {
        match self.0.get(idx) {
            Some(inner) => {
                let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    callback.call(AsWidgetsResult::from_some(idx,FixedIdx(idx),widget), root, ctx)
                });
                inner.with_widget(&mut callback,root,ctx)
            },
            None => callback.call_none(root,ctx),
        }
    }

    #[inline]
    fn by_id<F,R>(&self, id: &Self::ChildID, callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsDispatch<Self::ChildID,R,E>
    {
        self.by_index(id.0, callback, root, ctx)
    }

    #[inline]
    fn iter_ids(&self) -> Self::IdIdxIter {
        (0..self.len()).map(#[inline] |i| (i,FixedIdx(i)))
    }

    #[inline]
    fn len(&self) -> usize {
        N
    }

    #[inline]
    fn idx_range_filtered<F>(&self, idx_range: Range<usize>, mut filter: impl for<'a> FnMut(usize,&'a Self::ChildID) -> bool, mut callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedDispatch<Self::ChildID,E>
    {
        for (i,v) in self.0[idx_range].iter().enumerate() {
            if (filter)(i,&FixedIdx(i)) {
                let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    callback.call(i, FixedIdx(i), widget, root, ctx)
                });
                v.with_widget(&mut callback,root.fork(),ctx)
            }
        }
    }

    fn resolve<F,R>(&self, path: &(dyn PathResolvusDyn<E>+'_), mut callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsResolveDispatch<Self::ChildID,R,E>
    {
        if let Some(v) = path.try_fragment::<Self::ChildID>() {
            let idx = v.0;
            if let Some(inner) = self.0.get(idx) {
                let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    callback.call(AsWidgetsResolveResult::from_some(idx,FixedIdx(idx),path.inner().unwrap(),widget), root, ctx)
                });
                return inner.with_widget(&mut callback,root,ctx);
            }
        }

        callback.call_none(root,ctx)
    }
}

impl<E,T> AsWidgets<E> for WidgetsFixedIdx<&'_ [T]> where T: AsWidget<E>, E: Env {
    type WidgetCache = T::WidgetCache;
    type ChildID = FixedIdx;
    type IdIdxIter = impl Iterator<Item=(usize,Self::ChildID)>;

    #[inline]
    fn by_index<F,R>(&self, idx: usize, mut callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsDispatch<Self::ChildID,R,E>
    {
        match self.0.get(idx) {
            Some(inner) => {
                let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    callback.call(AsWidgetsResult::from_some(idx,FixedIdx(idx),widget), root, ctx)
                });
                inner.with_widget(&mut callback,root,ctx)
            },
            None => callback.call_none(root,ctx),
        }
    }

    #[inline]
    fn by_id<F,R>(&self, id: &Self::ChildID, callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsDispatch<Self::ChildID,R,E>
    {
        self.by_index(id.0, callback, root, ctx)
    }

    #[inline]
    fn iter_ids(&self) -> Self::IdIdxIter {
        (0..self.len()).map(#[inline] |i| (i,FixedIdx(i)))
    }

    #[inline]
    fn len(&self) -> usize {
        <[T]>::len(self.0)
    }

    #[inline]
    fn idx_range_filtered<F>(&self, idx_range: Range<usize>, mut filter: impl for<'a> FnMut(usize,&'a Self::ChildID) -> bool, mut callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        F: AsWidgetsIndexedDispatch<Self::ChildID,E>
    {
        for (i,v) in self.0[idx_range].iter().enumerate() {
            if (filter)(i,&FixedIdx(i)) {
                let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    callback.call(i, FixedIdx(i), widget, root, ctx)
                });
                v.with_widget(&mut callback,root.fork(),ctx)
            }
        }
    }

    fn resolve<F,R>(&self, path: &(dyn PathResolvusDyn<E>+'_), mut callback: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetsResolveDispatch<Self::ChildID,R,E>
    {
        if let Some(v) = path.try_fragment::<Self::ChildID>() {
            let idx = v.0;
            if let Some(inner) = self.0.get(idx) {
                let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    callback.call(AsWidgetsResolveResult::from_some(idx,FixedIdx(idx),path.inner().unwrap(),widget), root, ctx)
                });
                return inner.with_widget(&mut callback,root,ctx);
            }
        }

        callback.call_none(root,ctx)
    }
}
