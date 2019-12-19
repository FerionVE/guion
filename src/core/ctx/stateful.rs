use super::*;

pub trait AsHandlerStateful<E,C>: Sized where E: Env<Context=C>, C: Context<Link=Self> + Widgets<E>, <E::Context as Context>::Link: AsHandler<Self::T,C> + AsHandler<C::Handler,C> {
    type T: HandlerStateful<E,C>;
    #[inline]
    fn stateful_mut(e: &mut C) -> &mut Self::T {
        AsHandler::as_mut(e)
    }
    #[inline]
    fn stateful(e: &C) -> &Self::T {
        AsHandler::as_ref(e)
    }
} 

pub trait HandlerStateful<E,C>: Handler<C> + 'static where E: Env<Context=C>, C: Context + Widgets<E>, C::Link: AsHandler<Self,C> {
    #[inline] fn hovered(&self) -> Option<E::WidgetID> {
        None
    }
    #[inline] fn selected(&self) -> Option<E::WidgetID> {
        None
    }

    #[inline]
    fn is_hovered(&self, i: &E::WidgetID) -> bool {
        self.hovered().map_or(false, |w| w == *i )
    }
    #[inline]
    fn is_selected(&self, i: &E::WidgetID) -> bool {
        self.selected().map_or(false, |w| w == *i )
    }
}