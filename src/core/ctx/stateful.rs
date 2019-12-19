use super::*;

pub trait AsHandlerStateful<E> where E: Env, E::Context: Context<Link=Self> + Widgets<E>, <E::Context as Context>::Link: for<'a> From<&'a mut E::Context> + AsMut<E::Context> + AsMut<<E::Context as Context>::Handler> + AsMut<Self::T> {
    type T: HandlerStateful<E>;
    #[inline]
    fn stateful_mut(&mut self) -> &mut Self::T {
        self.as_mut()
    }
}

pub trait HandlerStateful<E>: Handler<E::Context> + 'static where E: Env, <E::Context as Context>::Link: AsMut<Self> {
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