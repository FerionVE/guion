use super::*;

/// AsWidget is an object which can interpret as Widget OR an Path
pub trait AsWidget<E> where E: Env {
    fn as_ref(&self) -> Resolvable<E>;
    fn as_mut(&mut self) -> ResolvableMut<E>;
}
pub trait AsWidgetImmediate<'a,E> where E: Env {
    fn into_ref(self) -> Resolvable<'a,E>;
    fn as_ref<'s>(&'s self) -> Resolvable<'s,E> where 'a: 's;
}
pub trait AsWidgetImmediateMut<'a,E> where E: Env {
    fn into_mut(self) -> ResolvableMut<'a,E>;
    fn as_mut<'s>(&'s mut self) -> ResolvableMut<'s,E> where 'a: 's;
}


impl<E,T> AsWidget<E> for T where T: Widget<E>, E: Env {
    fn as_ref(&self) -> Resolvable<E> {
        Resolvable::Widget(Rc::new(self.as_immediate()))
    }
    fn as_mut(&mut self) -> ResolvableMut<E> {
        ResolvableMut::Widget(self.as_immediate_mut())
    }
}
/*impl<'w,E,T> AsWidgetImmediate<'w,E> for T where T: WidgetImmediate<'w,E> + 'static, E: Env {
    fn as_ref(self) -> Resolvable<'w,E> {
        Resolvable::Widget(Rc::new(Box::new(self)))
    }
}*/
impl<'w,E> AsWidgetImmediate<'w,E> for WidgetRef<'w,E> where E: Env {
    fn into_ref(self) -> Resolvable<'w,E> {
        Resolvable::Widget(Rc::new(self))
    }
    fn as_ref<'s>(&'s self) -> Resolvable<'s,E> where 'w: 's {
        Resolvable::Widget(Rc::new(self.cloned()))
    }
}
impl<'w,E> AsWidgetImmediate<'w,E> for Rc<WidgetRef<'w,E>> where E: Env {
    fn into_ref(self) -> Resolvable<'w,E> {
        Resolvable::Widget(self)
    }
    fn as_ref<'s>(&'s self) -> Resolvable<'s,E> where 'w: 's {
        Resolvable::Widget(Rc::new(self.cloned()))
    }
}
impl<'w,E> AsWidgetImmediateMut<'w,E> for WidgetRefMut<'w,E> where E: Env {
    fn into_mut(self) -> ResolvableMut<'w,E> {
        ResolvableMut::Widget(self)
    }
    fn as_mut<'s>(&'s mut self) -> ResolvableMut<'s,E> where 'w: 's {
        ResolvableMut::Widget(self.cloned_mut())
    }
}