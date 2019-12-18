use super::*;

//TODO Look out!! This is part of the way to solve the DerefMut<Handler> problem
/*impl<T,E> HandlerStateful<E> for T where T: HandlerWithChild, E: Env, T::Child: HandlerStateful<E> {
    #[inline] fn hovered(&self) -> Option<E::WidgetID> {
        self.child().expect("HandlerWithChild but doesn't child").hovered()
    }
    #[inline] fn selected(&self) -> Option<E::WidgetID> {
        self.child().expect("HandlerWithChild but doesn't child").selected()
    }
}*/

/*impl<T,U> AsHandler<U> for T where T: HandlerWithChild, T::Child: AsHandler<U>, U: Handler {
    fn handler_mut(&mut self) -> &mut U {
        self.hwc_child_mut().handler_mut()
    }
}*/

/*impl<T,U> AsHandler<U> for HandlerChild<T> where T: Handler + AsHandler<U>, U: Handler {
    fn handler_mut(&mut self) -> &mut U {
        self.v.handler_mut()
    }
}*/