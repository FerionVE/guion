//! Types which can be referenced/casted as Widget or Path
use super::*;

/// AsWidget is an object which can interpret as Widget OR an Path
pub trait AsWidget<'w,E> where E: Env {
    fn as_ref<'s>(&'s self) -> Resolvable<'s,E> where 'w: 's;
    fn into_ref(self) -> Resolvable<'w,E>;
}
pub trait AsWidgetMut<'w,E>: AsWidget<'w,E> where E: Env {
    fn as_mut<'s>(&'s mut self) -> ResolvableMut<'s,E> where 'w: 's;
    fn into_mut(self) -> ResolvableMut<'w,E>;
}

impl<'w,E,T> AsWidget<'w,E> for T where T: Widget<'w,E>, E: Env {
    fn as_ref<'s>(&'s self) -> Resolvable<'s,E> where 'w: 's {
        Resolvable::Widget(self.box_ref())
    }
    fn into_ref(self) -> Resolvable<'w,E> {
        Resolvable::Widget(Box::new(self))
    }
}
impl<'w,E,T> AsWidgetMut<'w,E> for T where T: WidgetMut<'w,E>, E: Env {
    fn as_mut<'s>(&'s mut self) -> ResolvableMut<'s,E> where 'w: 's {
        ResolvableMut::Widget(self.box_mut())
    }
    fn into_mut(self) -> ResolvableMut<'w,E> {
        ResolvableMut::Widget(Box::new(self))
    }
}