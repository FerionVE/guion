use super::*;

pub trait WidgetID: Clone + PartialEq + Sized + 'static {
    #[inline]
    fn id_eq<I: WidgetID + 'static>(&self, o: &I) -> bool where Self: 'static {
        Any::downcast_ref::<Self>(o)
            .map_or(false, |r| self.eq(r) )
    }

    #[inline]
    fn is_hovered<E: Env<WidgetID=Self>>(&self, c: &E::Context) -> bool where ECHandler<E>: AsHandlerStateful<E>, EPressedKey<E>: PressedKey<E> {
        c.state().is_hovered(self)
    }
    #[inline]
    fn is_selected<E: Env<WidgetID=Self>>(&self, c: &E::Context) -> bool where ECHandler<E>: AsHandlerStateful<E>, EPressedKey<E>: PressedKey<E> {
        c.state().is_selected(self)
    }
}

/*impl WidgetID for Vec<Box<dyn Any>> {
    
}*/