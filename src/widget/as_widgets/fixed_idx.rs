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
    type Widget<'v> = <WidgetsFixedIdx<T> as AsWidgets<E>>::Widget<'v> where Self: 'v;
    type WidgetCache = <WidgetsFixedIdx<T> as AsWidgets<E>>::WidgetCache;
    type ChildID = <WidgetsFixedIdx<T> as AsWidgets<E>>::ChildID;
    type IdIdxIter = <WidgetsFixedIdx<T> as AsWidgets<E>>::IdIdxIter;

    #[inline]
    fn by_index<R>(&self, idx: usize, callback: &mut (dyn AsWidgetsDispatch<Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R {
        let mut callback = AsWidgetsClosure::new(#[inline] |result,root,ctx| {
            let result = match result {
                Some(r) => Some(AsWidgetsResult {
                    idx: r.idx,
                    child_id: r.child_id,
                    widget: WidgetsFixedIdx::<T>::covar_ref(r.widget),
                }),
                None => None,
            };
            callback.call(result, root, ctx)
        });
        (*bender(self)).by_index(idx, &mut callback, root, ctx)
    }

    #[inline]
    fn by_id<R>(&self, id: &Self::ChildID, callback: &mut (dyn AsWidgetsDispatch<Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R {
        let mut callback = AsWidgetsClosure::new(#[inline] |result,root,ctx| {
            let result = match result {
                Some(r) => Some(AsWidgetsResult {
                    idx: r.idx,
                    child_id: r.child_id,
                    widget: WidgetsFixedIdx::<T>::covar_ref(r.widget),
                }),
                None => None,
            };
            callback.call(result, root, ctx)
        });
        (*bender(self)).by_id(id, &mut callback, root, ctx)
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
    fn idx_range(&self, range: Range<usize>, callback: &mut (dyn AsWidgetsIndexedDispatch<Self,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) {
        let mut callback = AsWidgetsAllClosure::new(#[inline] |idx,child_id,widget,root,ctx| {
            callback.call(idx, child_id, WidgetsFixedIdx::<T>::covar_ref(widget), root, ctx)
        });
        (*bender(self)).idx_range(range, &mut callback, root, ctx)
    }

    #[inline]
    fn idx_range_filtered(&self, range: Range<usize>, filter: impl for<'a> FnMut(usize,&'a Self::ChildID) -> bool, callback: &mut (dyn AsWidgetsIndexedDispatch<Self,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) {
        let mut callback = AsWidgetsAllClosure::new(#[inline] |idx,child_id,widget,root,ctx| {
            callback.call(idx, child_id, WidgetsFixedIdx::<T>::covar_ref(widget), root, ctx)
        });
        (*bender(self)).idx_range_filtered(range, filter, &mut callback, root, ctx)
    }

    #[inline]
    fn resolve<R>(&self, path: &(dyn PathResolvusDyn<E>+'_), callback: &mut (dyn AsWidgetsResolveDispatch<Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R {
        let mut callback = AsWidgetsResolveClosure::new(#[inline] |result,root,ctx| {
            let result = match result {
                Some(r) => Some(AsWidgetsResolveResult {
                    idx: r.idx,
                    child_id: r.child_id,
                    resolvus: r.resolvus,
                    widget: WidgetsFixedIdx::<T>::covar_ref(r.widget),
                }),
                None => None,
            };
            callback.call(result, root, ctx)
        });
        (*bender(self)).resolve(path, &mut callback, root, ctx)
    }

    #[inline]
    fn covar_ref<'s,'ll,'ss>(w: &'s Self::Widget<'ll>) -> &'s Self::Widget<'ss> where 'll: 'ss, 'ss: 's, Self: 'll {
        WidgetsFixedIdx::<T>::covar_ref(w)
    }
}

impl<E,T> AsWidgets<E> for WidgetsFixedIdx<[T]> where T: AsWidget<E>, E: Env {
    type Widget<'v> = T::Widget<'v> where Self: 'v;
    type WidgetCache = T::WidgetCache;
    type ChildID = FixedIdx;
    type IdIdxIter = impl Iterator<Item=(usize,Self::ChildID)>;

    #[inline]
    fn by_index<R>(&self, idx: usize, callback: &mut (dyn AsWidgetsDispatch<Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R {
        match self.0.get(idx) {
            Some(inner) => {
                let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    callback.call(AsWidgetsResult::from_some(idx,FixedIdx(idx),widget), root, ctx)
                });
                inner.with_widget(&mut callback,root,ctx)
            },
            None => callback.call(None,root,ctx),
        }
    }

    #[inline]
    fn by_id<R>(&self, id: &Self::ChildID, callback: &mut (dyn AsWidgetsDispatch<Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R {
        match self.0.get(id.0) {
            Some(inner) => {
                let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    callback.call(AsWidgetsResult::from_some(id.0,id.clone(),widget), root, ctx)
                });
                inner.with_widget(&mut callback,root,ctx)
            },
            None => callback.call(None,root,ctx),
        }
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
    fn idx_range_filtered(&self, idx_range: Range<usize>, mut filter: impl for<'a> FnMut(usize,&'a Self::ChildID) -> bool, callback: &mut (dyn AsWidgetsIndexedDispatch<Self,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) {
        for (i,v) in self.0[idx_range].iter().enumerate() {
            if (filter)(i,&FixedIdx(i)) {
                let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    callback.call(i, FixedIdx(i), widget, root, ctx)
                });
                v.with_widget(&mut callback,root.fork(),ctx)
            }
        }
    }

    fn resolve<R>(&self, path: &(dyn PathResolvusDyn<E>+'_), callback: &mut (dyn AsWidgetsResolveDispatch<Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R {
        if let Some(v) = path.try_fragment::<Self::ChildID>() {
            let idx = v.0;
            if let Some(inner) = self.0.get(idx) {
                let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    callback.call(AsWidgetsResolveResult::from_some(idx,FixedIdx(idx),path.inner().unwrap(),widget), root, ctx)
                });
                return inner.with_widget(&mut callback,root,ctx);
            }
        }

        callback.call(None,root,ctx)
    }

    #[inline]
    fn covar_ref<'s,'ll,'ss>(w: &'s Self::Widget<'ll>) -> &'s Self::Widget<'ss> where 'll: 'ss, 'ss: 's, Self: 'll {
        T::covar_ref(w)
    }
}

impl<E,T,const N: usize> AsWidgets<E> for WidgetsFixedIdx<[T;N]> where T: AsWidget<E>, E: Env {
    type Widget<'v> = T::Widget<'v> where Self: 'v;
    type WidgetCache = T::WidgetCache;
    type ChildID = FixedIdx;
    type IdIdxIter = impl Iterator<Item=(usize,Self::ChildID)>;

    #[inline]
    fn by_index<R>(&self, idx: usize, callback: &mut (dyn AsWidgetsDispatch<Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R {
        match self.0.get(idx) {
            Some(inner) => {
                let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    callback.call(AsWidgetsResult::from_some(idx,FixedIdx(idx),widget), root, ctx)
                });
                inner.with_widget(&mut callback,root,ctx)
            },
            None => callback.call(None,root,ctx),
        }
    }

    #[inline]
    fn by_id<R>(&self, id: &Self::ChildID, callback: &mut (dyn AsWidgetsDispatch<Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R {
        match self.0.get(id.0) {
            Some(inner) => {
                let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    callback.call(AsWidgetsResult::from_some(id.0,id.clone(),widget), root, ctx)
                });
                inner.with_widget(&mut callback,root,ctx)
            },
            None => callback.call(None,root,ctx),
        }
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
    fn idx_range_filtered(&self, idx_range: Range<usize>, mut filter: impl for<'a> FnMut(usize,&'a Self::ChildID) -> bool, callback: &mut (dyn AsWidgetsIndexedDispatch<Self,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) {
        for (i,v) in self.0[idx_range].iter().enumerate() {
            if (filter)(i,&FixedIdx(i)) {
                let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    callback.call(i, FixedIdx(i), widget, root, ctx)
                });
                v.with_widget(&mut callback,root.fork(),ctx)
            }
        }
        // let h = WidgetsFixedIdx(&self.0[..]);
        // // let mut callback = AsWidgetsAllClosure::<_,WidgetsFixedIdx<&[T]>,_>::new(#[inline] |a,b,widget,root,ctx| {
        // //     todo!()//callback.call(a, b, WidgetsFixedIdx::<&[T]>::covar_ref(widget), root, ctx)
        // // });
        // // h.idx_range_filtered(idx_range, filter, &mut callback, root, ctx)
        // with_as_widgets_idx_range_filtered(
        //     &h,
        //     idx_range, filter,
        //     #[inline] |a,b,widget,root,ctx| {
        //         callback.call(a, b, WidgetsFixedIdx::<&[T]>::covar_ref(widget), root, ctx)
        //     },
        //     root, ctx
        // )
    }

    fn resolve<R>(&self, path: &(dyn PathResolvusDyn<E>+'_), callback: &mut (dyn AsWidgetsResolveDispatch<Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R {
        if let Some(v) = path.try_fragment::<Self::ChildID>() {
            let idx = v.0;
            if let Some(inner) = self.0.get(idx) {
                let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    callback.call(AsWidgetsResolveResult::from_some(idx,FixedIdx(idx),path.inner().unwrap(),widget), root, ctx)
                });
                return inner.with_widget(&mut callback,root,ctx);
            }
        }

        callback.call(None,root,ctx)
    }

    #[inline]
    fn covar_ref<'s,'ll,'ss>(w: &'s Self::Widget<'ll>) -> &'s Self::Widget<'ss> where 'll: 'ss, 'ss: 's, Self: 'll {
        T::covar_ref(w)
    }
}

// #[inline]
// pub fn with_as_widgets_idx_range_filtered<'z,W,C,E>(
//     w: &'z W, 
//     idx_range: Range<usize>, filter: impl for<'a> FnMut(usize,&'a W::ChildID) -> bool,
//     c: C, root: E::RootRef<'_>, ctx: &mut E::Context<'_>
// )
// where
//     W: AsWidgets<E> + ?Sized + 'z,
//     E: Env,
//     for<'w,'ww,'r,'c,'cc> C: FnMut(usize,W::ChildID,&'w W::Widget<'ww>,E::RootRef<'r>,&'c mut E::Context<'cc>),
// {
//     W::idx_range_filtered(
//         w,
//         idx_range, filter,
//         &mut AsWidgetsAllClosure::new(c),
//         root, ctx,
//     )
// }

impl<E,T> AsWidgets<E> for WidgetsFixedIdx<&'_ [T]> where T: AsWidget<E>, E: Env {
    type Widget<'v> = T::Widget<'v> where Self: 'v;
    type WidgetCache = T::WidgetCache;
    type ChildID = FixedIdx;
    type IdIdxIter = impl Iterator<Item=(usize,Self::ChildID)>;

    #[inline]
    fn by_index<R>(&self, idx: usize, callback: &mut (dyn AsWidgetsDispatch<Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R {
        match self.0.get(idx) {
            Some(inner) => {
                let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    callback.call(AsWidgetsResult::from_some(idx,FixedIdx(idx),T::covar_ref(widget)), root, ctx)
                });
                inner.with_widget(&mut callback,root,ctx)
            },
            None => callback.call(None,root,ctx),
        }
    }

    #[inline]
    fn by_id<R>(&self, id: &Self::ChildID, callback: &mut (dyn AsWidgetsDispatch<Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R {
        match self.0.get(id.0) {
            Some(inner) => {
                let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    callback.call(AsWidgetsResult::from_some(id.0,id.clone(),T::covar_ref(widget)), root, ctx)
                });
                inner.with_widget(&mut callback,root,ctx)
            },
            None => callback.call(None,root,ctx),
        }
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
    fn idx_range_filtered(&self, idx_range: Range<usize>, mut filter: impl for<'a> FnMut(usize,&'a Self::ChildID) -> bool, callback: &mut (dyn AsWidgetsIndexedDispatch<Self,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) {
        for (i,v) in self.0[idx_range].iter().enumerate() {
            if (filter)(i,&FixedIdx(i)) {
                let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    callback.call(i, FixedIdx(i), T::covar_ref(widget), root, ctx)
                });
                v.with_widget(&mut callback,root.fork(),ctx)
            }
        }
    }

    fn resolve<R>(&self, path: &(dyn PathResolvusDyn<E>+'_), callback: &mut (dyn AsWidgetsResolveDispatch<Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R {
        if let Some(v) = path.try_fragment::<Self::ChildID>() {
            let idx = v.0;
            if let Some(inner) = self.0.get(idx) {
                let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    callback.call(AsWidgetsResolveResult::from_some(idx,FixedIdx(idx),path.inner().unwrap(),T::covar_ref(widget)), root, ctx)
                });
                return inner.with_widget(&mut callback,root,ctx);
            }
        }

        callback.call(None,root,ctx)
    }

    #[inline]
    fn covar_ref<'s,'ll,'ss>(w: &'s Self::Widget<'ll>) -> &'s Self::Widget<'ss> where 'll: 'ss, 'ss: 's, Self: 'll {
        T::covar_ref(w)
    }
}
