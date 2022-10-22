use std::ops::{Range, Deref, DerefMut};

use crate::dispatchor::{AsWidgetClosure, AsWidgetsResult, AsWidgetsDispatch, AsWidgetsIndexedDispatch, AsWidgetsResolveDispatch, AsWidgetsResolveResult};
use crate::env::Env;
use crate::newpath::{PathFragment, PathResolvusDyn, PathResolvus};
use crate::root::RootRef;
use crate::widget::as_widget::AsWidget;

use super::AsWidgets;

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

impl<'h,E,I,T> AsWidgets<E> for Tupled<&'h [(I,T)]> where T: AsWidget<E> + 'h, E: Env, I: PathFragment<E> + Clone + PartialEq + 'static {
    type Widget<'v> = T::Widget<'v> where Self: 'v;
    type WidgetCache = T::WidgetCache;
    type ChildID = I;
    type IdIdxIter = impl Iterator<Item=(usize,Self::ChildID)>;

    #[inline]
    fn by_index<R>(&self, idx: usize, callback: &mut (dyn AsWidgetsDispatch<Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R {
        match self.0.get(idx) {
            Some((id,inner)) => {
                let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    callback.call(AsWidgetsResult::from_some(idx,id.clone(),T::covar_ref(widget)), root, ctx)
                });
                (*inner).with_widget(&mut callback,root,ctx)
            },
            None => callback.call(None,root,ctx),
        }
    }

    #[inline]
    fn by_id<R>(&self, id: &Self::ChildID, callback: &mut (dyn AsWidgetsDispatch<Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R {
        let res = self.0.iter().enumerate()
            .find(#[inline] |(_,(i,_))| *i == *id);
        
        match res {
            Some((idx,(id,inner))) => {
                let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    callback.call(AsWidgetsResult::from_some(idx,id.clone(),T::covar_ref(widget)), root, ctx)
                });
                (*inner).with_widget(&mut callback,root,ctx)
            },
            None => callback.call(None,root,ctx),
        }
    }

    #[inline]
    fn iter_ids(&self) -> Self::IdIdxIter {
        self.0.iter().enumerate().map(#[inline] |(i,(id,_))| (i,id.clone()) )
    }

    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    fn idx_range(&self, range: Range<usize>, callback: &mut (dyn AsWidgetsIndexedDispatch<Self,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) {
        for (i,(id,v)) in self.0[range].iter().enumerate() {
            let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                callback.call(i, id.clone(), T::covar_ref(widget), root, ctx)
            });
            v.with_widget(&mut callback,root.fork(),ctx)
        }
    }

    #[inline]
    fn idx_range_filtered(&self, range: Range<usize>, mut filter: impl for<'a> FnMut(usize,&'a Self::ChildID) -> bool, callback: &mut (dyn AsWidgetsIndexedDispatch<Self,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) {
        for (i,(id,v)) in self.0[range].iter().enumerate() {
            if (filter)(i,id) {
                let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    callback.call(i, id.clone(), T::covar_ref(widget), root, ctx)
                });
                v.with_widget(&mut callback,root.fork(),ctx)
            }
        }
    }

    fn resolve<R>(&self, path: &(dyn PathResolvusDyn<E>+'_), callback: &mut (dyn AsWidgetsResolveDispatch<Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R {
        if let Some(v) = path.try_fragment::<Self::ChildID>() {
            let res = self.0.iter().enumerate()
                .find(#[inline] |(_,(i,_))| *i == *v);

            if let Some((idx,(id,inner))) = res {
                let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    callback.call(AsWidgetsResolveResult::from_some(idx,id.clone(),path.inner().unwrap(),T::covar_ref(widget)), root, ctx)
                });
                return (*inner).with_widget(&mut callback,root,ctx);
            }
        }

        callback.call(None,root,ctx)
    }

    #[inline]
    fn covar_ref<'s,'ll,'ss>(w: &'s Self::Widget<'ll>) -> &'s Self::Widget<'ss> where 'll: 'ss, 'ss: 's, Self: 'll {
        T::covar_ref(w)
    }
}
