use crate::dispatchor::{AsWidgetDispatch, AsWidgetClosure};
use crate::env::Env;

use super::Widget;
use super::cache::{DynWidgetCache, WidgetCache};
use super::dyn_tunnel::WidgetDyn;

pub trait AsWidget<E> where E: Env {
    type Widget<'v,'z>: Widget<E,Cache=Self::WidgetCache> + ?Sized + 'v where 'z: 'v, Self: 'z;
    type WidgetCache: WidgetCache<E>; // this ugly hack as we can't even refer to 'static types behind lifetime GATs without effect of these lifetimes

    fn with_widget<'w,R>(&self, f: &mut (dyn AsWidgetDispatch<'w,Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        Self: 'w
   ;
}

impl<E> AsWidget<E> for dyn WidgetDyn<E> + '_ where E: Env {
    type Widget<'v,'z> = dyn WidgetDyn<E>+'v where 'z: 'v, Self: 'z;
    type WidgetCache = DynWidgetCache<E>;

    #[inline]
    fn with_widget<'w,R>(&self, f: &mut (dyn AsWidgetDispatch<'w,Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        Self: 'w
    {
        f.call(self, root, ctx)
    }
}

impl<T,E> AsWidget<E> for &'_ T where T: AsWidget<E> + ?Sized, E: Env {
    type Widget<'v,'z> = T::Widget<'v,'z> where 'z: 'v, Self: 'z;
    type WidgetCache = T::WidgetCache;

    #[inline]
    fn with_widget<'w,R>(&self, callback: &mut (dyn AsWidgetDispatch<'w,Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        Self: 'w
    {
        let mut callback = AsWidgetClosure::new(#[inline] move |widget,root,ctx| {
            callback.call(widget, root, ctx)
        });
        (**self).with_widget(&mut callback,root,ctx)
    }
}
impl<T,E> AsWidget<E> for &'_ mut T where T: AsWidget<E> + ?Sized, E: Env {
    type Widget<'v,'z> = T::Widget<'v,'z> where 'z: 'v, Self: 'z;
    type WidgetCache = T::WidgetCache;

    #[inline]
    fn with_widget<'w,R>(&self, callback: &mut (dyn AsWidgetDispatch<'w,Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        Self: 'w
    {
        let mut callback = AsWidgetClosure::new(#[inline] move |widget,root,ctx| {
            callback.call(widget, root, ctx)
        });
        (**self).with_widget(&mut callback,root,ctx)
    }
}
impl<T,E> AsWidget<E> for Box<T> where T: AsWidget<E> + ?Sized, E: Env {
    type Widget<'v,'z> = T::Widget<'v,'z> where 'z: 'v, Self: 'z;
    type WidgetCache = T::WidgetCache;

    #[inline]
    fn with_widget<'w,R>(&self, callback: &mut (dyn AsWidgetDispatch<'w,Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        Self: 'w
    {
        let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
            callback.call(widget, root, ctx)
        });
        (**self).with_widget(&mut callback,root,ctx)
    }
}
impl<T,E> AsWidget<E> for std::rc::Rc<T> where T: AsWidget<E> + ?Sized, E: Env {
    type Widget<'v,'z> = T::Widget<'v,'z> where 'z: 'v, Self: 'z;
    type WidgetCache = T::WidgetCache;

    #[inline]
    fn with_widget<'w,R>(&self, callback: &mut (dyn AsWidgetDispatch<'w,Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        Self: 'w
    {
        let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
            callback.call(widget, root, ctx)
        });
        (**self).with_widget(&mut callback,root,ctx)
    }
}
impl<T,E> AsWidget<E> for std::sync::Arc<T> where T: AsWidget<E> + ?Sized, E: Env {
    type Widget<'v,'z> = T::Widget<'v,'z> where 'z: 'v, Self: 'z;
    type WidgetCache = T::WidgetCache;

    #[inline]
    fn with_widget<'w,R>(&self, callback: &mut (dyn AsWidgetDispatch<'w,Self,R,E>+'_), root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        Self: 'w
    {
        let mut callback = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
            callback.call(widget, root, ctx)
        });
        (**self).with_widget(&mut callback,root,ctx)
    }
}

#[inline] //TODO maybe Ext frontend trait workz?!
pub fn with_as_widget<'z,W,C,R,E>(
    w: &'z W, c: C, root: E::RootRef<'_>, ctx: &mut E::Context<'_>
) -> R
where
    W: AsWidget<E> + ?Sized + 'z,
    E: Env,
    for<'w,'ww,'r,'c,'cc> C: FnMut(&'w W::Widget<'ww,'z>,E::RootRef<'r>,&'c mut E::Context<'cc>) -> R,
{
    w.with_widget(
        &mut AsWidgetClosure::new(c),
        root, ctx,
    )
}

// pub trait AsWidgetDyn<E> where E: Env {
//     fn with_widget_dyn<'w>(&'w self, f: Box<dyn for<'r,'s> FnOnce(&'r (dyn WidgetDyn<E>+'s))>, root: E::RootRef<'_>, ctx: &mut E::Context<'_>);
// }

// impl<T,E> AsWidgetDyn<E> for T where T: AsWidget<E>, E: Env {
//     fn with_widget_dyn<'w>(&'w self, f: Box<dyn for<'r,'s> FnOnce(&'r (dyn WidgetDyn<E>+'s))>, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) {
//         self.with_widget(f, root, ctx)
//     }
// }

// impl<'l,E> AsWidget<E> for dyn AsWidgetDyn<E> + 'l where E: Env {
//     type Widget<'k> = dyn WidgetDyn<E>+'k;

//     fn with_widget<'w>(&'w self, f: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>)
//     where
//         F: for<'r,'s> FnOnce(&'r Self::Widget<'s>)
//     {
//         self.with_widget_dyn(f, root, ctx)
//     }
// }

/// Implement AsWidget for a Widget
#[macro_export]
macro_rules! impl_as_widget_self {
    (
        $e:ident;
        ($($args:tt)*)
        $lt:lifetime $typ:ty
        $(where $($preds:tt)+)?
    ) => {
        impl<$($args)*> $crate::widget::as_widget::AsWidget<$lt,$e> for $typ where $e: $crate::env::Env, Self: $lt $(, $($preds)*)? {
            type Widget<'__impl_as_widget_self_v> = Self where $lt: '__impl_as_widget_self_v;

            #[inline]
            fn with_widget<'__impl_as_widget_self_w,F,R>(&'__impl_as_widget_self_w self, callback: F, root: <E as $crate::env::Env>::RootRef<'_>, ctx: &mut <E as $crate::env::Env>::Ctx<'_>) -> R
            where
                F: AsWidgetDispatch<$lt,Self,R,E>
            {
                callback.call(self, root, ctx)
            }
        }
    };
}
