//! Types which can be referenced/casted as [`Widget`] or [`Path`](Env::WidgetPath)
use super::*;

/// AsWidget is an object which can interpret as [`Widget`] or a [`Path`](Env::WidgetPath)  
/// [Example implementation for immediate widget](https://github.com/FerionVE/guion_sdl2/blob/544f045168f0960838f3cae1b46a2ea8d8afe361/src/simple/immediate_test.rs#L16) 
#[doc(alias="immediate")]#[doc(alias="immediat")]#[doc(alias="immedia")]#[doc(alias="immedi")]#[doc(alias="immed")]
pub trait AsWidget<E> where E: Env {
    /// ![IMPL](https://img.shields.io/badge/-impl-important?style=flat-square)
    /// [`Resolvable::from_widget`] can be used to create a [`Resolvable`] from a (immediate) [`Widget`]
    fn as_ref<'s>(&'s self) -> Resolvable<'s,E>;
    /// ![IMPL](https://img.shields.io/badge/-impl-important?style=flat-square)
    /// [`Resolvable::from_widget`] can be used to create a [`Resolvable`] from a (immediate) [`Widget`]
    fn into_ref<'w>(self) -> Resolvable<'w,E> where Self: 'w;
}
/// AsWidgetMut is an object which can interpret as [`Widget`] or a [`Path`](Env::WidgetPath)  
/// [Example implementation for immediate widget](https://github.com/FerionVE/guion_sdl2/blob/544f045168f0960838f3cae1b46a2ea8d8afe361/src/simple/immediate_test.rs#L16) 
#[doc(alias="immediate")]#[doc(alias="immediat")]#[doc(alias="immedia")]#[doc(alias="immedi")]#[doc(alias="immed")]
pub trait AsWidgetMut<E>: AsWidget<E> where E: Env {
    /// ![IMPL](https://img.shields.io/badge/-impl-important?style=flat-square)
    /// [`ResolvableMut::from_widget`] can be used to create a [`ResolvableMut`] from a (immediate) [`WidgetMut`]
    fn as_mut<'s>(&'s mut self) -> ResolvableMut<'s,E>;
    /// ![IMPL](https://img.shields.io/badge/-impl-important?style=flat-square)
    /// [`ResolvableMut::from_widget`] can be used to create a [`ResolvableMut`] from a (immediate) [`WidgetMut`]
    fn into_mut<'w>(self) -> ResolvableMut<'w,E> where Self: 'w;
}

impl<E,T> AsWidget<E> for T where T: Widget<E>, E: Env {
    #[inline]
    fn as_ref(&self) -> Resolvable<E> {
        Resolvable::Widget(self.box_ref())
    }
    #[inline]
    fn into_ref<'w>(self) -> Resolvable<'w,E> where Self: 'w {
        Resolvable::Widget(Box::new(self))
    }
}
impl<E,T> AsWidgetMut<E> for T where T: WidgetMut<E>, E: Env {
    #[inline]
    fn as_mut(&mut self) -> ResolvableMut<E> {
        ResolvableMut::Widget(self.box_mut())
    }
    #[inline]
    fn into_mut<'w>(self) -> ResolvableMut<'w,E> where Self: 'w {
        ResolvableMut::Widget(Box::new(self))
    }
}
