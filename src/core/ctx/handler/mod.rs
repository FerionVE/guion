use super::*;

pub mod access;
pub use access::*;

/// Handlers are stacked inside a Container and any render/event/size action goes through the handler stack
pub trait Handler<E>: Sized + 'static where E: Env {
    fn _render(l: Link<E>, r: (&mut ERenderer<E>,&Bounds));
    fn _event(l: Link<E>, e: (EEvent<E>,&Bounds));
    fn _event_root(l: Link<E>, e: (EEvent<E>,&Bounds));
    fn _size(l: Link<E>) -> ESize<E>;
}

impl<E> Handler<E> for () where E: Env {
    #[inline] 
    fn _render(mut l: Link<E>, r: (&mut ERenderer<E>,&Bounds)) {
        l._render(r)
    }
    #[inline] 
    fn _event(mut l: Link<E>, e: (EEvent<E>,&Bounds)) {
        l._event(e)
    }
    #[inline] 
    fn _event_root(l: Link<E>, e: (EEvent<E>,&Bounds)) {
        l.ctx.event(l.widget,e)
    }
    #[inline] 
    fn _size(mut l: Link<E>) -> ESize<E> {
        l._size()
    }
}