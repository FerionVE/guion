use std::borrow::Borrow;
use std::ops::Deref;

use crate::env::Env;

use super::Widget;

pub trait AsWidget<E> where E: Env {
    type Widget: Widget<E> + ?Sized;
    type WidgetOwned: Borrow<Self::Widget> + Sized;

    fn as_widget<'w>(&'w self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> WCow<'w,Self::Widget,Self::WidgetOwned> where Self: 'w;
    fn into_widget<'w>(self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> WCow<'w,Self::Widget,Self::WidgetOwned> where Self: Sized + 'w;
    fn box_into_widget<'w>(self: Box<Self>, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> WCow<'w,Self::Widget,Self::WidgetOwned> where Self: 'w {
        self.into_widget(root, ctx)
    }
}

/*impl<T,E> AsWidget<E> for T where T: Widget<E> {
    default type Widget = T;

    default fn as_widget<'w>(&'w self, _: &mut E::Context<'_>) -> WCow<'w,Self::Widget,Self::WidgetOwned> {
        WCow::Borrowed(self)
    }
}*/

impl<'a,E> AsWidget<E> for dyn Widget<E> + 'a where E: Env {
    type Widget = dyn Widget<E>+'a;
    type WidgetOwned = Box<dyn Widget<E>+'a>;

    fn as_widget<'w>(&'w self, _: E::RootRef<'_>, _: &mut E::Context<'_>) -> WCow<'w,Self::Widget,Self::WidgetOwned> where Self: 'w {
        WCow::Borrowed(self)
    }
    fn into_widget<'w>(self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> WCow<'w,Self::Widget,Self::WidgetOwned> where Self: Sized + 'w {
        WCow::Owned(Box::new(self))
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

#[doc(hidden)]
pub enum WCow<'a,T,U> where T: ?Sized + 'a, U: Borrow<T> + Sized + 'a {
    Borrowed(&'a T),
    Owned(U),
}

impl<'a,T,U> WCow<'a,T,U> where T: ?Sized + 'a, U: Borrow<T> + Sized + 'a {
    //TODO fix
    /*pub fn erase<E>(self) -> WCow<'a,dyn Widget<E>+'a,Box<dyn Widget<E>+'a>> where T: Widget<E>, E: Env {
        match self {
            WCow::Borrowed(t) => WCow::Borrowed(t.erase()),
            WCow::Owned(t) => WCow::Owned(t.boxx()),
        }
    }*/
    pub fn reference<'s>(&'s self) -> WCow<'s,T,U> where Self: 's {
        match self {
            WCow::Borrowed(t) => WCow::Borrowed(*t),
            WCow::Owned(t) => WCow::Borrowed(t.borrow()),
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

// fn sustest<AW,E>(a: &AW, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) where AW: AsWidget<E> + ?Sized, AW::Widget: RenderWidget<E>, E: Env {
//     a.as_widget(root,ctx).render(ctx)
// }
