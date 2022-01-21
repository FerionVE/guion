use std::borrow::Borrow;
use std::ops::Deref;

use crate::env::Env;

use super::Widget;

pub trait AsWidget<E> where E: Env {
    type Widget: Widget<E> + WCTSized + ?Sized;

    fn as_widget<'w>(&'w self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> WCow<'w,Self::Widget>;
}

/*impl<T,E> AsWidget<E> for T where T: Widget<E> {
    default type Widget = T;

    default fn as_widget<'w>(&'w self, _: &mut E::Context<'_>) -> WCow<'w,Self::Widget> {
        WCow::Borrowed(self)
    }
}*/

impl<'a,E> AsWidget<E> for dyn Widget<E> + 'a where E: Env {
    type Widget = dyn Widget<E>+'a;

    fn as_widget<'w>(&'w self, _: E::RootRef<'_>, _: &mut E::Context<'_>) -> WCow<'w,Self::Widget> {
        WCow::Borrowed(self)
    }
}

impl<T,E> AsWidget<E> for &T where T: AsWidget<E> + ?Sized, E: Env {
    type Widget = T::Widget;

    fn as_widget<'w>(&'w self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> WCow<'w,Self::Widget> {
        (**self).as_widget(root,ctx)
    }
}
impl<T,E> AsWidget<E> for &mut T where T: AsWidget<E> + ?Sized, E: Env {
    type Widget = T::Widget;

    fn as_widget<'w>(&'w self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> WCow<'w,Self::Widget> {
        (**self).as_widget(root,ctx)
    }
}
impl<T,E> AsWidget<E> for Box<T> where T: AsWidget<E> + ?Sized, E: Env {
    type Widget = T::Widget;

    fn as_widget<'w>(&'w self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> WCow<'w,Self::Widget> {
        (**self).as_widget(root,ctx)
    }
}
impl<T,E> AsWidget<E> for std::rc::Rc<T> where T: AsWidget<E> + ?Sized, E: Env {
    type Widget = T::Widget;

    fn as_widget<'w>(&'w self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> WCow<'w,Self::Widget> {
        (**self).as_widget(root,ctx)
    }
}
impl<T,E> AsWidget<E> for std::sync::Arc<T> where T: AsWidget<E> + ?Sized, E: Env {
    type Widget = T::Widget;

    fn as_widget<'w>(&'w self, root: E::RootRef<'_>, ctx: &mut E::Context<'_>) -> WCow<'w,Self::Widget> {
        (**self).as_widget(root,ctx)
    }
}

// epic hack to softly force Widgets to implement AsWidget without being subset, required for dyn Widget as we can't directly explain the associated type
pub trait AsWidgetImplemented<E> {}

impl<T,E> AsWidgetImplemented<E> for T where T: AsWidget<E> + ?Sized, E: Env {}

#[doc(hidden)]
pub enum WCow<'a,T> where T: WCTSized + ?Sized + 'a {
    Borrowed(&'a T),
    Owned(T::Sized),
}

impl<'a,T> WCow<'a,T> where T: WCTSized + ?Sized + 'a {
    pub fn erase<E>(self) -> WCow<'a,dyn Widget<E>+'a> where T: WCTErase<'a,E> {
        match self {
            WCow::Borrowed(t) => WCow::Borrowed(t.erase_ref()),
            WCow::Owned(t) => WCow::Owned(T::erase_owned(t)),
        }
    }
}

// epic hack to allow AsWidget for dyn Widget
#[doc(hidden)]
pub trait WCTSized {
    type Sized: Sized + Borrow<Self>;
}

impl<T> WCTSized for T where T: Sized {
    type Sized = T;
}
impl<'a,E> WCTSized for dyn Widget<E> + 'a {
    type Sized = Box<dyn Widget<E>+'a>;
}

pub trait WCTErase<'a,E>: WCTSized {
    fn erase_ref(&self) -> &(dyn Widget<E>+'a);
    fn erase_owned(s: Self::Sized) -> Box<dyn Widget<E>+'a>;
}

impl<'a,T,E> WCTErase<'a,E> for T where T: Widget<E> + Sized + 'a, E: Env {
    fn erase_ref(&self) -> &(dyn Widget<E>+'a) {
        self
    }
    fn erase_owned(s: Self::Sized) -> Box<dyn Widget<E>+'a> {
        Box::new(s)
    }
}

impl<'a,E> WCTErase<'a,E> for dyn Widget<E> + 'a where E: Env {
    fn erase_ref(&self) -> &(dyn Widget<E>+'a) {
        self
    }
    fn erase_owned(s: Self::Sized) -> Box<dyn Widget<E>+'a> {
        s
    }
}


impl<'a,T> Deref for WCow<'a,T> where T: WCTSized + ?Sized + 'a {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            WCow::Borrowed(t) => *t,
            WCow::Owned(t) => t.borrow(),
        }
    }
}

impl<'a,T> Borrow<T> for WCow<'a,T> where T: WCTSized + ?Sized + 'a {
    fn borrow(&self) -> &T {
        match self {
            WCow::Borrowed(t) => *t,
            WCow::Owned(t) => t.borrow(),
        }
    }
}

impl<'a,T> AsRef<T> for WCow<'a,T> where T: WCTSized + ?Sized + 'a {
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
