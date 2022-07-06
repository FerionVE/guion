use std::borrow::Borrow;
use std::marker::PhantomData;
use std::ops::Deref;
use super::*;
use super::dyn_tunnel::WidgetDyn;

use crate::dispatchor::{AsWidgetDispatch, AsWidgetClosure};
use crate::env::Env;

pub trait AsWidget<'z,E> where E: Env, Self: 'z {
    type Widget<'v>: Widget<E> + ?Sized + 'v where 'z: 'v;

    fn with_widget<'w,F,R>(&'w self, f: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetDispatch<'z,Self,R,E>;
}

impl<'a,E> AsWidget<'a,E> for dyn WidgetDyn<E> + 'a where E: Env {
    type Widget<'v> = dyn WidgetDyn<E>+'v where 'a: 'v;

    #[inline]
    fn with_widget<'w,F,R>(&'w self, f: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetDispatch<'a,Self,R,E>
    {
        f.call(&*self, root, ctx)
    }
}

impl<'z,T,E> AsWidget<'z,E> for &'z T where T: AsWidget<'z,E> + ?Sized, E: Env {
    type Widget<'v> = T::Widget<'v> where 'z: 'v;

    #[inline]
    fn with_widget<'w,F,R>(&'w self, f: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetDispatch<'z,Self,R,E>
    {
        let dis = AsWidgetClosure::new(#[inline] move |widget,root,ctx| {
            f.call(&widget, root, ctx)
        });
        (**self).with_widget(dis,root,ctx)
    }
}
impl<'z,T,E> AsWidget<'z,E> for &'z mut T where T: AsWidget<'z,E> + ?Sized, E: Env {
    type Widget<'v> = T::Widget<'v> where 'z: 'v;

    #[inline]
    fn with_widget<'w,F,R>(&'w self, f: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetDispatch<'z,Self,R,E>
    {
        let dis = AsWidgetClosure::new(#[inline] move |widget,root,ctx| {
            f.call(&widget, root, ctx)
        });
        (**self).with_widget(dis,root,ctx)
    }
}
impl<'z,T,E> AsWidget<'z,E> for Box<T> where T: AsWidget<'z,E> + ?Sized, E: Env {
    type Widget<'v> = T::Widget<'v> where 'z: 'v;

    #[inline]
    fn with_widget<'w,F,R>(&'w self, f: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetDispatch<'z,Self,R,E>
    {
        let dis = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
            f.call(widget, root, ctx)
        });
        (**self).with_widget(dis,root,ctx)
    }
}
impl<'z,T,E> AsWidget<'z,E> for std::rc::Rc<T> where T: AsWidget<'z,E> + ?Sized, E: Env {
    type Widget<'v> = T::Widget<'v> where 'z: 'v;

    #[inline]
    fn with_widget<'w,F,R>(&'w self, f: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetDispatch<'z,Self,R,E>
    {
        let dis = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
            f.call(widget, root, ctx)
        });
        (**self).with_widget(dis,root,ctx)
    }
}
impl<'z,T,E> AsWidget<'z,E> for std::sync::Arc<T> where T: AsWidget<'z,E> + ?Sized, E: Env {
    type Widget<'v> = T::Widget<'v> where 'z: 'v;

    #[inline]
    fn with_widget<'w,F,R>(&'w self, f: F, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> R
    where
        F: AsWidgetDispatch<'z,Self,R,E>
    {
        let dis = AsWidgetClosure::new(#[inline] |widget,root,ctx| {
            f.call(widget, root, ctx)
        });
        (**self).with_widget(dis,root,ctx)
    }
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
            fn with_widget<'__impl_as_widget_self_w,F,R>(&'__impl_as_widget_self_w self, dispatch: F, root: <E as $crate::env::Env>::RootRef<'_>, ctx: &mut <E as $crate::env::Env>::Ctx<'_>) -> R
            where
                F: AsWidgetDispatch<$lt,Self,R,E>
            {
                dispatch.call(self, root, ctx)
            }
        }
    };
}

pub(crate) use impl_as_widget_self;
