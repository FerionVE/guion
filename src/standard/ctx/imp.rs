use crate::core::ctx::aliases::*;
use crate::core::util::bounds::Bounds;
use crate::core::*;
use ctx::*;
use lazout::size::Size;
use super::*;

impl<S,E> Handler<E> for StandardCtx<S,E> where S: Handler<E>, E: Env, ECHandler<E>: AsHandler<Self,E> + 'static {
    #[inline] 
    fn _render(l: Link<E>, r: (&mut ERenderer<E>,&Bounds)) {
        S::_render(l,r);
        unimplemented!()
    }
    #[inline] 
    fn _event(l: Link<E>, e: (EEvent<E>,&Bounds)) {
        S::_event(l,e);
        unimplemented!()
    }
    #[inline] 
    fn _event_root(l: Link<E>, e: (EEvent<E>,&Bounds)) {
        Self::_event(l,e);
        unimplemented!()
    }
    #[inline] 
    fn _size(l: Link<E>) -> Size {
        unimplemented!();
        S::_size(l)
    }
}

impl<S,E> AsHandler<Self,E> for StandardCtx<S,E> where S: Handler<E>, E: Env, E::Context: Context<E,Handler=Self> {
    fn as_mut(c: &mut E::Context) -> &mut Self {
        c._handler_mut()
    }
    fn as_ref(c: &E::Context) -> &Self {
        c._handler()
    }
}

impl<S,E> AsHandler<S,E> for StandardCtx<S,E> where S: Handler<E>, E: Env, E::Context: Context<E,Handler=Self> {
    fn as_mut(c: &mut E::Context) -> &mut S {
        &mut c._handler_mut().sup
    }
    fn as_ref(c: &E::Context) -> &S {
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