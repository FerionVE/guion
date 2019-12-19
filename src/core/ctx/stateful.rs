use super::*;

pub trait AsHandlerStateful<E>: Sized where E: Env, E::Context: Context<Link=Self> + Widgets<E>, <E::Context as Context>::Link: for<'a> HandlerAccess<'a,E::Context> + for<'a> AsHandler<'a,Self::T> {
    type T: HandlerStateful<E>;
    #[inline]
    fn stateful_mut(&mut self) -> &mut Self::T {
        AsHandler::as_mut(self)
    }
}

pub trait HandlerStateful<E>: Handler<E::Context> + 'static where E: Env, <E::Context as Context>::Link: for<'a> AsHandler<'a,Self> {
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