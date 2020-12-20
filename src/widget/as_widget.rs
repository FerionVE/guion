//! Types which can be referenced/casted as Widget or Path
use super::*;

/// AsWidget is an object which can interpret as Widget OR an Path
pub trait AsWidget<E> where E: Env {
    fn as_ref<'s>(&'s self) -> Resolvable<'s,E>;
    fn into_ref<'w>(self) -> Resolvable<'w,E> where Self: 'w;
}
pub trait AsWidgetMut<E>: AsWidget<E> where E: Env {
    fn as_mut<'s>(&'s mut self) -> ResolvableMut<'s,E>;
    fn into_mut<'w>(self) -> ResolvableMut<'w,E> where Self: 'w;
}

impl<E,T> AsWidget<E> for T where T: Widget<E>, E: Env {
    #[inline]
    fn as_ref(&self) -> Resolvable<'_,E> {
        Resolvable::Widget(self.box_ref())
    }
    #[inline]
    fn into_ref<'w>(self) -> Resolvable<'w,E> where Self: 'w {
        Resolvable::Widget(Box::new(self))
    }
}
impl<E,T> AsWidgetMut<E> for T where T: WidgetMut<E>, E: Env {
    #[inline]
    fn as_mut(&mut self) -> ResolvableMut<'_,E> {
        ResolvableMut::Widget(self.box_mut())
    }
    #[inline]
    fn into_mut<'w>(self) -> ResolvableMut<'w,E> where Self: 'w {
        ResolvableMut::Widget(Box::new(self))
    }
}
