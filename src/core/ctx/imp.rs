use super::*;

impl Handler for () {
    type Child = ();
    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _render<E: Env>(senf: &mut E::Context, i: &E::WidgetID, r: E::Renderer) {
        (senf.widget_fns(i).render)(senf.link(i),r)
    }
    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _event<E: Env>(senf: &mut E::Context, i: &E::WidgetID, e: E::Event) {
        (senf.widget_fns(i).event)(senf.link(i),e)
    }
    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _size<E: Env>(senf: &mut E::Context, i: &E::WidgetID) -> Size {
        (senf.widget_fns(i).size)(senf.link(i))
    }

    #[inline]
    fn _child_mut(&mut self) -> &mut Self::Child {
        unreachable!("Handler::_child_mut shouldn't be called from external")
    }
    #[inline]
    fn child_mut(&mut self) -> Option<&mut Self::Child> {
        None
    }

    #[inline]
    fn _child(&self) -> &Self::Child {
        unreachable!("Handler::_child shouldn't be called from external")
    }
    #[inline]
    fn child(&self) -> Option<&Self::Child> {
        None
    }
}
//TODO Look out!! This is part of the way to solve the DerefMut<Handler> problem
impl<T,E> HandlerStateful<E> for T where T: HandlerWithChild, E: Env, T::Child: HandlerStateful<E> {
    #[inline] fn hovered(&self) -> Option<E::WidgetID> {
        self.child().expect("HandlerWithChild but doesn't child").hovered()
    }
    #[inline] fn selected(&self) -> Option<E::WidgetID> {
        self.child().expect("HandlerWithChild but doesn't child").selected()
    }
}

/*pub struct Reee;

impl Context for Reee {

}

pub trait AsHandler<C>: Context where C: Handler<Self> {
    fn handler_mut(&mut self) -> &mut C;
}

impl<E> AsHandler<E::Handler> for Option<E> where E: Env {
    fn handler_mut(&mut self) -> &mut E::Handler {
        self.handler_mut()
    }
}

impl<E> AsHandler<<E::Handler as Handler<E>>::Child> for Option<E> where E: Env {
    fn handler_mut(&mut self) -> &mut <E::Handler as Handler<E>>::Child {
        self.handler_mut()
    }
}*/

/*pub trait HandlerOf<E>: Handler<E> where E: Env {
    fn handler_mut(r: &mut E) -> &mut Self;
}

impl<E> HandlerOf<E> for E::Handler where E: Env {
    fn handler_mut(r: &mut E) -> &mut Self {
        r.handler_mut()
    }
}

impl<E> HandlerOf<E> for <E::Handler as Handler<E>>::Child where E: Env {
    fn handler_mut(r: &mut E) -> &mut Self {
        r.handler_mut()._child_mut()
    }
}*/