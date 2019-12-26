use crate::core::ctx::aliases::*;
use crate::core::util::bounds::Bounds;
use crate::core::*;
use ctx::*;
use lazout::size::Size;
use super::*;

impl<S,C> Handler<C> for StandardCtx<S,C> where S: Handler<C>, C: Context, C::Handler: AsHandler<Self,C> + 'static {
    #[inline] 
    fn _render<E: Env>(senf: &mut C, i: &E::WidgetID, r: (&mut ERenderer<E>,&Bounds)) {
        S::_render::<E>(senf,i,r);
        unimplemented!()
    }
    #[inline] 
    fn _event<E: Env>(senf: &mut C, i: &E::WidgetID, e: EEvent<E>) {
        S::_event::<E>(senf,i,e);
        unimplemented!()
    }
    #[inline] 
    fn _event_root<E: Env>(senf: &mut C, i: &E::WidgetID, e: EEvent<E>) {
        Self::_event::<E>(senf,i,e);
        unimplemented!()
    }
    #[inline] 
    fn _size<E: Env>(senf: &mut C, i: &E::WidgetID) -> Size {
        unimplemented!();
        S::_size::<E>(senf,i)
    }
}

impl<S,C> AsHandler<Self,C> for StandardCtx<S,C> where S: Handler<C>, C: Context<Handler=Self> {
    fn as_mut(c: &mut C) -> &mut Self {
        c._handler_mut()
    }
    fn as_ref(c: &C) -> &Self {
        c._handler()
    }
}

impl<S,C> AsHandler<S,C> for StandardCtx<S,C> where S: Handler<C>, C: Context<Handler=Self> {
    fn as_mut(c: &mut C) -> &mut S {
        &mut c._handler_mut().sup
    }
    fn as_ref(c: &C) -> &S {
        &c._handler().sup
    }
}

/*impl<S,C,T> AsHandler<S,C> for T where S: Handler<C>, C: Context, C::Handler: AsHandler<StandardCtx<S,C>,C> + 'static {
    fn as_mut(c: &mut C) -> &mut S {
        &mut c._handler_mut().sup
    }
    fn as_ref(c: &C) -> &S {
        &c._handler().sup
    }
}*/