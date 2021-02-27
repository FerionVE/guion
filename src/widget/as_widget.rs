//! Types which can be referenced/casted as Widget or Path
use super::*;

/// AsWidget is an object which can interpret as Widget OR an Path
/// [Example implementation for immediate widget](https://github.com/FerionVE/guion_sdl2/blob/544f045168f0960838f3cae1b46a2ea8d8afe361/src/simple/immediate_test.rs#L17) 
pub trait AsWidget<E> where E: Env {
    /// ![IMPL](https://img.shields.io/badge/-impl-important?style=flat-square)  [`Resolvable::from_widget`](Resolvable::from_widget) can be used to create a [`Resolvable`] from a (immediate) Widget
    fn as_widget<'s>(&'s self) -> Resolvable<'s,E>;
    /// ![IMPL](https://img.shields.io/badge/-impl-important?style=flat-square)  [`Resolvable::from_widget`](Resolvable::from_widget) can be used to create a [`Resolvable`] from a (immediate) Widget
    fn as_widget_mut<'s>(&'s mut self) -> Resolvable<'s,E>;
    /// ![IMPL](https://img.shields.io/badge/-impl-important?style=flat-square)  [`Resolvable::from_widget`](Resolvable::from_widget) can be used to create a [`Resolvable`] from a (immediate) Widget
    fn into_widget<'w>(self) -> Resolvable<'w,E> where Self: 'w;
}

impl<E,T> AsWidget<E> for T where T: Widget<E>, E: Env {
    #[inline]
    fn as_widget(&self) -> Resolvable<E> {
        Resolvable::Widget(self.box_ref())
    }
    #[inline]
    fn as_widget_mut(&mut self) -> Resolvable<E> {
        Resolvable::Widget(self.box_ref())
    }
    #[inline]
    fn into_widget<'w>(self) -> Resolvable<'w,E> where Self: 'w {
        Resolvable::Widget(Box::new(self))
    }
}
