use std::ops::Range;

use crate::dispatchor::{AsWidgetClosure, AsWidgetsResult, AsWidgetsDispatch, AsWidgetsIndexedDispatch, AsWidgetsResolveDispatch, AsWidgetsResolveResult};
use crate::env::Env;
use crate::newpath::{PathFragment, PathResolvusDyn, PathResolvus};
use crate::root::RootRef;
use crate::widget::as_widget::AsWidget;

use super::AsWidgets;

#[repr(transparent)]
pub struct Tupled<T>(pub T) where T: ?Sized;

impl<'s,E,I,T> AsWidgets<E> for Tupled<&'s [(I,T)]> where T: AsWidget<E> + 's, E: Env, I: PathFragment<E> + Clone + PartialEq + 'static {
    type Widget<'v,'z> = T::Widget<'v,'z> where 'z: 'v, Self: 'z;
    type WidgetCache = T::WidgetCache;
    type ChildID = I;
    type IdIdxIter = impl Iterator<Item=(usize,Self::ChildID)>;

    #[inline]
    fn by_index<'w,R>(&self, idx: usize, callback: &mut (dyn AsWidgetsDispatch<'w,Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        Self: 'w
    {
        match self.0.get(idx) {
            Some((id,inner)) => {
                let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    callback.call(AsWidgetsResult::from_some(idx,id.clone(),widget), root, ctx)
                });
                (*inner).with_widget(&mut callback,root,ctx)
            },
            None => callback.call(None,root,ctx),
        }
    }

    #[inline]
    fn by_id<'w,R>(&self, id: &Self::ChildID, callback: &mut (dyn AsWidgetsDispatch<'w,Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        Self: 'w
    {
        let res = self.0.iter().enumerate()
            .find(#[inline] |(_,(i,_))| *i == *id);
        
        match res {
            Some((idx,(id,inner))) => {
                let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    callback.call(AsWidgetsResult::from_some(idx,id.clone(),widget), root, ctx)
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
    fn idx_range<'w>(&self, range: Range<usize>, callback: &mut (dyn AsWidgetsIndexedDispatch<'w,Self,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        Self: 'w
    {
        for (i,(id,v)) in self.0[range].iter().enumerate() {
            let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                callback.call(i, id.clone(), widget, root, ctx)
            });
            v.with_widget(&mut callback,root.fork(),ctx)
        }
    }

    #[inline]
    fn idx_range_filtered<'w>(&self, range: Range<usize>, mut filter: impl for<'a> FnMut(usize,&'a Self::ChildID) -> bool, callback: &mut (dyn AsWidgetsIndexedDispatch<'w,Self,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
    where
        Self: 'w
    {
        for (i,(id,v)) in self.0[range].iter().enumerate() {
            if (filter)(i,id) {
                let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    callback.call(i, id.clone(), widget, root, ctx)
                });
                v.with_widget(&mut callback,root.fork(),ctx)
            }
        }
    }

    fn resolve<'w,R>(&self, path: &(dyn PathResolvusDyn<E>+'_), callback: &mut (dyn AsWidgetsResolveDispatch<'w,Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        Self: 'w
    {
        if let Some(v) = path.try_fragment::<Self::ChildID>() {
            let res = self.0.iter().enumerate()
                .find(#[inline] |(_,(i,_))| *i == *v);

            if let Some((idx,(id,inner))) = res {
                let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
                    callback.call(AsWidgetsResolveResult::from_some(idx,id.clone(),path.inner().unwrap(),widget), root, ctx)
                });
                return (*inner).with_widget(&mut callback,root,ctx);
            }
        }

        callback.call(None,root,ctx)
    }
}
