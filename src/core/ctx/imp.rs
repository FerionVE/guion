use super::*;

impl<E> ContextLayer<E> for () where E: Context {
    type Child = ();
    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _render(senf: &mut E, i: &E::WidgetID, r: E::Renderer) {
        (senf.widget_fns(i).render)(senf.link(i),r)
    }
    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _event(senf: &mut E, i: &E::WidgetID, e: E::Event) {
        (senf.widget_fns(i).event)(senf.link(i),e)
    }
    /// PANICKS if widget doesn't exists
    #[inline] 
    fn _size(senf: &mut E, i: &E::WidgetID) -> Size {
        (senf.widget_fns(i).size)(senf.link(i))
    }
    #[inline]
    fn _child_mut(&mut self) -> &mut Self::Child {
        unreachable!()
    }
    #[inline]
    fn ref_of<L: ContextLayer<E>>(&mut self) -> Option<&mut L> {
        if Any::is::<L>(self) {
            Any::downcast_mut::<L>(self)
        }else{
            None
        }
    }

    #[inline]
    fn get_self(senf: &mut E) -> Option<&mut Self> {
        senf.get_handler()
    }
}

/*pub struct Reee;

impl Context for Reee {

}

pub trait AsHandler<C>: Context where C: ContextLayer<Self> {
    fn handler_mut(&mut self) -> &mut C;
}

impl<E> AsHandler<E::Handler> for Option<E> where E: Context {
    fn handler_mut(&mut self) -> &mut E::Handler {
        self.handler_mut()
    }
}

impl<E> AsHandler<<E::Handler as ContextLayer<E>>::Child> for Option<E> where E: Context {
    fn handler_mut(&mut self) -> &mut <E::Handler as ContextLayer<E>>::Child {
        self.handler_mut()
    }
}*/

/*pub trait HandlerOf<E>: ContextLayer<E> where E: Context {
    fn handler_mut(r: &mut E) -> &mut Self;
}

impl<E> HandlerOf<E> for E::Handler where E: Context {
    fn handler_mut(r: &mut E) -> &mut Self {
        r.handler_mut()
    }
}

impl<E> HandlerOf<E> for <E::Handler as ContextLayer<E>>::Child where E: Context {
    fn handler_mut(r: &mut E) -> &mut Self {
        r.handler_mut()._child_mut()
    }
}*/