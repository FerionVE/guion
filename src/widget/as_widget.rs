use std::borrow::Borrow;
use std::ops::Deref;

use crate::aliases::WidgetRef;
use crate::env::Env;

use super::Widget;

pub trait AsWidget<E> where E: Env {
    type Widget: Widget<E> + ?Sized;
    type WidgetOwned: Borrow<Self::Widget> + Sized;

    fn as_widget<'w>(&'w self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> WCow<'w,Self::Widget,Self::WidgetOwned> where Self: 'w;
    fn into_widget<'w>(self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> WCow<'w,Self::Widget,Self::WidgetOwned> where Self: Sized + 'w;
    #[inline]
    fn box_into_widget<'w>(self: Box<Self>, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> WCow<'w,Self::Widget,Self::WidgetOwned> where Self: 'w {
        self.into_widget(root, ctx)
    }

    fn as_widget_dyn<'w,'s>(&'w self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> DynWCow<'w,E> where Self: 'w;
    fn into_widget_dyn<'w,'s>(self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> DynWCow<'w,E> where Self: Sized + 'w;
    fn box_into_widget_dyn<'w,'s>(self: Box<Self>, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> DynWCow<'w,E> where Self: 'w;
}

impl<'a,E> AsWidget<E> for dyn Widget<E> + 'a where E: Env {
    type Widget = dyn Widget<E>+'a;
    type WidgetOwned = Box<dyn Widget<E>+'a>;

    fn as_widget<'w>(&'w self, _: E::RootRef<'_>, _: &mut E::Context<'_>) -> WCow<'w,Self::Widget,Self::WidgetOwned> where Self: 'w {
        WCow::Borrowed(self)
    }
    fn into_widget<'w>(self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> WCow<'w,Self::Widget,Self::WidgetOwned> where Self: Sized + 'w {
        WCow::Owned(Box::new(self))
    }
    fn box_into_widget<'w>(self: Box<Self>, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> WCow<'w,Self::Widget,Self::WidgetOwned> where Self: 'w {
        WCow::Owned(self)
    }

    fn as_widget_dyn<'w,'s>(&'w self, root: <E as Env>::RootRef<'_>, ctx: &mut <E as Env>::Context<'_>) -> DynWCow<'w,E> where Self: 'w {
        WCow::Borrowed(self)
    }
    fn into_widget_dyn<'w,'s>(self, root: <E as Env>::RootRef<'_>, ctx: &mut <E as Env>::Context<'_>) -> DynWCow<'w,E> where Self: Sized + 'w {
        WCow::Owned(Box::new(self))
    }
    fn box_into_widget_dyn<'w,'s>(self: Box<Self>, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> DynWCow<'w,E> where Self: 'w {
        WCow::Owned(self)
    }
}

impl<T,E> AsWidget<E> for &T where T: AsWidget<E> + ?Sized, E: Env {
    type Widget = T::Widget;
    type WidgetOwned = T::WidgetOwned;

    fn as_widget<'w>(&'w self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> WCow<'w,Self::Widget,Self::WidgetOwned> where Self: 'w {
        (**self).as_widget(root,ctx)
    }
    fn into_widget<'w>(self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> WCow<'w,Self::Widget,Self::WidgetOwned> where Self: Sized + 'w {
        (*self).as_widget(root,ctx)
    }

    fn as_widget_dyn<'w,'s>(&'w self, root: <E as Env>::RootRef<'_>, ctx: &mut <E as Env>::Context<'_>) -> DynWCow<'w,E> where Self: 'w {
        (**self).as_widget_dyn(root,ctx)
    }
    fn into_widget_dyn<'w,'s>(self, root: <E as Env>::RootRef<'_>, ctx: &mut <E as Env>::Context<'_>) -> DynWCow<'w,E> where Self: Sized + 'w {
        (*self).as_widget_dyn(root,ctx)
    }
    fn box_into_widget_dyn<'w,'s>(self: Box<Self>, root: <E as Env>::RootRef<'_>, ctx: &mut <E as Env>::Context<'_>) -> DynWCow<'w,E> where Self: 'w {
        self.into_widget_dyn(root,ctx)
    }
}
impl<T,E> AsWidget<E> for &mut T where T: AsWidget<E> + ?Sized, E: Env {
    type Widget = T::Widget;
    type WidgetOwned = T::WidgetOwned;

    fn as_widget<'w>(&'w self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> WCow<'w,Self::Widget,Self::WidgetOwned> where Self: 'w {
        (**self).as_widget(root,ctx)
    }
    fn into_widget<'w>(self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> WCow<'w,Self::Widget,Self::WidgetOwned> where Self: Sized + 'w {
        (*self).as_widget(root,ctx)
    }

    fn as_widget_dyn<'w,'s>(&'w self, root: <E as Env>::RootRef<'_>, ctx: &mut <E as Env>::Context<'_>) -> DynWCow<'w,E> where Self: 'w {
        (**self).as_widget_dyn(root,ctx)
    }
    fn into_widget_dyn<'w,'s>(self, root: <E as Env>::RootRef<'_>, ctx: &mut <E as Env>::Context<'_>) -> DynWCow<'w,E> where Self: Sized + 'w {
        (*self).as_widget_dyn(root,ctx)
    }
    fn box_into_widget_dyn<'w,'s>(self: Box<Self>, root: <E as Env>::RootRef<'_>, ctx: &mut <E as Env>::Context<'_>) -> DynWCow<'w,E> where Self: 'w {
        self.into_widget_dyn(root,ctx)
    }
}
impl<T,E> AsWidget<E> for Box<T> where T: AsWidget<E> + ?Sized, E: Env {
    type Widget = T::Widget;
    type WidgetOwned = T::WidgetOwned;

    fn as_widget<'w>(&'w self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> WCow<'w,Self::Widget,Self::WidgetOwned> where Self: 'w {
        (**self).as_widget(root,ctx)
    }
    fn into_widget<'w>(self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> WCow<'w,Self::Widget,Self::WidgetOwned> where Self: Sized + 'w {
        <T as AsWidget<E>>::box_into_widget(self, root, ctx)
    }

    fn as_widget_dyn<'w,'s>(&'w self, root: <E as Env>::RootRef<'_>, ctx: &mut <E as Env>::Context<'_>) -> DynWCow<'w,E> where Self: 'w {
        (**self).as_widget_dyn(root,ctx)
    }
    fn into_widget_dyn<'w,'s>(self, root: <E as Env>::RootRef<'_>, ctx: &mut <E as Env>::Context<'_>) -> DynWCow<'w,E> where Self: Sized + 'w {
        <T as AsWidget<E>>::box_into_widget_dyn(self, root, ctx)
    }
    fn box_into_widget_dyn<'w,'s>(self: Box<Self>, root: <E as Env>::RootRef<'_>, ctx: &mut <E as Env>::Context<'_>) -> DynWCow<'w,E> where Self: 'w {
        self.into_widget_dyn(root,ctx)
    }
}
/*impl<T,E> AsWidget<E> for std::rc::Rc<T> where T: AsWidget<E> + ?Sized, E: Env {
    type Widget = T::Widget;
    type WidgetOwned = T::WidgetOwned;

    fn as_widget<'w>(&'w self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> WCow<'w,Self::Widget,Self::WidgetOwned> {
        (**self).as_widget(root,ctx)
    }
    fn into_widget<'w>(self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> WCow<'w,Self::Widget,Self::WidgetOwned> where Self: Sized + 'w {
        (*self).as_widget(root,ctx)
    }
}
impl<T,E> AsWidget<E> for std::sync::Arc<T> where T: AsWidget<E> + ?Sized, E: Env {
    type Widget = T::Widget;
    type WidgetOwned = T::WidgetOwned;

    fn as_widget<'w>(&'w self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> WCow<'w,Self::Widget,Self::WidgetOwned> {
        (**self).as_widget(root,ctx)
    }
    fn into_widget<'w>(self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> WCow<'w,Self::Widget,Self::WidgetOwned> where Self: Sized + 'w {
        (*self).as_widget(root,ctx)
    }
}*/

pub trait AsWidgetImplemented<E> {}

impl<T,E> AsWidgetImplemented<E> for T where T: AsWidget<E> + ?Sized, E: Env {}

pub type DynWCow<'w,E> = WCow<'w,dyn Widget<E>+'w,Box<dyn Widget<E>+'w>>; 

#[doc(hidden)]
pub enum WCow<'a,T,U> where T: ?Sized + 'a, U: Borrow<T> + Sized + 'a {
    Borrowed(&'a T),
    Owned(U),
}

impl<'a,T,U> WCow<'a,T,U> where T: ?Sized + 'a, U: Borrow<T> + Sized + 'a {
    pub fn reference<'s>(&'s self) -> WCow<'s,T,U> where Self: 's {
        match self {
            WCow::Borrowed(t) => WCow::Borrowed(*t),
            WCow::Owned(t) => WCow::Borrowed(t.borrow()),
        }
    }
}

//TODO also for generic WCow<'_,T,U>
impl<'a,E> WidgetRef<'a,E> where E: Env {
    pub fn into_resolve(self, i: E::WidgetPath, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> Result<WidgetRef<'a,E>,E::Error> {
        match self {
            WCow::Borrowed(t) => t.resolve(i,root,ctx),
            WCow::Owned(t) => t.into_resolve(i,root,ctx),
        }
    }
}

impl<'a,T,U> Deref for WCow<'a,T,U> where T: ?Sized + 'a, U: Borrow<T> + Sized + 'a {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            WCow::Borrowed(t) => *t,
            WCow::Owned(t) => t.borrow(),
        }
    }
}

impl<'a,T,U> Borrow<T> for WCow<'a,T,U> where T: ?Sized + 'a, U: Borrow<T> + Sized + 'a{
    fn borrow(&self) -> &T {
        match self {
            WCow::Borrowed(t) => *t,
            WCow::Owned(t) => t.borrow(),
        }
    }
}

impl<'a,T,U> AsRef<T> for WCow<'a,T,U> where T: ?Sized + 'a, U: Borrow<T> + Sized + 'a {
    fn as_ref(&self) -> &T {
        match self {
            WCow::Borrowed(t) => *t,
            WCow::Owned(t) => t.borrow(),
        }
    }
}
