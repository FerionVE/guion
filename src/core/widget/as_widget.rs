use super::*;

/// AsWidget is an object which can interpret as Widget OR an Path
pub trait AsWidget<'a,E> where E: Env {
    fn as_ref<'s>(&'s self) -> Resolvable<'s,E> where 'a: 's;
    fn consume_ref(self) -> Resolvable<'a,E>;
}
pub trait AsWidgetMut<'a,E>: AsWidget<'a,E> where E: Env {
    fn as_mut<'s>(&'s mut self) -> ResolvableMut<'s,E> where 'a: 's;
    fn consume_mut(self) -> ResolvableMut<'a,E>;
}

impl<'a,E,T> AsWidget<'a,E> for T where T: Widget<'a,E>, E: Env {
    fn as_ref<'s>(&'s self) -> Resolvable<'s,E> where 'a: 's {
        Resolvable::Widget(self.box_ref())
    }
    fn consume_ref(self) -> Resolvable<'a,E> {
        Resolvable::Widget(Box::new(self))
    }
}
impl<'a,E,T> AsWidgetMut<'a,E> for T where T: WidgetMut<'a,E>, E: Env {
    fn as_mut<'s>(&'s mut self) -> ResolvableMut<'s,E> where 'a: 's {
        ResolvableMut::Widget(self.box_mut())
    }
    fn consume_mut(self) -> ResolvableMut<'a,E> {
        ResolvableMut::Widget(Box::new(self))
    }
}